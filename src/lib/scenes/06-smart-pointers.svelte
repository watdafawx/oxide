<!-- Machine graph: machines and belts link up; items ride the belts once, when throughput passes. -->
<script lang="ts">
  import Frame from "./Frame.svelte";
  import { lit, prog, type SceneProps } from "./scene";
  let { objectives }: SceneProps = $props();
  const S = 6;
  const p = $derived(prog(objectives));
  const on = (k: number) => lit(p.frac, k, S);
  const nodes = [{ x: 18, n: "miner", c: "#fb923c" }, { x: 123, n: "smelter", c: "#f87171" }, { x: 228, n: "assembler", c: "#60a5fa" }];
</script>

<Frame complete={p.complete} label="Machines linked by belts">
  {#each nodes as nd}
    <rect x={nd.x} y="56" width="56" height="50" rx="6" fill="none" stroke="#38332e" stroke-dasharray="3 4" />
  {/each}
  {#each [0, 1] as b}
    <g class="s" class:off={!on(2 + b * 2)}>
      <rect x={72 + b * 105} y="76" width="51" height="10" fill="#262320" stroke="#38332e" />
      {#each [0, 1, 2, 3] as c}<path d="M{80 + b * 105 + c * 11} 78l4 3-4 3" stroke="#a89d92" stroke-width="1.4" fill="none" />{/each}
    </g>
    {#if on(2 + b * 2)}
      <rect class="item" class:run={p.complete} x={80 + b * 105} y="78" width="6" height="6" rx="1" fill={b ? "#cbd5e1" : "#fb923c"} />
    {/if}
  {/each}
  {#each nodes as nd, i}
    <g class="s pop" class:off={!on(1 + i * 2)}>
      <rect x={nd.x} y="56" width="56" height="50" rx="6" fill="#221e1a" stroke={nd.c} stroke-width="2" />
      <circle cx={nd.x + 28} cy="76" r="9" fill="none" stroke={nd.c} stroke-width="2" /><circle cx={nd.x + 28} cy="76" r="3" fill={nd.c} />
      <text class="t" x={nd.x + 28} y="98" text-anchor="middle" style="font-size:8px">{nd.n}</text>
    </g>
  {/each}
  <g class="s" class:off={!on(6)}>
    <path d="M256 54C220 18 90 18 46 54" stroke="#c084fc" stroke-width="1.5" stroke-dasharray="4 3" fill="none" />
    <text x="150" y="30" text-anchor="middle" style="fill:#c084fc;font-size:9px">Weak</text>
    <text x="150" y="132" text-anchor="middle" style="fill:#4ade80;font-size:10px">12 items/s</text>
  </g>
</Frame>

<style>
  .item { transition: opacity 0.4s; }
  .item.run { animation: ride 1.4s ease-in-out 0.3s both; }
  @keyframes ride { from { transform: translateX(0); } to { transform: translateX(34px); } }
</style>
