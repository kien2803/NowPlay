use serde::{Deserialize, Serialize};

/// Playback state of a media player, normalized across platforms.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PlaybackStatus {
    Playing,
    Paused,
    Stopped,
}

/// Static metadata of the currently loaded track.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackMetadata {
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    /// URL or file path to the album art (may be a remote http URL).
    pub art_url: Option<String>,
    /// Track length in milliseconds, if reported by the player.
    pub length_ms: Option<u64>,
}

/// A full snapshot of what the frontend needs to render the mini player.
/// Emitted to the UI via the `player-state` Tauri event.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerState {
    /// Stable identifier for the source player (e.g. MPRIS bus name "spotify").
    pub player_id: String,
    /// Human-readable name (e.g. "Spotify", "Firefox").
    pub player_name: String,
    pub status: PlaybackStatus,
    pub metadata: TrackMetadata,
    /// Current position in milliseconds.
    pub position_ms: u64,
    pub can_control: bool,
    pub can_seek: bool,
    /// 0.0 - 1.0
    pub volume: Option<f64>,
}
