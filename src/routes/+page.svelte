<script lang="ts">
  import { goto } from "$app/navigation";
  import { api } from "$lib/api";
  import { app } from "$lib/store.svelte";
  import { kindClass, kindLabel } from "$lib/ui";
  import XpBar from "$lib/components/XpBar.svelte";
  import type { Home } from "$lib/types";

  let home = $state<Home | null>(null);
  let starting = $state(false);

  async function load() {
    try {
      home = await api.home();
    } catch (e) {
      app.error(e);
    }
  }
  $effect(() => {
    void app.profile?.completed.length; // refresh when progress changes
    void app.profile?.settings.track; // ...or the track switches
    void load();
  });

  // Nothing available: find the requirement that blocks the active track.
  const blocker = $derived.by(() => {
    const p = app.profile;
    if (!home || home.next || !p) return null;
    for (const w of app.catalog[home.track.id] ?? [])
      for (const q of w.quests) {
        if (p.completed.includes(q.id)) continue;
        const missing = q.requires.find((r) => !p.completed.includes(r));
        if (missing) return { quest: q, missing };
      }
    return null;
  });

  async function start(id: string) {
    starting = true;
    try {
      await api.startQuest(id);
      await goto(`/quest/${id}`);
    } catch (e) {
      app.error(e);
    } finally {
      starting = false;
    }
  }
</script>

<div class="mx-auto flex max-w-2xl flex-col gap-8 px-6 py-10">
  <XpBar />

  {#if home?.next}
    {@const q = home.next}
    <section class="rounded-3xl border border-line bg-panel p-8 shadow-xl" data-testid="next-card">
      <div class="mb-1 text-xs font-semibold tracking-wide text-dim uppercase" data-testid="next-track">{home.track.title}</div>
      <div class="mb-4 flex items-center gap-3 text-sm">
        <span class="rounded-full border px-3 py-0.5 font-semibold {kindClass[q.kind]}">{kindLabel[q.kind]}</span>
        <span class="text-dim">{q.minutes} min</span>
        <span class="font-bold text-rust-hi">+{q.xp} XP</span>
      </div>
      <h1 class="text-4xl leading-tight font-extrabold">{q.title}</h1>
      <p class="mt-3 text-lg text-dim">{q.summary}</p>
      <button class="btn btn-primary btn-lg mt-8 w-full" disabled={starting} onclick={() => start(q.id)}>
        {app.profile?.started.includes(q.id) ? "Continue" : "Start"} →
      </button>
    </section>

    {#if home.alternate}
      <a href="/quest/{home.alternate.id}" class="text-center text-dim hover:text-ink">or: <span class="underline">{home.alternate.title}</span></a>
    {/if}
    {#if home.resume}
      <a href="/quest/{home.resume.id}" class="flex items-center justify-between rounded-2xl border border-line bg-panel-2 px-5 py-3 hover:border-rust/50">
        <span>Continue <b>{home.resume.title}</b></span><span class="text-rust-hi">→</span>
      </a>
    {/if}
  {:else if home && blocker}
    <section class="rounded-3xl border border-line bg-panel p-10 text-center" data-testid="blocked">
      <div class="text-xs font-semibold tracking-wide text-dim uppercase">{home.track.title}</div>
      <div class="mt-3 text-6xl">🔒</div>
      <h1 class="mt-4 text-3xl font-extrabold">Not unlocked yet</h1>
      <p class="mt-2 text-dim">{blocker.quest.title} needs another quest first.</p>
      <a href="/quest/{blocker.missing}" class="btn btn-primary btn-lg mt-6">Needs {app.questPath(blocker.missing)} →</a>
    </section>
  {:else if home}
    <section class="rounded-3xl border border-line bg-panel p-10 text-center">
      <div class="text-6xl">🏁</div>
      <h1 class="mt-4 text-3xl font-extrabold">Nothing left right now</h1>
      <p class="mt-2 text-dim">You cleared everything that is unlocked. Check the map.</p>
      <a href="/map" class="btn btn-primary btn-lg mt-6">Open map</a>
    </section>
  {:else}
    <div class="h-72 animate-pulse rounded-3xl bg-panel"></div>
  {/if}
</div>
