<script lang="ts">
  import { api } from "$lib/api";
  import { app } from "$lib/store.svelte";
  import XpBar from "$lib/components/XpBar.svelte";
  import type { Settings } from "$lib/types";

  let dates = $state<Set<string>>(new Set());
  let form = $state<Settings>({ editor: "", sound: true, focus_minutes: 25 });
  let saved = $state(false);
  let dirty = $state(false);

  $effect(() => {
    api.activeDates().then((d) => (dates = new Set(d))).catch((e) => app.error(e));
  });
  $effect(() => {
    // seed the form once the profile arrives, but never clobber edits
    if (app.profile && !dirty) form = { ...app.profile.settings };
  });

  async function save() {
    try {
      const s = { ...form, focus_minutes: Math.max(1, Math.round(Number(form.focus_minutes) || 25)) };
      form = await api.setSettings(s);
      dirty = false;
      saved = true;
      setTimeout(() => (saved = false), 1800);
    } catch (e) {
      app.error(e);
    }
  }

  const iso = (d: Date) => `${d.getFullYear()}-${String(d.getMonth() + 1).padStart(2, "0")}-${String(d.getDate()).padStart(2, "0")}`;
  // 12 weeks, columns = weeks (Mon..Sun rows), last column = this week
  const weeks = $derived.by(() => {
    const today = new Date();
    today.setHours(12, 0, 0, 0);
    const dow = (today.getDay() + 6) % 7; // Mon=0
    const start = new Date(today);
    start.setDate(today.getDate() - dow - 7 * 11);
    const cols: { date: string; future: boolean }[][] = [];
    for (let w = 0; w < 12; w++) {
      const col = [];
      for (let d = 0; d < 7; d++) {
        const dt = new Date(start);
        dt.setDate(start.getDate() + w * 7 + d);
        col.push({ date: iso(dt), future: dt > today });
      }
      cols.push(col);
    }
    return cols;
  });
  const p = $derived(app.profile);
  const stats = $derived(p ? [["Level", p.level], ["Total XP", p.xp], ["Best streak", `${p.best_streak}d`]] : []);
</script>

<div class="mx-auto flex max-w-3xl flex-col gap-8 px-6 py-10">
  <h1 class="text-3xl font-extrabold">Profile</h1>
  <XpBar />

  <div class="grid grid-cols-3 gap-4 text-center">
    {#each stats as [label, v]}
      <div class="rounded-2xl border border-line bg-panel p-5">
        <div class="text-3xl font-extrabold text-rust-hi tabular-nums">{v}</div>
        <div class="text-sm text-dim">{label}</div>
      </div>
    {/each}
  </div>

  <section>
    <h2 class="mb-3 text-xl font-bold">Last 12 weeks</h2>
    <div class="flex gap-1.5 overflow-x-auto rounded-2xl border border-line bg-panel p-5" data-testid="calendar">
      {#each weeks as col}
        <div class="flex flex-col gap-1.5">
          {#each col as c}
            <div
              class="h-5 w-5 rounded-[5px] {c.future ? 'bg-transparent' : dates.has(c.date) ? 'bg-rust' : 'bg-panel-2'}"
              title={c.date}
            ></div>
          {/each}
        </div>
      {/each}
    </div>
  </section>

  <section>
    <h2 class="mb-3 text-xl font-bold">Achievements</h2>
    <div class="grid grid-cols-2 gap-3 sm:grid-cols-3">
      {#each app.achievements as a (a.id)}
        {@const got = p?.achievements.includes(a.id)}
        <div class="rounded-2xl border p-4 {got ? 'border-rust/60 bg-rust/10' : 'border-line bg-panel opacity-50'}" data-earned={got}>
          <div class="text-2xl">{got ? "🏆" : "🔒"}</div>
          <div class="mt-1 font-bold">{a.title}</div>
          <div class="text-sm text-dim">{a.description}</div>
        </div>
      {/each}
    </div>
  </section>

  <section>
    <h2 class="mb-3 text-xl font-bold">Settings</h2>
    <form class="flex flex-col gap-5 rounded-2xl border border-line bg-panel p-6" onsubmit={(e) => { e.preventDefault(); void save(); }} oninput={() => (dirty = true)}>
      <label class="flex flex-col gap-1.5">
        <span class="font-semibold">Editor command</span>
        <input class="rounded-xl border border-line bg-bg px-4 py-2.5 font-mono" bind:value={form.editor} placeholder="code" />
        <span class="text-sm text-dim">Run with the quest folder as its argument.</span>
      </label>
      <label class="flex items-center gap-3">
        <input type="checkbox" class="h-5 w-5 accent-orange-500" bind:checked={form.sound} />
        <span class="font-semibold">Sound</span>
      </label>
      <label class="flex flex-col gap-1.5">
        <span class="font-semibold">Focus minutes</span>
        <input type="number" min="1" max="180" class="w-32 rounded-xl border border-line bg-bg px-4 py-2.5" bind:value={form.focus_minutes} />
      </label>
      <div class="flex items-center gap-4">
        <button class="btn btn-primary" type="submit" disabled={!dirty}>Save</button>
        {#if saved}<span class="text-ok">Saved ✓</span>{/if}
      </div>
    </form>
  </section>
</div>
