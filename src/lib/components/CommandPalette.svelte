<script lang="ts">
  import { goto } from "$app/navigation";
  import { tick } from "svelte";
  import { app, LOOKS } from "$lib/store.svelte";
  import type { Look } from "$lib/types";

  let { open = $bindable(false) }: { open?: boolean } = $props();

  type Item = { label: string; sub: string; run: () => void };
  let q = $state("");
  let sel = $state(0);
  let input = $state<HTMLInputElement | null>(null);

  const lookName: Record<Look, string> = { workbench: "Workbench", notebook: "Notebook", arcade: "Arcade" };
  const items = $derived.by<Item[]>(() => {
    const p = app.profile;
    const out: Item[] = [
      { label: "Home", sub: "Page", run: () => goto("/") },
      { label: "Map", sub: "Page", run: () => goto("/map") },
      { label: "Profile & settings", sub: "Page", run: () => goto("/profile") },
      ...LOOKS.map((l) => ({ label: `Look: ${lookName[l]}`, sub: l === app.look ? "Current look" : "Switch look", run: () => void app.setLook(l) })),
      ...app.tracks.map((t) => ({ label: `Track: ${t.title}`, sub: t.id === app.track?.id ? "Current track" : "Switch track", run: () => void app.switchTrack(t.id) })),
    ];
    for (const w of app.catalog[app.track?.id ?? ""] ?? [])
      for (const x of w.quests) {
        const done = p?.completed.includes(x.id);
        const open = p ? x.requires.every((r) => p.completed.includes(r)) : true;
        if (!open && !done) continue;
        out.push({ label: x.title, sub: `${w.title} · ${done ? "done" : `${x.minutes} min`}`, run: () => goto(`/quest/${x.id}`) });
      }
    return out;
  });
  const shown = $derived.by(() => {
    const t = q.trim().toLowerCase();
    if (!t) return items.slice(0, 12);
    return items.filter((i) => `${i.label} ${i.sub}`.toLowerCase().includes(t)).slice(0, 12);
  });
  $effect(() => { void q; sel = 0; });
  $effect(() => {
    if (!open) return;
    q = "";
    void tick().then(() => input?.focus());
  });

  function pick(i: Item | undefined) {
    if (!i) return;
    open = false;
    i.run();
  }
  function key(e: KeyboardEvent) {
    if (e.key === "ArrowDown") { e.preventDefault(); sel = Math.min(sel + 1, shown.length - 1); }
    else if (e.key === "ArrowUp") { e.preventDefault(); sel = Math.max(sel - 1, 0); }
    else if (e.key === "Enter") { e.preventDefault(); pick(shown[sel]); }
    else if (e.key === "Escape") { e.preventDefault(); e.stopPropagation(); open = false; }
  }
</script>

{#if open}
  <div class="fixed inset-0 z-[80] flex justify-center bg-black/40 pt-[12vh]" role="presentation" onclick={(e) => { if (e.target === e.currentTarget) open = false; }}>
    <div class="pop h-fit w-[34rem] max-w-[calc(100vw-2rem)] overflow-hidden rounded-[var(--radius-box)] border border-line bg-panel shadow-2xl" role="dialog" aria-label="Command palette" data-testid="palette">
      <input
        bind:this={input} bind:value={q} onkeydown={key}
        class="w-full border-b border-line bg-transparent px-4 py-3 text-ink outline-none placeholder:text-dim"
        placeholder="Jump to a quest, page, look or track…" spellcheck="false" autocomplete="off"
      />
      <ul class="max-h-[50vh] overflow-auto p-1.5">
        {#each shown as it, i}
          <li>
            <button class="flex w-full items-baseline justify-between gap-4 rounded-md px-3 py-2 text-left {i === sel ? 'bg-tint text-ink' : 'text-ink'}" onmouseenter={() => (sel = i)} onclick={() => pick(it)}>
              <span class="truncate">{it.label}</span><span class="shrink-0 text-xs text-dim">{it.sub}</span>
            </button>
          </li>
        {:else}
          <li class="px-3 py-3 text-sm text-dim">Nothing matches.</li>
        {/each}
      </ul>
      <div class="flex gap-4 border-t border-line px-4 py-2 text-xs text-dim"><span>↑↓ pick</span><span>Enter open</span><span>Esc close</span></div>
    </div>
  </div>
{/if}
