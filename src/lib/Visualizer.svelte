<script lang="ts">
  // Decorative equalizer bars — animates while `playing`, freezes otherwise.
  // (A real FFT visualizer would need audio capture, which Wayland doesn't
  // permit without a PipeWire permission prompt — out of scope for a mini player.)
  let { playing }: { playing: boolean } = $props();
  const bars = [0.9, 0.6, 1.0, 0.45, 0.8];
</script>

<div class="viz" class:paused={!playing} aria-hidden="true">
  {#each bars as d, i}
    <span style="animation-duration: {d}s; animation-delay: -{i * 0.17}s"></span>
  {/each}
</div>

<style>
  .viz {
    display: flex;
    align-items: flex-end;
    gap: 2px;
    height: 14px;
  }
  span {
    width: 3px;
    border-radius: 2px;
    background: var(--accent);
    animation: bounce 0.9s ease-in-out infinite alternate;
  }
  .viz.paused span {
    animation-play-state: paused;
    height: 20%;
    opacity: 0.4;
  }
  @keyframes bounce {
    from { height: 15%; }
    to { height: 100%; }
  }
</style>
