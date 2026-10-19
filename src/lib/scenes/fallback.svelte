<!-- Generic scene: an XP crystal that fills with the share of objectives passed. -->
<script lang="ts">
  import Frame from "./Frame.svelte";
  import { prog, uid, type SceneProps } from "./scene";
  let { quest, objectives }: SceneProps = $props();
  const p = $derived(prog(objectives));
  const id = uid("crystal");
</script>

<Frame complete={p.complete} label="XP crystal">
  <defs>
    <clipPath id={id}><polygon points="160,14 198,58 160,116 122,58" /></clipPath>
  </defs>
  <polygon points="160,14 198,58 160,116 122,58" fill="#221e1a" stroke="#38332e" stroke-width="2" />
  <g clip-path="url(#{id})">
    <rect x="110" y="14" width="100" height="102" fill="#fb923c" style="transform:translateY({(1 - p.frac) * 102}px);transition:transform .8s ease" />
    <path d="M160 14 122 58h76zM160 116 140 58h40z" fill="#fff" opacity="0.12" />
  </g>
  <polygon points="160,14 198,58 160,116 122,58" fill="none" stroke={p.complete ? "#fdba74" : "#6b6258"} stroke-width="2" style="transition:stroke .6s" />
  <rect x="60" y="130" width="200" height="8" rx="4" fill="#262320" />
  <rect x="60" y="130" width="200" height="8" rx="4" fill="#fb923c" style="transform:scaleX({p.frac});transform-origin:60px 134px;transition:transform .8s ease" />
  <text class="t" x="60" y="152">{p.n}/{p.total} objectives</text>
  <text class="t" x="260" y="152" text-anchor="end" style="fill:{p.complete ? '#fb923c' : '#a89d92'}">+{quest.xp} XP</text>
</Frame>
