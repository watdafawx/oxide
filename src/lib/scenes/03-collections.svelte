<!-- Inventory and loot: a 9x4 grid fills with coloured items, the chest lid opens. -->
<script lang="ts">
  import Frame from "./Frame.svelte";
  import { prog, type SceneProps } from "./scene";
  let { objectives }: SceneProps = $props();
  const p = $derived(prog(objectives));
  const slots = [0, 1, 2, 3, 4, 5, 6, 9, 10, 12, 13, 14, 18, 19, 21, 22, 27, 28, 30, 31];
  const rarity = ["#9ca3af", "#4ade80", "#60a5fa", "#c084fc", "#fbbf24"];
  const filled = $derived(Math.round(p.frac * slots.length));
  const lid = $derived(p.complete ? -48 : p.frac >= 0.5 ? -14 : 0);
</script>

<Frame complete={p.complete} label="Inventory grid and loot chest">
  <text class="t" x="10" y="24">inventory</text>
  {#each Array(36) as _, i}
    <rect x={10 + (i % 9) * 24} y={32 + Math.floor(i / 9) * 24} width="22" height="22" rx="3" fill="#221e1a" stroke="#38332e" />
  {/each}
  {#each slots as cell, k}
    <rect class="s pop" class:off={k >= filled} x={14 + (cell % 9) * 24} y={36 + Math.floor(cell / 9) * 24} width="14" height="14" rx="3"
      fill={rarity[(k * 7 + 2) % 5]} />
  {/each}
  <g transform="translate(236 66)">
    <rect x="4" y="22" width="60" height="38" rx="4" fill="#8a5e2f" stroke="#5c3d1d" stroke-width="2" />
    <rect x="30" y="22" width="8" height="38" fill="#fbbf24" opacity="0.8" />
    <ellipse cx="34" cy="24" rx="28" ry="5" fill="#fbbf24" opacity={p.complete ? 0.85 : 0} style="transition:opacity .6s .3s" />
    <g style="transform:rotate({lid}deg);transform-origin:4px 22px;transition:transform .7s ease">
      <rect x="4" y="6" width="60" height="16" rx="4" fill="#a8742f" stroke="#5c3d1d" stroke-width="2" />
      <rect x="30" y="6" width="8" height="16" fill="#fbbf24" opacity="0.8" />
    </g>
  </g>
  <text class="t" x="270" y="152" text-anchor="middle">loot</text>
</Frame>
