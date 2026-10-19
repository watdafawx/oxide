<!-- Crafting calculator: the grid gains ingredients, the output appears when the recipe is complete. -->
<script lang="ts">
  import Frame from "./Frame.svelte";
  import { lit, prog, type SceneProps } from "./scene";
  let { objectives }: SceneProps = $props();
  const S = 6;
  const p = $derived(prog(objectives));
  const on = (k: number) => lit(p.frac, k, S);
  // ingredient cells in placement order: three planks, then two sticks
  const cells: { c: number; r: number; kind: "plank" | "stick" }[] = [
    { c: 0, r: 0, kind: "plank" }, { c: 1, r: 0, kind: "plank" }, { c: 2, r: 0, kind: "plank" },
    { c: 1, r: 1, kind: "stick" }, { c: 1, r: 2, kind: "stick" },
  ];
</script>

<Frame complete={p.complete} label="Crafting grid with a pickaxe recipe">
  <g transform="translate(22 0)">
    <text class="t" x="12" y="24">crafting</text>
    {#each [0, 1, 2] as r}
      {#each [0, 1, 2] as c}
        <rect x={12 + c * 30} y={34 + r * 30} width="28" height="28" rx="3" fill="#221e1a" stroke="#38332e" />
      {/each}
    {/each}
    {#each cells as it, i}
      <g class="s pop" class:off={!on(i + 1)}>
        {#if it.kind === "plank"}
          <rect x={15 + it.c * 30} y={37 + it.r * 30} width="22" height="22" rx="2" fill="#b9854a" stroke="#8a5e2f" />
          <path d="M16 {43 + it.r * 30}h20M16 {50 + it.r * 30}h20" stroke="#8a5e2f" stroke-width="1" transform="translate({it.c * 30} 0)" />
        {:else}
          <rect x={22 + it.c * 30} y={38 + it.r * 30} width="6" height="20" rx="2" fill="#8a5e2f" transform="rotate(20 {25 + it.c * 30} {48 + it.r * 30})" />
        {/if}
      </g>
    {/each}

    <g class="s" class:off={!on(6)}>
      <path d="M140 78h34m-8-8 8 8-8 8" stroke="#fb923c" stroke-width="3" fill="none" stroke-linecap="round" stroke-linejoin="round" />
    </g>
    <rect x="190" y="52" width="56" height="56" rx="6" fill="#221e1a" stroke={on(6) ? "#fb923c" : "#38332e"} stroke-width="2" />
    <g class="s pop" class:off={!on(6)}>
      <path d="M206 98 232 64" stroke="#8a5e2f" stroke-width="5" stroke-linecap="round" />
      <path d="M210 64Q226 50 244 70" stroke="#cbd5e1" stroke-width="6" fill="none" stroke-linecap="round" />
    </g>
    {#if p.complete}<text x="218" y="126" class="t" text-anchor="middle" style="fill:#4ade80">pickaxe ×1 ✓</text>{/if}
  </g>
</Frame>
