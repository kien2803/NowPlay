use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use serde::Serialize;
use tauri::{AppHandle, Emitter};

use super::model::PlaybackStatus;
use super::player::{MediaPlayer, PlayerSource};

/// Poll interval for player state. 500ms is a good balance between
/// progress-bar smoothness and D-Bus traffic.
const TICK: Duration = Duration::from_millis(500);

/// Coordinates all `PlayerSource`s, picks the "active" player,
/// and streams state snapshots to the frontend.
///
/// Player selection policy (for now):
///   1. If a player is Playing, it wins (first playing player found).
///   2. Otherwise stick to the previously-active player if still alive.
///   3. Otherwise pick the first discovered player.
/// A user-configurable priority list can slot in here later.
pub struct PlayerManager {
    sources: Vec<Box<dyn PlayerSource>>,
    active_id: Arc<Mutex<Option<String>>>,
    app: AppHandle,
}

impl PlayerManager {
    pub fn new(sources: Vec<Box<dyn PlayerSource>>, app: AppHandle) -> Arc<Self> {
        Arc::new(Self {
            sources,
            active_id: Arc::new(Mutex::new(None)),
            app,
        })
    }

    /// Spawn the background polling thread.
    pub fn start(self: &Arc<Self>) {
        let mgr = Arc::clone(self);
        thread::spawn(move || mgr.run());
    }

    fn run(&self) {
        loop {
            let players = self.discover();
            let state = self.pick_active(&players).map(|p| p.snapshot());
            self.emit(state);
            thread::sleep(TICK);
        }
    }

    fn discover(&self) -> Vec<Box<dyn MediaPlayer>> {
        self.sources.iter().flat_map(|s| s.discover()).collect()
    }

    /// Preference ranking: lower is better. Players whose id contains
    /// an earlier token in `PREFERRED` win. Dedicated music players
    /// outrank browsers, which often hold an MPRIS instance for video.
    fn preference_rank(id: &str) -> usize {
        const PREFERRED: &[&str] = &[
            "spotify", "amberol", "rhythmbox", "vlc", "mpv", "elisa",
            "firefox", "zen", "chrom",
        ];
        PREFERRED
            .iter()
            .position(|t| id.to_lowercase().contains(t))
            .unwrap_or(PREFERRED.len())
    }

    fn pick_active<'a>(
        &self,
        players: &'a [Box<dyn MediaPlayer>],
    ) -> Option<&'a dyn MediaPlayer> {
        if players.is_empty() {
            return None;
        }

        let mut active_id = self.active_id.lock().unwrap();

        // Prefer the highest-ranked player that is actually playing.
        if let Some(p) = players
            .iter()
            .filter(|p| p.snapshot().status == PlaybackStatus::Playing)
            .min_by_key(|p| Self::preference_rank(&p.id()))
        {
            *active_id = Some(p.id());
            return Some(&**p);
        }

        // Stick with the previous player if it's still around.
        if let Some(id) = active_id.as_deref() {
            if let Some(p) = players.iter().find(|p| p.id() == id) {
                return Some(&**p);
            }
        }

        // Nothing is playing — fall back to the best-ranked player.
        let p = players
            .iter()
            .min_by_key(|p| Self::preference_rank(&p.id()))
            .unwrap();
        *active_id = Some(p.id());
        Some(&**p)
    }

    fn emit<T: Serialize + Clone>(&self, payload: T) {
        // Frontend subscribes to this to render / auto-hide the window.
        let _ = self.app.emit("player-state", payload);
    }

    // --- Commands called from the frontend ---

    /// Run a transport action on whichever player is currently active.
    /// Re-discovers each time so controls keep working even if the
    /// polling tick hasn't refreshed the player list yet.
    pub fn with_active(&self, f: impl FnOnce(&dyn MediaPlayer)) {
        let players = self.discover();
        if let Some(p) = self.pick_active(&players) {
            f(p);
        }
    }
}
