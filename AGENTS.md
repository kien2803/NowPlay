# NowPlay — dev notes

Cross-platform floating mini player. Stack: Tauri 2 (Rust) + Svelte 5 + Vite + TypeScript.

## Architecture
- `src-tauri/src/core/` — backend-agnostic: `model.rs` (PlayerState/TrackMetadata), `player.rs` (`MediaPlayer` + `PlayerSource` traits), `manager.rs` (PlayerManager: discovery, active-player selection, emits `player-state` events every 500ms).
- `src-tauri/src/platforms/linux/mpris.rs` — MPRIS2 over D-Bus via `mpris` crate (covers Spotify, Firefox, VLC, Amberol, ...). No playerctl subprocess.
- `src-tauri/src/platforms/macos/spotify.rs` — Spotify via a single `osascript` snapshot script.
- `src-tauri/src/platforms/windows/smtc.rs` — SMTC sessions via `windows` crate (no volume API → slider hidden).
- `src-tauri/src/window_pos.rs` — persists window position (JSON in app_data_dir, throttled writes).
- `src/` — Svelte UI. `lib/stores.ts` listens to `player-state`; auto-hides on `null`, auto-shows only on the no-player→playing transition.

## Features
- Compact/pill mode: double-click album art; window resizes (see `SIZE_FULL`/`SIZE_COMPACT` in `App.svelte`).
- Volume slider: hover the speaker icon in the bottom row.
- Player priority: `preference_rank` in `manager.rs` (dedicated players > browsers).
- Commands (invoke): `play_pause`, `next_track`, `previous_track`, `seek_to { positionMs }`, `set_volume { volume }`.

## System deps (Fedora)
```
sudo dnf install -y webkit2gtk4.1-devel gtk3-devel libsoup3-devel openssl-devel \
  libappindicator-gtk3-devel librsvg2-devel atk-devel cairo-devel gdk-pixbuf2-devel pango-devel
```

## Commands
- `pnpm install`
- `pnpm tauri dev` — dev run
- `pnpm build` — frontend only
- `pnpm check` — svelte-check

## Notes
- Toggle hotkey: Ctrl+Shift+Space (hardcoded in `src-tauri/src/lib.rs`).
- Tray icon: left-click toggles window; menu has Show/Hide + Quit.
- CSP in `tauri.conf.json` allows `https:`/`http:` images for remote album art.

## Theme
- Dark/light/system via `theme` store (`stores.ts`), persisted in localStorage `nowplay:theme`. Toggle: chip button in bottom row cycles dark → light → system. CSS vars live in `src/styles.css` (`--bg`, `--fg`, `--fg-dim`, `--fill`, `--hover`, `--border`, `--chip-bg`) — never hardcode `rgba(255,255,255,x)` in components.

## Lyrics
- `src/lib/lyrics.ts` fetches lrclib.net (synced LRC preferred, plain fallback); `LyricsPanel.svelte` auto-scrolls to the active line by `positionMs`. Toggle: ♪ chip. Window grows to `SIZE_LYRICS` in `App.svelte`.
