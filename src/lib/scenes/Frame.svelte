<script lang="ts">
  import type { Snippet } from "svelte";
  let { complete, label, children }: { complete: boolean; label: string; children: Snippet } = $props();
</script>

<svg viewBox="0 0 320 160" class="scene" class:complete role="img" aria-label={label} preserveAspectRatio="xMidYMid meet">
  <rect width="320" height="160" fill="#171411" />
  {@render children()}
</svg>

<style>
  .scene { width: 100%; height: 160px; display: block; filter: saturate(0.5) brightness(0.88); transition: filter 0.9s ease; font-family: ui-monospace, "Cascadia Code", Consolas, monospace; }
  .scene.complete { filter: none; }
  /* staged elements: hidden until their stage is lit, then ease in (one-shot, no looping motion) */
  :global(.scene .s) { transition: opacity 0.5s ease, transform 0.5s ease, fill 0.5s ease, stroke 0.5s ease; transform-box: fill-box; transform-origin: center; }
  :global(.scene .s.off) { opacity: 0; transform: translateY(5px); }
  :global(.scene .s.pop.off) { transform: scale(0.3); }
  :global(.scene .t) { fill: #a89d92; font-size: 9px; }
</style>
