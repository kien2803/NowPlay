<script lang="ts">
  import type { PlayerState } from "./types";
  import Controls from "./Controls.svelte";
  import ProgressBar from "./ProgressBar.svelte";
  import VolumeControl from "./VolumeControl.svelte";
  import LyricsPanel from "./LyricsPanel.svelte";
  import Visualizer from "./Visualizer.svelte";
  import SettingsPopover from "./SettingsPopover.svelte";
  import { convertFileSrc } from "@tauri-apps/api/core";
  import { accent, compact, showLyrics, theme, settings, type Theme } from "./stores";
  import { dominantColor } from "./accent";

  // NB: prop renamed locally — `$state` would clash with a `state` var.
  let { state: player }: { state: PlayerState } = $props();

  // MPRIS art can be remote (https://i.scdn.co/...) or a local file://
  // path (browsers cache art on disk). file:// URLs are CSP-blocked in
  // the webview — route them through Tauri's asset protocol instead.
  const art = $derived.by(() => {
    const url = player.metadata.artUrl;
    if (!url) return undefined;
    if (url.startsWith("file://")) return convertFileSrc(url.slice(7));
    if (url.startsWith("/")) return convertFileSrc(url);
    return url;
  });
  const playing = $derived(player.status === "playing");
  let showSettings = $state(false);

  // If the art URL 404s (e.g. a browser rotated its MPRIS cache file),
  // fall back to the placeholder instead of showing an empty square.
  let artFailed = $state(false);
  $effect(() => {
    art; artFailed = false; // reset when the URL changes
  });
  const showArt = $derived(art && !artFailed);

  // Extract accent color whenever the album art changes.
  $effect(() => {
    if (art) {
      dominantColor(art).then((c) => c && accent.set(c));
    }
  });
</script>

<div class="card" class:compact={$compact} data-tauri-drag-region>
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="art-wrap" ondblclick={() => compact.update((c) => !c)} title="Double-click for compact mode">
    {#if showArt}
      <img class="art" src={art} alt="" draggable="false" onerror={() => (artFailed = true)} />
    {:else}
      <div class="art placeholder">
        <svg viewBox="0 0 24 24"><path d="M12 3v10.55A4 4 0 1 0 14 17V7h4V3z" fill="currentColor"/></svg>
      </div>
    {/if}
  </div>

  <div class="info" data-tauri-drag-region>
    <div class="meta" data-tauri-drag-region>
      <span class="title" class:dim={!playing}>{player.metadata.title ?? "Unknown title"}</span>
      <div class="artist-row">
        {#if $settings.visualizer}
          <Visualizer {playing} />
        {/if}
        <span class="artist">{player.metadata.artist ?? player.playerName}</span>
      </div>
    </div>
    <Controls state={player} />
  </div>

  <div class="bottom" data-tauri-drag-region>
    <ProgressBar state={player} />
    {#if player.volume !== null}
      <VolumeControl state={player} />
    {/if}
    <button
      class="player-chip lyrics-btn"
      class:on={$showLyrics}
      onclick={() => { compact.set(false); showLyrics.update((v) => !v); }}
      title="Toggle lyrics"
    >♪</button>
    <button
      class="player-chip lyrics-btn"
      onclick={() =>
        theme.update((t: Theme) =>
          t === "dark" ? "light" : t === "light" ? "system" : "dark",
        )}
      title="Theme: {$theme} (click to cycle)"
    >{$theme === "dark" ? "☾" : $theme === "light" ? "☀" : "◐"}</button>
    <button
      class="player-chip lyrics-btn"
      class:on={showSettings}
      onclick={() => (showSettings = !showSettings)}
      title="Settings"
    >⚙</button>
    <span class="player-chip">{player.playerName}</span>
  </div>

  {#if showSettings}
    <SettingsPopover onclose={() => (showSettings = false)} />
  {/if}

  {#if $showLyrics}
    <div class="lyrics-row">
      <LyricsPanel state={player} />
    </div>
  {/if}
</div>

<style>
  .card {
    position: relative;
    display: grid;
    grid-template-columns: 100px 1fr;
    grid-template-rows: 1fr auto;
    gap: 6px 12px;
    width: 100%; height: 100%;
    padding: 14px;
    background: var(--bg);
    border-radius: var(--radius);
    backdrop-filter: blur(20px);
    -webkit-backdrop-filter: blur(20px);
    border: 1px solid var(--border);
    cursor: grab;
    overflow: hidden;
  }
  .card:active { cursor: grabbing; }

  .art-wrap { grid-row: 1 / 3; align-self: center; }
  .art {
    width: 100px; height: 100px;
    object-fit: cover;
    border-radius: 14px;
    box-shadow: 0 4px 16px rgba(0,0,0,0.35);
    pointer-events: none;
    animation: pop 0.25s ease;
  }
  .art.placeholder {
    display: grid; place-items: center;
    background: var(--chip-bg);
    color: var(--fg-dim);
  }
  .art.placeholder svg { width: 40px; height: 40px; }
  @keyframes pop {
    from { transform: scale(0.92); opacity: 0.4; }
    to { transform: scale(1); opacity: 1; }
  }

  .info {
    display: flex; align-items: center; justify-content: space-between;
    min-width: 0; gap: 8px;
  }
  .meta { display: flex; flex-direction: column; min-width: 0; }
  .title {
    font-size: 14px; font-weight: 600; color: var(--fg);
    white-space: nowrap; overflow: hidden; text-overflow: ellipsis;
  }
  .title.dim { color: var(--fg-dim); }
  .artist-row {
    display: flex; align-items: center; gap: 6px;
    min-width: 0;
  }
  .artist {
    font-size: 12px; color: var(--fg-dim);
    white-space: nowrap; overflow: hidden; text-overflow: ellipsis;
  }

  .bottom {
    display: flex; align-items: center; gap: 10px;
    min-width: 0;
  }
  .player-chip {
    font-size: 9px; font-weight: 600;
    text-transform: uppercase; letter-spacing: 0.06em;
    color: var(--fg-dim);
    background: var(--chip-bg);
    padding: 2px 7px;
    border-radius: 8px;
    white-space: nowrap;
  }

  /* --- Compact "pill" mode: art + controls only --- */
  .card.compact {
    grid-template-columns: 48px 1fr;
    grid-template-rows: 1fr;
    padding: 8px;
    gap: 10px;
    border-radius: 16px;
  }
  .card.compact .art-wrap { grid-row: 1; }
  .card.compact .art { width: 48px; height: 48px; border-radius: 10px; }
  .card.compact .art.placeholder svg { width: 22px; height: 22px; }
  .card.compact .meta { display: none; }
  .card.compact .bottom { display: none; }
  .card.compact .info { justify-content: center; }

  /* --- Lyrics row (spans full card width under the controls) --- */
  .lyrics-row { grid-column: 1 / 3; }
  .player-chip.lyrics-btn {
    border: none;
    cursor: pointer;
    font-family: inherit;
    font-size: 11px;
  }
  .player-chip.lyrics-btn.on {
    color: #fff;
    background: var(--accent);
  }
</style>
