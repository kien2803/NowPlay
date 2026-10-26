use std::sync::Arc;
use tauri::State;

use crate::core::manager::PlayerManager;

/// Transport commands — all operate on the currently active player.

#[tauri::command]
pub fn play_pause(mgr: State<'_, Arc<PlayerManager>>) {
    mgr.with_active(|p| p.play_pause());
}

#[tauri::command]
pub fn next_track(mgr: State<'_, Arc<PlayerManager>>) {
    mgr.with_active(|p| p.next());
}

#[tauri::command]
pub fn previous_track(mgr: State<'_, Arc<PlayerManager>>) {
    mgr.with_active(|p| p.previous());
}

#[tauri::command]
pub fn seek_to(mgr: State<'_, Arc<PlayerManager>>, position_ms: u64) {
    mgr.with_active(|p| p.seek_to(position_ms));
}

#[tauri::command]
pub fn set_volume(mgr: State<'_, Arc<PlayerManager>>, volume: f64) {
    mgr.with_active(|p| p.set_volume(volume));
}
