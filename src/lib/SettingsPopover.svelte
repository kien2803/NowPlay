<script lang="ts">
  import { settings, theme, type Theme } from "./stores";

  let { onclose }: { onclose: () => void } = $props();

  const themes: { id: Theme; label: string }[] = [
    { id: "dark", label: "Dark" },
    { id: "light", label: "Light" },
    { id: "system", label: "System" },
  ];

  function toggle(key: "visualizer" | "lyricsAutoOpen") {
    settings.update((s) => ({ ...s, [key]: !s[key] }));
  }
</script>

<button class="backdrop" onclick={onclose} aria-label="Close settings"></button>
<div class="popover">
  <div class="row">
    <span>Visualizer</span>
    <button class="switch" class:on={$settings.visualizer} onclick={() => toggle("visualizer")} aria-label="Toggle visualizer">
      <i></i>
    </button>
  </div>
  <div class="row">
    <span>Auto-open lyrics</span>
    <button class="switch" class:on={$settings.lyricsAutoOpen} onclick={() => toggle("lyricsAutoOpen")} aria-label="Toggle auto lyrics">
      <i></i>
    </button>
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
</style>
