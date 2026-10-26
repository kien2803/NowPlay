import { writable } from "svelte/store";
import { listen } from "@tauri-apps/api/event";
import { getCurrentWindow } from "@tauri-apps/api/window";
import type { PlayerState } from "./types";

export const playerState = writable<PlayerState | null>(null);
export const accent = writable<string>("#7c6cf0");
/** Compact "pill" mode — small art + controls only. */
export const compact = writable<boolean>(false);

let hadPlayer = false;

listen<PlayerState | null>("player-state", async (e) => {
  const win = getCurrentWindow();
  const wasVisible = await win.isVisible();
  playerState.set(e.payload);

  if (e.payload === null) {
    // Nothing is playing anywhere — hide the floating window.
    if (wasVisible) win.hide();
    hadPlayer = false;
  } else {
    // Auto-pop only on the "no player -> playing" transition, so a
    // manually-hidden window is not forced back open every tick.
    if (!hadPlayer && e.payload.status === "playing" && !wasVisible) {
      win.show();
    }
    hadPlayer = true;
  }
});
