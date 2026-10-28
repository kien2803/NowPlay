use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use serde::Serialize;
use tauri::{AppHandle, Emitter};

use super::model::PlaybackStatus;
use super::player::{MediaPlayer, PlayerSource};

/// Idle-ish waits: keep progress-bar smoothness while playing,
/// go nearly dormant otherwise. `wait_event` wakes early on real
/// player events (MPRIS), so these are ceilings, not polling periods.
const PLAYING_WAIT_MS: u64 = 600;
const PAUSED_WAIT_MS: u64 = 4_000;
const IDLE_WAIT_MS: u64 = 5_000;

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
            // Snapshot every player exactly once per tick — each snapshot
            // is several D-Bus calls, so re-snapshotting during selection
            // (filter + chosen player) would roughly double our traffic.
            let snaps = self.snapshots();
            let active = self.pick_active(&snaps);
            self.emit(active.map(|(_, snap)| snap.clone()));

            // Block until a player event fires or the timeout hits.
            // While paused/idle this makes us nearly dormant.
            match active {
                Some((p, s)) => p.wait_event(match s.status {
                    PlaybackStatus::Playing => PLAYING_WAIT_MS,
                    _ => PAUSED_WAIT_MS,
                }),
                None => thread::sleep(Duration::from_millis(IDLE_WAIT_MS)),
            }
        }
    }

    fn discover(&self) -> Vec<Box<dyn MediaPlayer>> {
        self.sources.iter().flat_map(|s| s.discover()).collect()
    }

    fn snapshots(&self) -> Vec<(Box<dyn MediaPlayer>, super::model::PlayerState)> {
        self.discover()
            .into_iter()
            .map(|p| {
                let s = p.snapshot();
                (p, s)
            })
            .collect()
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
        snaps: &'a [(Box<dyn MediaPlayer>, super::model::PlayerState)],
    ) -> Option<&'a (Box<dyn MediaPlayer>, super::model::PlayerState)> {
        if snaps.is_empty() {
            return None;
        }

        let mut active_id = self.active_id.lock().unwrap();

        // Prefer the highest-ranked player that is actually playing.
        if let Some(pair) = snaps
            .iter()
            .filter(|(_, s)| s.status == PlaybackStatus::Playing)
            .min_by_key(|(p, _)| Self::preference_rank(&p.id()))
        {
            *active_id = Some(pair.0.id());
            return Some(pair);
        }

        // Stick with the previous player if it's still around.
        if let Some(id) = active_id.as_deref() {
            if let Some(pair) = snaps.iter().find(|(p, _)| p.id() == id) {
                return Some(pair);
            }
        }

        // Nothing is playing — fall back to the best-ranked player.
        let pair = snaps
            .iter()
            .min_by_key(|(p, _)| Self::preference_rank(&p.id()))
            .unwrap();
        *active_id = Some(pair.0.id());
        Some(pair)
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
        let snaps = self.snapshots();
        if let Some((p, _)) = self.pick_active(&snaps) {
            f(&**p);
        }
    }
}
