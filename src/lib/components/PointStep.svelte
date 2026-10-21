<script lang="ts">
  import type { Point } from "$lib/types";
  import { renderInline } from "$lib/markdown";

  let { point, solved, onsolve, lang = "", compact = false }: { point: Point; solved: boolean; onsolve: () => void; lang?: string; compact?: boolean } = $props();

  let wrong = $state<number[]>([]);
  let value = $state("");
  const isPick = $derived(point.pick.length > 0);
  const parts = $derived(point.code ? point.code.split("___") : null);
  const blankOk = $derived(point.blank !== null && value === point.blank);
  $effect(() => { if (blankOk && !solved) onsolve(); });

  function choose(i: number) {
    if (solved) return;
    if (i === point.answer) onsolve();
    else if (!wrong.includes(i)) wrong = [...wrong, i];
  }
</script>

<div class="point" class:compact data-testid="point-step">
  <p class="text-2xl leading-snug font-semibold">{@html renderInline(point.prompt, lang)}</p>
  {#if isPick}
    <div class="mt-5 flex flex-wrap gap-2">
      {#each point.pick as tok, i}
        <button
          class="rounded-lg border px-4 py-1.5 font-mono text-xl {solved && i === point.answer ? 'border-ok bg-ok/10 text-ok' : wrong.includes(i) ? 'border-bad bg-bad/10 text-bad' : 'border-line bg-panel-2 hover:border-rust'}"
          onclick={() => choose(i)} data-testid="pick" data-state={solved && i === point.answer ? "right" : wrong.includes(i) ? "wrong" : ""}
        >{tok}</button>
      {/each}
    </div>
  {:else}
    {#if parts && parts.length > 1}
      <pre class="mt-5 rounded-xl border border-line bg-[#0d0c0b] p-5 font-mono text-xl leading-relaxed whitespace-pre-wrap">{parts[0]}<input class="blank" class:ok={solved || blankOk} bind:value disabled={solved} size={Math.max(6, (point.blank ?? "").length + 2)} spellcheck="false" autocomplete="off" data-testid="blank" />{parts.slice(1).join("___")}</pre>
    {:else}
      {#if point.code}<pre class="mt-5 rounded-xl border border-line bg-[#0d0c0b] p-5 font-mono text-xl whitespace-pre-wrap">{point.code}</pre>{/if}
      <input class="blank mt-5" class:ok={solved || blankOk} bind:value disabled={solved} size={Math.max(10, (point.blank ?? "").length + 2)} spellcheck="false" autocomplete="off" data-testid="blank" />
    {/if}
  {/if}
  <p class="mt-4 min-h-6 text-base {solved ? 'font-semibold text-ok' : 'text-dim'}">{solved ? "Yes." : isPick ? "Tap one. Guessing is free." : "Type it exactly."}</p>
</div>

<style>
  .blank { font-family: var(--font-mono); background: #0d0c0b; color: var(--color-ink); border: 1px solid var(--color-rust); border-radius: 0.5rem; padding: 0.1rem 0.5rem; outline: none; }
  .compact :global(.text-2xl) { font-size: 1.125rem; }
  .compact :global(.text-xl) { font-size: 1rem; }
  .compact :global(.mt-5) { margin-top: 0.75rem; }
  .compact pre { padding: 0.75rem 1rem; }
  .blank.ok { border-color: var(--color-ok); color: var(--color-ok); }
</style>
