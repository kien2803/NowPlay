mod commands;
mod core;
mod platforms;
mod window_pos;

use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    Manager, PhysicalPosition, WebviewUrl, WebviewWindowBuilder,
};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut};

use core::manager::PlayerManager;
use core::player::PlayerSource;

/// Hotkey to toggle the floating player: Ctrl+Shift+Space.
/// (Configurable later via a settings store.)
fn toggle_shortcut() -> Shortcut {
    Shortcut::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::Space)
}

fn build_sources() -> Vec<Box<dyn PlayerSource>> {
    let mut sources: Vec<Box<dyn PlayerSource>> = Vec::new();

    #[cfg(target_os = "linux")]
    sources.push(Box::new(platforms::linux::mpris::MprisSource));

    #[cfg(target_os = "macos")]
    sources.push(Box::new(platforms::macos::spotify::SpotifySource));

    sources
}

fn toggle_window(app: &tauri::AppHandle) {
    if let Some(w) = app.get_webview_window("player") {
        match w.is_visible() {
            Ok(true) => {
                let _ = w.hide();
            }
            _ => {
                let _ = w.show();
                let _ = w.set_focus();
            }
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            commands::play_pause,
            commands::next_track,
            commands::previous_track,
            commands::seek_to,
            commands::set_volume,
        ])
        .setup(|app| {
            let app_handle = app.handle().clone();

            // --- Floating window ---
            // Transparent, frameless, always-on-top.
            let win = WebviewWindowBuilder::new(
                &app_handle,
                "player",
                WebviewUrl::App("index.html".into()),
            )
            .title("NowPlay")
            .inner_size(400.0, 132.0)
            .resizable(false)
            .decorations(false)
            .transparent(true)
            .always_on_top(true)
            .skip_taskbar(true)
            .shadow(true)
            .build()?;

            // Restore the last dragged position (or center on first run).
            match window_pos::load(&app_handle) {
                Some((x, y)) => {
                    let _ = win.set_position(PhysicalPosition::new(x, y));
                }
                None => {
                    let _ = win.center();
                }
            }

            // Persist position as the window is dragged / on exit.
            // Moved fires every frame while dragging, so track the latest
            // position and flush (throttled); write once more on destroy.
            {
                let h = app_handle.clone();
                let last_pos = std::sync::Mutex::new(None::<(i32, i32)>);
                win.on_window_event(move |event| match event {
                    tauri::WindowEvent::Moved(pos) => {
                        *last_pos.lock().unwrap() = Some((pos.x, pos.y));
                        window_pos::save_throttled(&h, pos.x, pos.y);
                    }
                    tauri::WindowEvent::Destroyed => {
                        if let Some((x, y)) = *last_pos.lock().unwrap() {
                            window_pos::save_final(&h, x, y);
                        }
                    }
                    _ => {}
                });
            }

            // --- System tray ---
            let show = MenuItem::with_id(app, "show", "Show / Hide", true, None::<&str>)?;
            let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show, &quit])?;
            TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .tooltip("NowPlay")
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "show" => toggle_window(app),
                    "quit" => app.exit(0),
                    _ => {}
                })
                .on_tray_icon_event(|tray, _| {
                    // Left-click on the tray icon toggles the window too.
                    toggle_window(tray.app_handle());
                })
                .build(app)?;

            // --- Global hotkey ---
            app.global_shortcut().on_shortcut(toggle_shortcut(), |app, _, _| {
                toggle_window(app);
            })?;

            // --- Player engine ---
            let mgr = PlayerManager::new(build_sources(), app_handle.clone());
            mgr.start();
            app_handle.manage(mgr);

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running NowPlay");
}
