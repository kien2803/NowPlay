<script lang="ts">
  import { settings, theme, type Theme } from "./stores";

  let { onclose }: { onclose: () => void } = $props();

  const themes: { id: Theme; label: string }[] = [
    { id: "dark", label: "Dark" },
    { id: "light", label: "Light" },
    { id: "system", label: "System" },
  ];

  function toggle(key: "visualizer" | "lyricsAutoOpen" | "accentFromArt" | "alwaysOnTop" | "autoShowOnPlay") {
    settings.update((s) => ({ ...s, [key]: !s[key] }));
  }

  const toggles: { key: "visualizer" | "lyricsAutoOpen" | "accentFromArt" | "alwaysOnTop" | "autoShowOnPlay"; label: string }[] = [
    { key: "visualizer", label: "Visualizer" },
    { key: "lyricsAutoOpen", label: "Auto-open lyrics" },
    { key: "accentFromArt", label: "Accent from album art" },
    { key: "alwaysOnTop", label: "Always on top" },
    { key: "autoShowOnPlay", label: "Auto-show on play" },
  ];
</script>

<button class="backdrop" onclick={onclose} aria-label="Close settings"></button>
<div class="popover">
  {#each toggles as t}
    <div class="row">
      <span>{t.label}</span>
      <button class="switch" class:on={$settings[t.key]} onclick={() => toggle(t.key)} aria-label={t.label}>
        <i></i>
      </button>
    </div>
  {/each}
  <div class="row">
    <span>Opacity</span>
    <input
      class="slider"
      type="range" min="0.4" max="1" step="0.05"
      value={$settings.bgOpacity}
      oninput={(e) => settings.update((s) => ({ ...s, bgOpacity: parseFloat((e.target as HTMLInputElement).value) }))}
    />
  </div>
  <div class="row">
    <span>Theme</span>
    <div class="seg">
      {#each themes as t}
        <button class:on={$theme === t.id} onclick={() => theme.set(t.id)}>{t.label}</button>
      {/each}
    </div>
  </div>
</div>

<style>
  .backdrop {
    position: absolute;
    inset: 0;
    z-index: 10;
    border: none;
    background: transparent;
    cursor: default;
    padding: 0;
  }
  .popover {
    position: absolute;
    right: 10px;
    bottom: 46px;
    z-index: 11;
    width: 210px;
    padding: 12px;
    background: var(--bg);
    backdrop-filter: blur(24px);
    border: 1px solid var(--border);
    border-radius: 14px;
    box-shadow: 0 8px 28px rgba(0, 0, 0, 0.4);
    display: flex;
    flex-direction: column;
    gap: 10px;
    cursor: default;
  }
  .row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    font-size: 12px;
    color: var(--fg);
  }
  .switch {
    width: 34px;
    height: 19px;
    border-radius: 10px;
    border: none;
    background: var(--fill);
    cursor: pointer;
    position: relative;
    transition: background 0.2s;
  }
  .switch i {
    position: absolute;
    top: 2px;
    left: 2px;
    width: 15px;
    height: 15px;
    border-radius: 50%;
    background: #fff;
    transition: transform 0.2s;
  }
  .switch.on { background: var(--accent); }
  .switch.on i { transform: translateX(15px); }
  .seg {
    display: flex;
    background: var(--chip-bg);
    border-radius: 9px;
    padding: 2px;
    gap: 2px;
  }
  .seg button {
    border: none;
    background: transparent;
    color: var(--fg-dim);
    font-size: 10px;
    padding: 3px 8px;
    border-radius: 7px;
    cursor: pointer;
    font-family: inherit;
  }
  .seg button.on {
    background: var(--accent);
    color: #fff;
  }
  .slider {
    -webkit-appearance: none;
    appearance: none;
    width: 90px;
    height: 4px;
    border-radius: 2px;
    background: var(--fill);
    outline: none;
    cursor: pointer;
  }
  .slider::-webkit-slider-thumb {
    -webkit-appearance: none;
    width: 12px;
    height: 12px;
    border-radius: 50%;
    background: var(--accent);
  }
</style>
