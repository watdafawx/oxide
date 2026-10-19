<!-- Manifest loader: each mod row flips from red "missing" to green "loaded". -->
<script lang="ts">
  import Frame from "./Frame.svelte";
  import { lit, prog, type SceneProps } from "./scene";
  let { objectives }: SceneProps = $props();
  const S = 6;
  const p = $derived(prog(objectives));
  const mods = [["sodium", "0.6.0"], ["lithium", "0.14.3"], ["jei", "19.21"], ["create", "6.0.4"], ["ae2", "19.2.1"], ["curios", "9.0.0"]];
  const loaded = $derived(mods.filter((_, i) => lit(p.frac, i + 1, S)).length);
</script>

<Frame complete={p.complete} label="Mod manifest with loaded and missing mods">
  <text class="t" x="16" y="16">manifest.toml</text>
  <text class="t" x="304" y="16" text-anchor="end" style="fill:{p.complete ? '#4ade80' : '#f87171'}">{loaded}/{mods.length} loaded</text>
  {#each mods as [name, ver], i}
    {@const ok = lit(p.frac, i + 1, S)}
    <rect x="16" y={24 + i * 22} width="288" height="19" rx="4" fill={ok ? "#12301e" : "#341616"} stroke={ok ? "#2f7a4b" : "#7a2f2f"} style="transition:fill .5s,stroke .5s" />
    <text x="28" y={37 + i * 22} style="fill:#f2ebe4;font-size:10px">{name}</text>
    <text class="t" x="130" y={37 + i * 22}>{ver}</text>
    <g transform="translate(0 {i * 22})">
      <g class="s" class:off={ok}>
        <rect x="236" y="28" width="60" height="11" rx="5" fill="#f87171" /><text x="266" y="36.5" text-anchor="middle" style="fill:#2a0c0c;font-size:8px">missing</text>
      </g>
      <g class="s" class:off={!ok}>
        <rect x="236" y="28" width="60" height="11" rx="5" fill="#4ade80" /><text x="266" y="36.5" text-anchor="middle" style="fill:#07210f;font-size:8px">loaded</text>
      </g>
    </g>
  {/each}
</Frame>
