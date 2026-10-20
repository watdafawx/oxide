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
    void app.profile?.settings.flavour;
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

  // Past 20:00 with nothing done today and a streak to lose: offer the shortest quest that is open.
  const atRisk = $derived.by(() => {
    const p = app.profile;
    if (!p || p.streak_days < 1 || new Date().getHours() < 20) return null;
    const d = new Date();
    const today = `${d.getFullYear()}-${String(d.getMonth() + 1).padStart(2, "0")}-${String(d.getDate()).padStart(2, "0")}`;
    if (p.last_active === today) return null;
    const open = (app.catalog[app.track?.id ?? ""] ?? [])
      .flatMap((w) => w.quests)
      .filter((q) => !p.completed.includes(q.id) && q.requires.every((r) => p.completed.includes(r)));
    return open.sort((a, b) => a.minutes - b.minutes)[0] ?? null;
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

  {#if atRisk}
    <a href="/quest/{atRisk.id}" class="flex items-center justify-between gap-3 rounded-2xl border border-amber-300/40 bg-amber-300/10 px-5 py-3 text-amber-200 hover:border-amber-300" data-testid="streak-nudge">
      <span>Streak at risk: one quest keeps it alive</span>
      <span class="shrink-0 font-semibold">{atRisk.title} · {atRisk.minutes} min →</span>
    </a>
  {/if}

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
