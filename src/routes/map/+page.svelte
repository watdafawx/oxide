<script lang="ts">
  import { goto } from "$app/navigation";
  import { api } from "$lib/api";
  import { app } from "$lib/store.svelte";
  import { kindLabel } from "$lib/ui";
  import type { Quest, World } from "$lib/types";

  let worlds = $state<World[]>([]);
  let loaded = $state(false);

  $effect(() => {
    void app.profile?.completed.length;
    void app.profile?.settings.flavour;
    const t = app.track?.id;
    api.listWorlds(t).then((w) => { worlds = w; loaded = true; }).catch((e) => app.error(e));
  });

  type NodeState = "locked" | "available" | "started" | "done";
  function stateOf(q: Quest): NodeState {
    const p = app.profile;
    if (!p) return "locked";
    if (p.completed.includes(q.id)) return "done";
    if (!q.requires.every((r) => p.completed.includes(r))) return "locked";
    return p.started.includes(q.id) ? "started" : "available";
  }
  const done = (w: World) => w.quests.filter((q) => app.profile?.completed.includes(q.id)).length;
  function title(q: Quest) {
    const base = `${q.title} (${kindLabel[q.kind]}, +${q.xp} XP)`;
    const p = app.profile;
    const foreign = p ? q.requires.find((r) => !p.completed.includes(r) && r.split("/")[0] !== q.track_id) : undefined;
    return foreign ? `${base}. Needs ${app.questPath(foreign)} (other track)` : base;
  }

  const ring: Record<NodeState, string> = {
    locked: "border-line bg-panel text-dim/50 cursor-not-allowed",
    available: "border-rust bg-panel-2 text-rust-hi hover:bg-rust/20 cursor-pointer",
    started: "border-amber-300 bg-amber-300/15 text-amber-200 cursor-pointer",
    done: "border-ok bg-ok/20 text-ok cursor-pointer",
  };
  const glyph: Record<NodeState, string> = { locked: "🔒", available: "", started: "…", done: "✓" };
</script>

<div class="mx-auto max-w-3xl px-6 py-10">
  <h1 class="mb-8 text-3xl font-extrabold">Map{#if app.track}<span class="ml-3 text-xl font-bold text-dim">{app.track.title}</span>{/if}</h1>
  {#if !loaded}
    <div class="h-48 animate-pulse rounded-2xl bg-panel"></div>
  {/if}
  <div class="flex flex-col gap-6">
    {#each worlds as w (w.id)}
      <section class="rounded-2xl border border-line bg-panel p-6">
        <div class="mb-5 flex items-baseline justify-between gap-4">
          <div>
            <h2 class="text-xl font-bold">{w.title}</h2>
            <p class="text-sm text-dim">{w.blurb}</p>
          </div>
          <span class="font-bold text-rust-hi tabular-nums">{done(w)}/{w.quests.length}</span>
        </div>
        <div class="flex flex-wrap items-center gap-x-3 gap-y-4">
          {#each w.quests as q, i (q.id)}
            {@const s = stateOf(q)}
            {#if i > 0}<div class="h-0.5 w-6 {s === 'locked' ? 'bg-line' : 'bg-rust/50'}"></div>{/if}
            <button
              class="group relative flex shrink-0 items-center justify-center border-2 text-lg font-bold transition
                {q.kind === 'boss' ? 'h-16 w-16 rotate-45 rounded-lg' : 'h-14 w-14 rounded-full'} {ring[s]}"
              disabled={s === "locked"}
              title={title(q)}
              aria-label="{title(q)} — {s}"
              data-state={s}
              onclick={() => goto(`/quest/${q.id}`)}
            >
              <span class={q.kind === "boss" ? "-rotate-45" : ""}>{glyph[s] || (q.kind === "read" ? "📖" : q.index)}</span>
            </button>
          {/each}
        </div>
      </section>
    {/each}
  </div>
</div>
