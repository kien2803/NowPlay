//! Persist the floating window's position across restarts.
//! Saved as JSON in the app data dir; writes are throttled during
//! drag (Moved fires every frame) and flushed once more on close.

use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicI64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};

const MIN_SAVE_INTERVAL_MS: i64 = 500;
static LAST_SAVE_MS: AtomicI64 = AtomicI64::new(0);

#[derive(Serialize, Deserialize)]
struct WindowPos {
    x: i32,
    y: i32,
}

fn file_path(app: &AppHandle) -> Option<PathBuf> {
    app.path()
        .app_data_dir()
        .ok()
        .map(|d| d.join("window-pos.json"))
}

pub fn load(app: &AppHandle) -> Option<(i32, i32)> {
    let s = fs::read_to_string(file_path(app)?).ok()?;
    let pos: WindowPos = serde_json::from_str(&s).ok()?;
    Some((pos.x, pos.y))
}

fn write_now(app: &AppHandle, x: i32, y: i32) {
    if let Some(p) = file_path(app) {
        if let Some(dir) = p.parent() {
            let _ = fs::create_dir_all(dir);
        }
        if let Ok(json) = serde_json::to_string(&WindowPos { x, y }) {
            let _ = fs::write(p, json);
        }
    }
}

/// Save on move, throttled to avoid a disk write per frame while dragging.
pub fn save_throttled(app: &AppHandle, x: i32, y: i32) {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0);
    let last = LAST_SAVE_MS.load(Ordering::Relaxed);
    if now - last >= MIN_SAVE_INTERVAL_MS {
        LAST_SAVE_MS.store(now, Ordering::Relaxed);
        write_now(app, x, y);
    }
}

/// Unconditional save — call when the window is destroyed so the
/// final position is never lost to throttling.
pub fn save_final(app: &AppHandle, x: i32, y: i32) {
    write_now(app, x, y);
}
