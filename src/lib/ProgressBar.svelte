<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import type { PlayerState } from "./types";

  let { state }: { state: PlayerState } = $props();

  const length = $derived(state.metadata.lengthMs ?? 0);
  const pct = $derived(length > 0 ? Math.min(100, (state.positionMs / length) * 100) : 0);

  function fmt(ms: number): string {
    const s = Math.floor(ms / 1000);
    return `${Math.floor(s / 60)}:${String(s % 60).padStart(2, "0")}`;
  }

  function onSeek(e: MouseEvent) {
    if (!state.canSeek || length <= 0) return;
    const bar = e.currentTarget as HTMLElement;
    const rect = bar.getBoundingClientRect();
    const ratio = Math.min(1, Math.max(0, (e.clientX - rect.left) / rect.width));
    invoke("seek_to", { positionMs: Math.round(ratio * length) });
  }

  function onKey(e: KeyboardEvent) {
    if (!state.canSeek || length <= 0) return;
    const step = 5000; // 5s per arrow press
    if (e.key === "ArrowRight")
      invoke("seek_to", { positionMs: Math.min(length, state.positionMs + step) });
    else if (e.key === "ArrowLeft")
      invoke("seek_to", { positionMs: Math.max(0, state.positionMs - step) });
  }
</script>

<div class="progress">
  <span class="time">{fmt(state.positionMs)}</span>
  <div
    class="bar"
    class:seekable={state.canSeek}
    role="slider"
    tabindex={state.canSeek ? 0 : -1}
    aria-label="Seek"
    aria-valuemin={0}
    aria-valuemax={length}
    aria-valuenow={state.positionMs}
    onclick={onSeek}
    onkeydown={onKey}
  >
    <div class="fill" style="width: {pct}%"></div>
  </div>
  <span class="time">{fmt(length)}</span>
</div>

<style>
  .progress { display: flex; align-items: center; gap: 8px; width: 100%; }
  .time {
    font-size: 10px;
    color: var(--fg-dim);
    font-variant-numeric: tabular-nums;
    min-width: 28px;
  }
  .time:last-child { text-align: right; }
  .bar {
    flex: 1; height: 4px;
    background: var(--fill);
    border-radius: 2px;
    overflow: hidden;
  }
  .bar.seekable { cursor: pointer; }
  .bar.seekable:hover { height: 6px; }
  .fill {
    height: 100%;
    background: var(--accent);
    border-radius: 2px;
    transition: width 0.4s linear;
  }
</style>
