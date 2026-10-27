<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import type { PlayerState } from "./types";

  // NB: don't destructure the prop as `state` — `$state` would then be
  // parsed as a store subscription instead of the rune.
  let { state: player }: { state: PlayerState } = $props();

  // Local mirror so the slider moves instantly; the next snapshot
  // (500ms tick) confirms the real value.
  let local = $state<number | null>(null);
  const vol = $derived(local ?? player.volume ?? 1);

  function onInput(e: Event) {
    const v = parseFloat((e.target as HTMLInputElement).value);
    local = v;
    invoke("set_volume", { volume: v });
  }
</script>

<div class="vol" title="Volume">
  <svg viewBox="0 0 24 24" class="icon">
    {#if vol === 0}
      <path d="M3 9v6h4l5 5V4L7 9H3zm13.6 3l2.7-2.7-1.4-1.4-2.7 2.7-2.7-2.7-1.4 1.4 2.7 2.7-2.7 2.7 1.4 1.4 2.7-2.7 2.7 2.7 1.4-1.4-2.7-2.7z" fill="currentColor"/>
    {:else}
      <path d="M3 9v6h4l5 5V4L7 9H3zm13.5 3A4.5 4.5 0 0 0 14 8v8a4.5 4.5 0 0 0 2.5-4zM14 3.2v2.1c2.9.9 5 3.6 5 6.7s-2.1 5.8-5 6.7v2.1c4-1 7-4.6 7-8.8s-3-7.8-7-8.8z" fill="currentColor"/>
    {/if}
  </svg>
  <input
    type="range"
    min="0" max="1" step="0.01"
    value={vol}
    oninput={onInput}
    style="--vol: {vol * 100}%"
  />
</div>

<style>
  .vol {
    display: flex; align-items: center; gap: 6px;
    color: var(--fg-dim);
  }
  .icon { width: 14px; height: 14px; flex-shrink: 0; }
  input[type="range"] {
    -webkit-appearance: none;
    appearance: none;
    width: 0;
    opacity: 0;
    height: 3px;
    border-radius: 2px;
    background: linear-gradient(to right, var(--accent) var(--vol), var(--fill) var(--vol));
    outline: none;
    cursor: pointer;
    transition: width 0.2s ease, opacity 0.2s ease;
  }
  .vol:hover input,
  input:focus-visible {
    width: 56px;
    opacity: 1;
  }
  input::-webkit-slider-thumb {
    -webkit-appearance: none;
    width: 10px; height: 10px;
    border-radius: 50%;
    background: var(--fg);
  }
</style>
