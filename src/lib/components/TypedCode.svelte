<script module lang="ts">
  /** cards already animated this session: remounting the card must not replay */
  const played = new Set<string>();
</script>

<script lang="ts">
  import { onDestroy, onMount } from "svelte";
  import { highlight, type CardCode } from "$lib/markdown";

  let { code, playKey, animate = true }: { code: CardCode; playKey: string; animate?: boolean } = $props();

  const reduced = () => typeof matchMedia === "function" && matchMedia("(prefers-reduced-motion: reduce)").matches;
  const MS = 25;
  const MAX_MS = 6000;

  let n = $state(0);
  let typing = $state(false);
  let timer: ReturnType<typeof setTimeout> | null = null;

  function play() {
    if (timer) clearTimeout(timer);
    if (reduced()) { n = code.raw.length; typing = false; return; }
    const step = Math.max(1, Math.ceil((code.raw.length * MS) / MAX_MS));
    n = 0;
    typing = true;
    const tick = () => {
      n = Math.min(code.raw.length, n + step);
      if (n >= code.raw.length) { typing = false; return; }
      timer = setTimeout(tick, MS);
    };
    timer = setTimeout(tick, MS);
  }
  onMount(() => {
    if (!animate) { n = code.raw.length; return; }
    if (played.has(playKey)) { n = code.raw.length; return; }
    played.add(playKey);
    play();
  });
  onDestroy(() => timer && clearTimeout(timer));

  const full = $derived(n >= code.raw.length);
  const partial = $derived(typing ? highlight(code.raw.slice(0, n), code.lang) : code.html);
</script>

<div class="group relative" data-testid="typed-code">
  <div class="prose-ox big-code">
    <!-- the finished block sizes the box so the card does not jump while typing -->
    <pre class={typing ? "invisible" : ""}><code class="hljs language-{code.lang}">{@html code.html}</code></pre>
    {#if typing}
      <pre class="absolute inset-0 m-0"><code class="hljs language-{code.lang}">{@html partial}<span class="caret">&nbsp;</span></code></pre>
    {/if}
  </div>
  {#if animate && !reduced()}
    <button class="absolute top-2 right-2 rounded-md border border-line bg-panel-2 px-2.5 py-1 text-xs text-dim hover:text-ink" onclick={play} data-testid="replay">▶ Replay</button>
  {/if}
</div>

<style>
  .big-code :global(pre) { padding: 1.25rem 1.4rem; }
  .big-code :global(pre code) { font-size: 1.2rem; line-height: 1.65; }
  .caret { display: inline-block; width: 0.55em; background: var(--color-rust-hi); animation: blink 1s steps(2) infinite; }
  @keyframes blink { 50% { opacity: 0 } }
</style>
