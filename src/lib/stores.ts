import { get, writable } from "svelte/store";
import { listen } from "@tauri-apps/api/event";
import { getCurrentWindow } from "@tauri-apps/api/window";
import type { PlayerState } from "./types";

export const playerState = writable<PlayerState | null>(null);
export const accent = writable<string>("#7c6cf0");
/** Compact "pill" mode — small art + controls only. */
export const compact = writable<boolean>(false);
/** Lyrics panel visible under the card. */
export const showLyrics = writable<boolean>(false);

// --- Theme: "dark" | "light" | "system", persisted in localStorage ---
export type Theme = "dark" | "light" | "system";
export const theme = writable<Theme>(
  (localStorage.getItem("nowplay:theme") as Theme) || "system",
);

function resolvedTheme(t: Theme): "dark" | "light" {
  return t === "system"
    ? matchMedia("(prefers-color-scheme: light)").matches
      ? "light"
      : "dark"
    : t;
}

theme.subscribe((t) => {
  localStorage.setItem("nowplay:theme", t);
  document.documentElement.dataset.theme = resolvedTheme(t);
});

// Re-resolve when the OS theme flips while set to "system".
matchMedia("(prefers-color-scheme: light)").addEventListener("change", () => {
  if (get(theme) === "system")
    document.documentElement.dataset.theme = resolvedTheme("system");
});

// --- User settings, persisted in localStorage ---
export interface Settings {
  /** Show the animated visualizer while playing. */
  visualizer: boolean;
  /** Auto-open the lyrics panel whenever a new track starts. */
  lyricsAutoOpen: boolean;
}
export const settings = writable<Settings>({
  visualizer: true,
  lyricsAutoOpen: false,
  ...JSON.parse(localStorage.getItem("nowplay:settings") ?? "{}"),
});
settings.subscribe((s) =>
  localStorage.setItem("nowplay:settings", JSON.stringify(s)),
);

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
    // Auto-open lyrics on new track if enabled.
    if (get(settings).lyricsAutoOpen && e.payload.status === "playing") {
      showLyrics.set(true);
    }
  }
});
