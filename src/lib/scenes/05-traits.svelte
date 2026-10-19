<!-- Entity system: mobs spawn onto a field one by one, each with a health bar. -->
<script lang="ts">
  import Frame from "./Frame.svelte";
  import { lit, prog, type SceneProps } from "./scene";
  let { objectives }: SceneProps = $props();
  const S = 5;
  const p = $derived(prog(objectives));
  const hp = [0.7, 0.55, 0.85, 0.4, 0.65];
  const xs = [30, 92, 154, 216, 270];
</script>

<Frame complete={p.complete} label="Mobs on a field with health bars">
  <rect y="104" width="320" height="56" fill="#1d2a1b" />
  <rect y="104" width="320" height="3" fill="#2f4a2b" />
  <circle cx="276" cy="30" r="12" fill="#fbbf24" opacity="0.55" />
  {#each xs as x, i}
    <g transform="translate({x} 0)">
      <g class="s pop" class:off={!lit(p.frac, i + 1, S)}>
        <rect x="-12" y="46" width="24" height="4" rx="2" fill="#38332e" />
        <rect x="-12" y="46" width="24" height="4" rx="2" fill="#4ade80" style="transform:scaleX({p.complete ? 1 : hp[i]});transform-origin:-12px 48px;transition:transform .8s ease" />
        {#if i === 0}
          <rect x="-8" y="60" width="16" height="14" fill="#5b8f4e" /><rect x="-8" y="74" width="16" height="18" fill="#3b6ea5" />
          <rect x="-8" y="76" width="22" height="5" fill="#5b8f4e" /><rect x="-8" y="92" width="7" height="12" fill="#2f4a73" /><rect x="1" y="92" width="7" height="12" fill="#2f4a73" />
          <rect x="-5" y="64" width="3" height="3" fill="#111" /><rect x="2" y="64" width="3" height="3" fill="#111" />
        {:else if i === 1}
          <rect x="-9" y="58" width="18" height="18" fill="#5fbf5a" /><rect x="-7" y="76" width="14" height="16" fill="#4ea84a" />
          <rect x="-9" y="92" width="8" height="12" fill="#4ea84a" /><rect x="1" y="92" width="8" height="12" fill="#4ea84a" />
          <rect x="-6" y="63" width="4" height="4" fill="#111" /><rect x="2" y="63" width="4" height="4" fill="#111" /><rect x="-3" y="68" width="6" height="6" fill="#111" />
        {:else if i === 2}
          <rect x="-8" y="60" width="16" height="15" fill="#c89b78" /><rect x="-2" y="68" width="4" height="8" fill="#a67c5b" />
          <rect x="-10" y="75" width="20" height="29" fill="#7a5a3a" /><rect x="-11" y="82" width="22" height="6" fill="#5c4228" />
          <rect x="-5" y="64" width="3" height="2" fill="#2a6b3a" /><rect x="2" y="64" width="3" height="2" fill="#2a6b3a" />
        {:else if i === 3}
          <rect x="-7" y="60" width="14" height="14" fill="#d4d4d4" /><rect x="-5" y="75" width="10" height="18" fill="#bdbdbd" />
          <rect x="-8" y="93" width="5" height="11" fill="#d4d4d4" /><rect x="3" y="93" width="5" height="11" fill="#d4d4d4" /><rect x="7" y="76" width="12" height="3" fill="#a16207" />
          <rect x="-4" y="64" width="3" height="3" fill="#111" /><rect x="1" y="64" width="3" height="3" fill="#111" />
        {:else}
          <rect x="-14" y="82" width="28" height="16" rx="3" fill="#f4a6b8" /><rect x="10" y="78" width="12" height="12" rx="2" fill="#f4a6b8" />
          <rect x="19" y="83" width="5" height="5" fill="#e07a93" /><rect x="-12" y="98" width="6" height="6" fill="#e07a93" /><rect x="6" y="98" width="6" height="6" fill="#e07a93" />
          <rect x="15" y="80" width="2" height="2" fill="#111" />
        {/if}
      </g>
    </g>
  {/each}
</Frame>
