//! Linux backend: talks to every MPRIS2-compatible player over D-Bus.
//! Covers Spotify, Firefox, Chrome/Chromium, VLC, Amberol, Rhythmbox,
//! mpv (with mpris script), KDE's plasma-browser-integration, etc.

use std::time::Duration;

use mpris::{Metadata, Player, PlaybackStatus as MprisStatus};

use crate::core::model::{PlaybackStatus, PlayerState, TrackMetadata};
use crate::core::player::{MediaPlayer, PlayerSource};

/// `PlayerSource` that wraps `mpris::PlayerFinder`.
pub struct MprisSource;

impl PlayerSource for MprisSource {
    fn discover(&self) -> Vec<Box<dyn MediaPlayer>> {
        let finder = match mpris::PlayerFinder::new() {
            Ok(f) => f,
            Err(_) => return Vec::new(), // no session bus — nothing to do
        };
        finder
            .find_all()
            .unwrap_or_default()
            .into_iter()
            .map(|p| Box::new(MprisPlayer(p)) as Box<dyn MediaPlayer>)
            .collect()
    }
}

/// Thin adapter from `mpris::Player` to our `MediaPlayer` trait.
struct MprisPlayer(Player);

impl MprisPlayer {
    fn metadata(&self) -> Option<Metadata> {
        self.0.get_metadata().ok()
    }

    fn status(&self) -> PlaybackStatus {
        match self.0.get_playback_status().unwrap_or(MprisStatus::Stopped) {
            MprisStatus::Playing => PlaybackStatus::Playing,
            MprisStatus::Paused => PlaybackStatus::Paused,
            MprisStatus::Stopped => PlaybackStatus::Stopped,
        }
    }
}

impl MediaPlayer for MprisPlayer {
    fn id(&self) -> String {
        // e.g. "org.mpris.MediaPlayer2.spotify" -> "spotify"
        // Stable while the app runs; used for active-player tracking.
        self.0.bus_name().to_string()
    }

    fn display_name(&self) -> String {
        self.0.identity().to_string()
    }

    fn snapshot(&self) -> PlayerState {
        let md = self.metadata();
        let track = TrackMetadata {
            title: md.as_ref().and_then(|m| m.title().map(String::from)),
            artist: md
                .as_ref()
                .and_then(|m| m.artists())
                .map(|a| a.join(", ")),
            album: md
                .as_ref()
                .and_then(|m| m.album_name().map(String::from)),
            art_url: md
                .as_ref()
                .and_then(|m| m.art_url().map(String::from)),
            length_ms: md
                .as_ref()
                .and_then(|m| m.length())
                .map(|d| d.as_millis() as u64),
        };

        PlayerState {
            player_id: self.id(),
            player_name: self.display_name(),
            status: self.status(),
            metadata: track,
            position_ms: self
                .0
                .get_position()
                .map(|d| d.as_millis() as u64)
                .unwrap_or(0),
            can_control: self.0.can_control().unwrap_or(false),
            can_seek: self.0.can_seek().unwrap_or(false),
            volume: self.0.get_volume().ok(),
        }
    }

    fn play_pause(&self) {
        let _ = self.0.play_pause();
    }
    fn next(&self) {
        let _ = self.0.next();
    }
    fn previous(&self) {
        let _ = self.0.previous();
    }

    fn seek_to(&self, position_ms: u64) {
        // MPRIS SetPosition requires the track id, not just a time.
        if let Some(md) = self.metadata() {
            if let Some(track_id) = md.track_id() {
                let _ = self
                    .0
                    .set_position(track_id, &Duration::from_millis(position_ms));
            }
        }
    }

    fn set_volume(&self, volume: f64) {
        let _ = self.0.set_volume(volume.clamp(0.0, 1.0));
    }
}
