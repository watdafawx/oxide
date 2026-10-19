<!-- Server console: lines echo in as objectives pass; the server goes online at the end. -->
<script lang="ts">
  import Frame from "./Frame.svelte";
  import { lit, prog, type SceneProps } from "./scene";
  let { objectives }: SceneProps = $props();
  const S = 7;
  const p = $derived(prog(objectives));
  const on = (k: number) => lit(p.frac, k, S);
  const lines = [
    { t: "> start", c: "#fb923c" },
    { t: "[info] server ready on :25565", c: "#a89d92" },
    { t: "Steve joined the game", c: "#4ade80" },
    { t: "> list", c: "#fb923c" },
    { t: "Alex joined the game", c: "#4ade80" },
    { t: "> kick Alex", c: "#fb923c" },
    { t: "Alex left the game", c: "#f87171" },
  ];
</script>

<Frame complete={p.complete} label="Server console log">
  <rect x="16" y="12" width="288" height="136" rx="8" fill="#0d0c0b" stroke="#38332e" />
  <path d="M16 20a8 8 0 0 1 8-8h272a8 8 0 0 1 8 8v12H16z" fill="#262320" />
  <circle cx="30" cy="22" r="4" fill={p.complete ? "#4ade80" : "#f87171"} style="transition:fill .6s" />
  <text class="t" x="42" y="25">server.log</text>
  <text class="t" x="294" y="25" text-anchor="end" style="fill:{p.complete ? '#4ade80' : '#f87171'}">{p.complete ? "online" : "offline"}</text>
  {#each lines as l, i}
    <text class="s" class:off={!on(i + 1)} x="26" y={50 + i * 13.5} style="fill:{l.c};font-size:10px">{l.t}</text>
  {/each}
  <rect x="26" y={50 + 7 * 13.5 - 8} width="6" height="10" fill="#a89d92" opacity="0.7" />
</Frame>
