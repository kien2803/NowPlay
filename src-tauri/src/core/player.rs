use super::model::PlayerState;

/// A single media player source (one running app instance).
///
/// Every platform backend (MPRIS on Linux, AppleScript on macOS, ...)
/// adapts its native API to this trait so the rest of the app is
/// backend-agnostic.
///
/// NOTE: intentionally NOT `Send` — e.g. `mpris::Player` uses `Rc`
/// internally. Player objects are created, used, and dropped within a
/// single polling/command call and never cross threads.
pub trait MediaPlayer {
    /// Stable identifier used to match a player across polling ticks
    /// (e.g. the D-Bus bus name suffix, or an app bundle id).
    fn id(&self) -> String;

    /// Human-readable name for UI display / priority selection.
    fn display_name(&self) -> String;

    /// Take a snapshot of the current playback state for the UI.
    /// Should be cheap; called roughly once per polling tick.
    fn snapshot(&self) -> PlayerState;

    // --- Transport controls ---
    fn play_pause(&self);
    fn next(&self);
    fn previous(&self);

    /// Seek to an absolute position within the current track.
    fn seek_to(&self, position_ms: u64);

    /// Set player volume (0.0 - 1.0). Optional on some backends.
    fn set_volume(&self, _volume: f64) {}

    /// Block until the player's state changes or `timeout_ms` elapses.
    /// Backends without push notifications just sleep — the default.
    fn wait_event(&self, timeout_ms: u64) {
        std::thread::sleep(std::time::Duration::from_millis(timeout_ms));
    }
}

/// A source that can discover `MediaPlayer`s on the current platform.
/// One implementation per platform; adding support for a new player
/// means adding another `PlayerSource` (or extending MPRIS coverage).
pub trait PlayerSource: Send + Sync {
    /// List all currently reachable players, no specific order.
    fn discover(&self) -> Vec<Box<dyn MediaPlayer>>;
}
