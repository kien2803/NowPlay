<script lang="ts">
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { LogicalSize } from "@tauri-apps/api/dpi";
  import { playerState, accent, compact } from "./lib/stores";
  import PlayerCard from "./lib/PlayerCard.svelte";

  // Window sizes for each mode (must match the card layouts).
  const SIZE_FULL = new LogicalSize(400, 132);
  const SIZE_COMPACT = new LogicalSize(232, 72);

  $effect(() => {
    getCurrentWindow().setSize($compact ? SIZE_COMPACT : SIZE_FULL);
  });
</script>

<main style="--accent: {$accent}">
  {#if $playerState}
    {#key $playerState.metadata.title}
      <PlayerCard state={$playerState} />
    {/key}
  {:else}
    <div class="idle">
      <span>No player detected</span>
    </div>
  {/if}
</main>

<style>
  main {
    width: 100%; height: 100%;
    padding: 8px; /* leave room for the window shadow */
  }
  .idle {
    width: 100%; height: 100%;
    display: grid; place-items: center;
    background: var(--bg);
    border-radius: var(--radius);
    border: 1px solid rgba(255,255,255,0.08);
    color: var(--fg-dim);
    font-size: 12px;
  }
</style>
