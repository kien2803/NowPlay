<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import type { PlayerState } from "./types";

  let { state }: { state: PlayerState } = $props();
  const playing = $derived(state.status === "playing");
</script>

<div class="controls">
  <button class="btn" onclick={() => invoke("previous_track")} aria-label="Previous" disabled={!state.canControl}>
    <svg viewBox="0 0 24 24"><path d="M6 5h2v14H6zM20 5v14L9.5 12z" fill="currentColor"/></svg>
  </button>

  <button class="btn play" onclick={() => invoke("play_pause")} aria-label="Play/Pause" disabled={!state.canControl}>
    {#if playing}
      <svg viewBox="0 0 24 24"><path d="M7 5h4v14H7zM13 5h4v14h-4z" fill="currentColor"/></svg>
    {:else}
      <svg viewBox="0 0 24 24"><path d="M8 5v14l11-7z" fill="currentColor"/></svg>
    {/if}
  </button>

  <button class="btn" onclick={() => invoke("next_track")} aria-label="Next" disabled={!state.canControl}>
    <svg viewBox="0 0 24 24"><path d="M16 5h2v14h-2zM4 5v14l10.5-7z" fill="currentColor"/></svg>
  </button>
</div>

<style>
  .controls { display: flex; align-items: center; gap: 6px; }
  .btn {
    width: 32px; height: 32px;
    display: grid; place-items: center;
    border: none; border-radius: 50%;
    background: transparent;
    color: var(--fg);
    cursor: pointer;
    transition: background 0.15s, transform 0.1s;
  }
  .btn svg { width: 18px; height: 18px; }
  .btn:hover { background: var(--hover); }
  .btn:active { transform: scale(0.92); }
  .btn:disabled { opacity: 0.35; cursor: default; }
  .btn.play {
    width: 38px; height: 38px;
    background: var(--accent);
    color: #fff;
  }
  .btn.play svg { width: 20px; height: 20px; }
  .btn.play:hover { filter: brightness(1.1); }
</style>
