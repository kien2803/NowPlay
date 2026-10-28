//! Windows backend: SMTC (System Media Transport Controls).
//!
//! `GlobalSystemMediaTransportControlsSessionManager` exposes every
//! session a media app registers — Spotify, Edge/Chrome media,
//! VLC, Media Player, foobar2000, … — the same source the Windows
//! media overlay reads. No per-player integration needed.
//!
//! Volume: SMTC has no volume API, so `volume` is None and
//! `set_volume` is a no-op (the UI hides the slider).

use std::io::Write;

use windows::Media::Control::{
    GlobalSystemMediaTransportControlsSessionManager as SmtcManager,
    GlobalSystemMediaTransportControlsSessionPlaybackStatus as SmtcStatus,
};
use windows::Storage::Streams::DataReader;

use crate::core::model::{PlaybackStatus, PlayerState, TrackMetadata};
use crate::core::player::{MediaPlayer, PlayerSource};

/// `PlayerSource` wrapping the SMTC session manager.
pub struct SmtcSource;

impl PlayerSource for SmtcSource {
    fn discover(&self) -> Vec<Box<dyn MediaPlayer>> {
        let Ok(op) = SmtcManager::RequestAsync() else {
            return Vec::new();
        };
        let Ok(manager) = op.get() else {
            return Vec::new();
        };
        let Ok(sessions) = manager.GetSessions() else {
            return Vec::new();
        };
        sessions
            .into_iter()
            .map(|s| Box::new(SmtcPlayer(s)) as Box<dyn MediaPlayer>)
            .collect()
    }
}

/// One SMTC session = one app currently producing media.
struct SmtcPlayer(windows::Media::Control::GlobalSystemMediaTransportControlsSession);

impl SmtcPlayer {
    /// Dump the thumbnail stream to a temp file so the webview can load
    /// it via the asset protocol (SMTC gives us a stream, not a URL).
    fn art_path(&self) -> Option<String> {
        let props = self.0.TryGetMediaPropertiesAsync().ok()?.get().ok()?;
        let thumb = props.Thumbnail().ok()?;
        let stream = thumb.OpenReadAsync().ok()?.get().ok()?;
        let size = stream.Size().ok()? as u32;
        if size == 0 || size > 8 * 1024 * 1024 {
            return None;
        }
        let reader = DataReader::CreateDataReader(&stream).ok()?;
        reader.LoadAsync(size).ok()?.get().ok()?;
        let mut buf = vec![0u8; size as usize];
        reader.ReadBytes(&mut buf).ok()?;
        let path = std::env::temp_dir().join("nowplay-art.png");
        std::fs::File::create(&path)
            .and_then(|mut f| f.write_all(&buf))
            .ok()?;
        Some(path.to_string_lossy().into_owned())
    }

    fn status(&self) -> PlaybackStatus {
        let info = self.0.GetPlaybackInfo();
        match info.map(|i| i.PlaybackStatus()) {
            Ok(Ok(SmtcStatus::Playing)) => PlaybackStatus::Playing,
            Ok(Ok(SmtcStatus::Paused)) => PlaybackStatus::Paused,
            _ => PlaybackStatus::Stopped,
        }
    }

    fn position_ms(&self) -> u64 {
        self.0
            .GetTimelineProperties()
            .ok()
            .and_then(|t| t.Position().ok())
            .map(|ts| (ts.Duration / 10_000) as u64) // TimeSpan is 100ns
            .unwrap_or(0)
    }
}

impl MediaPlayer for SmtcPlayer {
    fn id(&self) -> String {
        self.0
            .SourceAppUserModelId()
            .map(|s| s.to_string_lossy())
            .unwrap_or_else(|_| "smtc".into())
    }

    fn display_name(&self) -> String {
        self.id()
    }

    fn snapshot(&self) -> PlayerState {
        let props = self.0.TryGetMediaPropertiesAsync().ok().and_then(|o| o.get().ok());
        let controls = self.0.GetPlaybackInfo().ok().and_then(|i| i.Controls().ok());

        let metadata = TrackMetadata {
            title: props.as_ref().and_then(|p| p.Title().ok()).map(|s| s.to_string_lossy()),
            artist: props.as_ref().and_then(|p| p.Artist().ok()).map(|s| s.to_string_lossy()),
            album: props.as_ref().and_then(|p| p.AlbumTitle().ok()).map(|s| s.to_string_lossy()),
            art_url: self.art_path(),
            length_ms: self
                .0
                .GetTimelineProperties()
                .ok()
                .and_then(|t| t.EndTime().ok())
                .map(|ts| (ts.Duration / 10_000) as u64),
        };

        PlayerState {
            player_id: self.id(),
            player_name: self.display_name(),
            status: self.status(),
            metadata,
            position_ms: self.position_ms(),
            can_control: controls
                .as_ref()
                .and_then(|c| c.IsPlayEnabled().ok())
                .unwrap_or(true),
            can_seek: controls
                .as_ref()
                .and_then(|c| c.IsPlaybackPositionEnabled().ok())
                .unwrap_or(true),
            volume: None, // SMTC exposes no volume API
        }
    }

    fn play_pause(&self) {
        let playing = self.status() == PlaybackStatus::Playing;
        let _ = if playing {
            self.0.TryPauseAsync().and_then(|o| o.get())
        } else {
            self.0.TryPlayAsync().and_then(|o| o.get())
        };
    }
    fn next(&self) {
        let _ = self.0.TrySkipNextAsync().and_then(|o| o.get());
    }
    fn previous(&self) {
        let _ = self.0.TrySkipPreviousAsync().and_then(|o| o.get());
    }
    fn seek_to(&self, position_ms: u64) {
        let ts = windows::Foundation::TimeSpan {
            Duration: (position_ms as i64) * 10_000,
        };
        let _ = self
            .0
            .TryChangePlaybackPositionAsync(ts)
            .and_then(|o| o.get());
    }
}
