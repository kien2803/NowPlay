<script lang="ts">
  import type { PlayerState } from "./types";
  import { fetchLyrics, type Lyrics } from "./lyrics";

  // NB: prop renamed locally — `$state` would clash with a `state` var.
  let { state: player }: { state: PlayerState } = $props();

  let lyrics = $state<Lyrics | null>(null);
  let loading = $state(false);
  let trackKey = $state("");

  // Fetch whenever the track changes.
  $effect(() => {
    const md = player.metadata;
    const key = `${md.title}|${md.artist}`;
    if (key === trackKey || !md.title) return;
    trackKey = key;
    lyrics = null;
    loading = true;
    fetchLyrics(md.title, md.artist ?? "", md.album ?? "", md.lengthMs ?? 0)
      .then((l) => (lyrics = l))
      .finally(() => (loading = false));
  });

  // Index of the currently-sung synced line.
  const activeIdx = $derived.by(() => {
    if (!lyrics?.synced) return -1;
    let i = -1;
    for (const [n, l] of lyrics.synced.entries())
      if (l.t <= player.positionMs) i = n;
      else break;
    return i;
  });

  let listEl = $state<HTMLDivElement>();

  // Keep the active line centered.
  $effect(() => {
    if (activeIdx < 0 || !listEl) return;
    const el = listEl.children[activeIdx] as HTMLElement | undefined;
    el?.scrollIntoView({ block: "center", behavior: "smooth" });
  });
</script>

<div class="lyrics" data-tauri-drag-region>
  {#if loading}
    <span class="dim">Loading lyrics…</span>
  {:else if lyrics?.synced}
    <div class="scroll" bind:this={listEl}>
      {#each lyrics.synced as line, i}
        <p class:active={i === activeIdx} class:done={i < activeIdx}>{line.text}</p>
      {/each}
    </div>
  {:else if lyrics?.plain}
    <div class="scroll plain">{lyrics.plain}</div>
  {:else}
    <span class="dim">No lyrics found</span>
  {/if}
</div>

<style>
  .lyrics {
    height: 180px;
    padding: 4px 14px 10px;
    overflow: hidden;
  }
  .scroll {
    height: 100%;
    overflow-y: auto;
    scrollbar-width: none;
    mask-image: linear-gradient(to bottom, transparent, black 18%, black 82%, transparent);
  }
  .scroll::-webkit-scrollbar { display: none; }
  p {
    font-size: 12px;
    color: var(--fg-dim);
    text-align: center;
    padding: 3px 0;
    transition: color 0.25s, font-weight 0.25s;
  }
  p.active {
    color: var(--fg);
    font-weight: 600;
  }
  p.done { opacity: 0.45; }
  .plain {
    font-size: 12px;
    color: var(--fg-dim);
    white-space: pre-wrap;
    padding: 8px 0;
  }
  .dim {
    display: block;
    text-align: center;
    color: var(--fg-dim);
    font-size: 11px;
    padding-top: 70px;
  }
</style>
