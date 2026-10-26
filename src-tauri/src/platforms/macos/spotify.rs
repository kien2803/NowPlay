//! macOS backend: drives Spotify via AppleScript (`osascript`).
//!
//! Spotify's AppleScript dictionary exposes `player state`,
//! `player position` (seconds), `sound volume` (0-100), and
//! `current track` metadata — everything `MediaPlayer` needs.
//! Later sources (Music.app, MediaRemote) slot in as new
//! `PlayerSource`s without touching the core.

use std::process::Command;

use crate::core::model::{PlaybackStatus, PlayerState, TrackMetadata};
use crate::core::player::{MediaPlayer, PlayerSource};

/// `PlayerSource` that yields a `SpotifyPlayer` while Spotify is running.
pub struct SpotifySource;

impl PlayerSource for SpotifySource {
    fn discover(&self) -> Vec<Box<dyn MediaPlayer>> {
        if spotify_running() {
            vec![Box::new(SpotifyPlayer)]
        } else {
            Vec::new()
        }
    }
}

fn osascript(script: &str) -> Option<String> {
    let out = Command::new("osascript").arg("-e").arg(script).output().ok()?;
    if out.status.success() {
        Some(String::from_utf8_lossy(&out.stdout).trim().to_string())
    } else {
        None
    }
}

fn spotify_running() -> bool {
    osascript(
        r#"tell application "System Events" to (name of processes) contains "Spotify""#,
    )
    .map(|s| s == "true")
    .unwrap_or(false)
}

/// One AppleScript invocation returning newline-separated fields, so
/// each snapshot costs a single osascript spawn instead of ~7.
const SNAPSHOT_SCRIPT: &str = r#"tell application "Spotify"
    set t to current track
    set out to (player state as string) & "\n" & ¬
        (name of t) & "\n" & ¬
        (artist of t) & "\n" & ¬
        (album of t) & "\n" & ¬
        (artwork url of t) & "\n" & ¬
        ((duration of t) as string) & "\n" & ¬
        ((player position) as string) & "\n" & ¬
        ((sound volume) as string)
    return out
end tell"#;

pub struct SpotifyPlayer;

impl MediaPlayer for SpotifyPlayer {
    fn id(&self) -> String {
        "com.spotify.client".to_string()
    }

    fn display_name(&self) -> String {
        "Spotify".to_string()
    }

    fn snapshot(&self) -> PlayerState {
        let out = osascript(SNAPSHOT_SCRIPT).unwrap_or_default();
        let mut l = out.lines();
        let next = |l: &mut std::str::Lines| l.next().map(str::trim).filter(|s| !s.is_empty() && *s != "missing value");

        let status = match next(&mut l) {
            Some("playing") => PlaybackStatus::Playing,
            Some("paused") => PlaybackStatus::Paused,
            _ => PlaybackStatus::Stopped,
        };

        let metadata = TrackMetadata {
            title: next(&mut l).map(String::from),
            artist: next(&mut l).map(String::from),
            album: next(&mut l).map(String::from),
            art_url: next(&mut l).map(String::from),
            // AppleScript reports duration in milliseconds.
            length_ms: next(&mut l).and_then(|s| s.parse::<f64>().ok()).map(|v| v as u64),
        };

        PlayerState {
            player_id: self.id(),
            player_name: self.display_name(),
            status,
            metadata,
            // Player position is reported in seconds (float).
            position_ms: next(&mut l)
                .and_then(|s| s.parse::<f64>().ok())
                .map(|s| (s * 1000.0) as u64)
                .unwrap_or(0),
            can_control: true,
            can_seek: true,
            // sound volume is 0-100.
            volume: next(&mut l)
                .and_then(|s| s.parse::<f64>().ok())
                .map(|v| (v / 100.0).clamp(0.0, 1.0)),
        }
    }

    fn play_pause(&self) {
        osascript(r#"tell application "Spotify" to playpause"#);
    }
    fn next(&self) {
        osascript(r#"tell application "Spotify" to next track"#);
    }
    fn previous(&self) {
        osascript(r#"tell application "Spotify" to previous track"#);
    }

    fn seek_to(&self, position_ms: u64) {
        osascript(&format!(
            r#"tell application "Spotify" to set player position to {}"#,
            position_ms as f64 / 1000.0
        ));
    }

    fn set_volume(&self, volume: f64) {
        osascript(&format!(
            r#"tell application "Spotify" to set sound volume to {}"#,
            (volume.clamp(0.0, 1.0) * 100.0) as i32
        ));
    }
}
