<script lang="ts">
  import { api } from "$lib/api";
  import { app } from "$lib/store.svelte";
  import XpBar from "$lib/components/XpBar.svelte";
  import { preview } from "$lib/sound";
  import type { Settings, SoundKey } from "$lib/types";

  let dates = $state<Set<string>>(new Set());
  let autosave = $state((() => { try { return localStorage.getItem("oxide:autosave") !== "0"; } catch { return true; } })());
  const setAutosave = (v: boolean) => { autosave = v; try { localStorage.setItem("oxide:autosave", v ? "1" : "0"); } catch { /* private mode */ } };
  let form = $state<Settings>({ editor: "", sound: true, focus_minutes: 25, track: "", flavour: "games", volume: 0.6, sounds: { objective: true, clear: true, level: true, achievement: true, crit: true } });
  const soundRows: [SoundKey, string, string][] = [
    ["objective", "Objective tick", "a rising note per objective"],
    ["clear", "All green", "a soft chord"],
    ["level", "Level up", "a short fanfare"],
    ["achievement", "Achievement", "a sparkle"],
    ["crit", "Crit", "a quick zap"],
  ];
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
      const s = { ...form, volume: Number(form.volume), focus_minutes: Math.max(1, Math.round(Number(form.focus_minutes) || 25)) };
      form = await api.setSettings(s);
      dirty = false;
      saved = true;
      setTimeout(() => (saved = false), 1800);
    } catch (e) {
      app.error(e);
    }
  }

  let flavours = $state<string[]>(["games", "plain"]);
  $effect(() => { api.listFlavours().then((f) => (flavours = f)).catch(() => {}); });
  const flavourLabel = (f: string) => (f === "games" ? "Game" : f === "plain" ? "Plain" : f);
  async function setFlavour(f: string) {
    if (!app.profile || f === app.profile.settings.flavour) return;
    try {
      await api.setSettings({ ...app.profile.settings, flavour: f });
      app.profile = await api.getProfile();
      form.flavour = f;
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
  // The engine has no per-track XP field: sum quest xp from the catalog.
  const trackXp = $derived(
    app.tracks.map((t) => {
      const qs = (app.catalog[t.id] ?? []).flatMap((w) => w.quests);
      const total = qs.reduce((n, x) => n + x.xp, 0);
      const earned = qs.filter((x) => p?.completed.includes(x.id)).reduce((n, x) => n + x.xp, 0);
      return { track: t, earned, total, pct: total ? (earned / total) * 100 : 0 };
    }),
  );
  const stats = $derived(p ? [["Level", p.level], ["Total XP", p.xp], ["Best streak", `${p.best_streak}d`]] : []);

  // ---------- profiles: each has its own XP, progress and quest code
  let newName = $state("");
  let renaming = $state<string | null>(null);
  let renameTo = $state("");
  let confirmDelete = $state<string | null>(null);
  let busy = $state(false);
  let importInput = $state<HTMLInputElement | null>(null);

  async function profileAction(fn: () => Promise<unknown>) {
    if (busy) return;
    busy = true;
    try {
      await fn();
    } catch (e) {
      app.error(e);
    } finally {
      busy = false;
    }
  }
  const createProfile = () =>
    profileAction(async () => {
      app.profiles = await api.createProfile(newName);
      newName = "";
    });
  const renameProfile = (id: string) =>
    profileAction(async () => {
      app.profiles = await api.renameProfile(id, renameTo);
      renaming = null;
    });
  const deleteProfile = (id: string) =>
    profileAction(async () => {
      app.profiles = await api.deleteProfile(id);
      confirmDelete = null;
    });
  const exportProfile = (id: string) =>
    profileAction(async () => {
      const path = await api.exportProfile(id);
      app.toast("info", `Saved to ${path}`);
    });
  function importFile(file: File) {
    const reader = new FileReader();
    reader.onload = () =>
      profileAction(async () => {
        const data = String(reader.result).split(",", 2)[1] ?? "";
        app.profiles = await api.importProfile(data);
        app.toast("info", `Imported ${file.name}. Switch to it below.`);
      });
    reader.readAsDataURL(file);
  }
</script>

<div class="mx-auto flex max-w-3xl flex-col gap-8 px-6 py-10">
  <h1 class="text-3xl font-extrabold">Profile</h1>
  <XpBar />

  {#if trackXp.length}
    <div class="-mt-4 flex flex-col gap-2 px-5" data-testid="track-xp">
      {#each trackXp as t (t.track.id)}
        <div class="flex items-center gap-3 text-xs text-dim">
          <span class="w-12 shrink-0 font-semibold text-ink">{t.track.title}</span>
          <div class="h-1.5 flex-1 overflow-hidden rounded-full bg-panel-2">
            <div class="h-full rounded-full bg-rust transition-all duration-700" style="width:{t.pct}%"></div>
          </div>
          <span class="w-20 shrink-0 text-right tabular-nums">{t.earned} / {t.total} XP</span>
        </div>
      {/each}
    </div>
  {/if}

  <div class="grid grid-cols-3 gap-4 text-center">
    {#each stats as [label, v]}
      <div class="rounded-2xl border border-line bg-panel p-5">
        <div class="text-3xl font-extrabold text-rust-hi tabular-nums">{v}</div>
        <div class="text-sm text-dim">{label}</div>
      </div>
    {/each}
  </div>

  <section data-testid="profiles">
    <h2 class="mb-1 text-xl font-bold">Profiles</h2>
    <p class="mb-3 text-sm text-dim">Each profile keeps its own XP, progress, settings and quest code. Export one to back it up or take it to another PC.</p>
    <div class="flex flex-col divide-y divide-line rounded-2xl border border-line bg-panel">
      {#each app.profiles as pr (pr.id)}
        <div class="flex flex-wrap items-center gap-3 px-5 py-3" data-testid="profile-row" data-active={pr.active}>
          {#if renaming === pr.id}
            <form class="flex flex-1 gap-2" onsubmit={(e) => { e.preventDefault(); renameProfile(pr.id); }}>
              <input class="flex-1 rounded-xl border border-line bg-bg px-3 py-1.5" bind:value={renameTo} maxlength="40" aria-label="New name" />
              <button class="btn btn-primary !py-1.5" type="submit" disabled={busy || !renameTo.trim()}>Save</button>
              <button class="btn btn-ghost !py-1.5" type="button" onclick={() => (renaming = null)}>Cancel</button>
            </form>
          {:else}
            <div class="flex flex-1 flex-col">
              <span class="font-semibold">{pr.name}{#if pr.active}<span class="ml-2 rounded-full bg-rust/20 px-2 py-0.5 text-xs text-rust-hi">active</span>{/if}</span>
              <span class="text-xs text-dim">since {pr.created}</span>
            </div>
            {#if !pr.active}
              <button class="btn btn-primary !py-1.5 text-sm" disabled={busy} onclick={() => app.switchProfile(pr.id)}>Switch</button>
            {/if}
            <button class="btn btn-ghost !py-1.5 text-sm" disabled={busy} onclick={() => { renaming = pr.id; renameTo = pr.name; }}>Rename</button>
            <button class="btn btn-ghost !py-1.5 text-sm" disabled={busy} onclick={() => exportProfile(pr.id)}>Export</button>
            {#if !pr.active}
              {#if confirmDelete === pr.id}
                <button class="btn !py-1.5 text-sm bg-red-600 text-white hover:bg-red-500" disabled={busy} onclick={() => deleteProfile(pr.id)}>Delete {pr.name} and its code</button>
                <button class="btn btn-ghost !py-1.5 text-sm" onclick={() => (confirmDelete = null)}>Keep</button>
              {:else}
                <button class="btn btn-ghost !py-1.5 text-sm text-dim hover:text-red-400" disabled={busy} onclick={() => (confirmDelete = pr.id)}>Delete</button>
              {/if}
            {/if}
          {/if}
        </div>
      {/each}
      <form class="flex flex-wrap items-center gap-2 px-5 py-3" onsubmit={(e) => { e.preventDefault(); createProfile(); }}>
        <input class="flex-1 rounded-xl border border-line bg-bg px-3 py-1.5" bind:value={newName} placeholder="New profile name" maxlength="40" aria-label="New profile name" data-testid="new-profile-name" />
        <button class="btn btn-primary !py-1.5 text-sm" type="submit" disabled={busy || !newName.trim()}>Create</button>
        <button class="btn btn-ghost !py-1.5 text-sm" type="button" disabled={busy} onclick={() => importInput?.click()}>Import…</button>
        <input type="file" accept=".zip" class="hidden" bind:this={importInput} onchange={(e) => { const f = e.currentTarget.files?.[0]; if (f) importFile(f); e.currentTarget.value = ""; }} />
      </form>
    </div>
  </section>

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
        {@const secret = a.hidden && !got}
        <div class="rounded-2xl border p-4 {got ? 'border-rust/60 bg-rust/10' : 'border-line bg-panel opacity-50'}" data-earned={got} data-hidden={secret}>
          <div class="text-2xl">{got ? "🏆" : "🔒"}</div>
          <div class="mt-1 font-bold">{secret ? "???" : a.title}</div>
          <div class="text-sm text-dim">{secret ? "Hidden. Keep playing." : a.description}</div>
        </div>
      {/each}
    </div>
  </section>

  <section>
    <h2 class="mb-3 text-xl font-bold">Settings</h2>
    <form class="flex flex-col gap-5 rounded-2xl border border-line bg-panel p-6" onsubmit={(e) => { e.preventDefault(); void save(); }} oninput={() => (dirty = true)}>
      <label class="flex flex-col gap-1.5">
        <span class="font-semibold">External editor command (optional)</span>
        <input class="rounded-xl border border-line bg-bg px-4 py-2.5 font-mono" bind:value={form.editor} placeholder="code (leave empty to code in Oxide only)" />
        <span class="text-sm text-dim">Quests are edited right in Oxide. Set this to also get an "Open in …" button; it runs with the quest folder as its argument.</span>
      </label>
      <div class="flex flex-col gap-1.5">
        <span class="font-semibold">Style</span>
        <div class="flex w-fit gap-1 rounded-xl border border-line bg-bg p-1" data-testid="flavour">
          {#each flavours as f}
            <button type="button" class="rounded-lg px-4 py-1.5 font-medium {app.profile?.settings.flavour === f ? 'bg-rust text-black' : 'text-dim hover:text-ink'}" onclick={() => setFlavour(f)}>{flavourLabel(f)}</button>
          {/each}
        </div>
        <span class="text-sm text-dim">Game tells a story around each quest. Plain is just the lesson. Applies right away.</span>
      </div>
      <label class="flex items-center gap-3">
        <input type="checkbox" class="h-5 w-5 accent-orange-500" checked={autosave} onchange={(e) => setAutosave(e.currentTarget.checked)} data-testid="autosave" />
        <span class="font-semibold">Autosave code</span>
        <span class="text-sm text-dim">1.5 s after you stop typing. Ctrl+S always works.</span>
      </label>
      <div class="flex flex-col gap-3" data-testid="sound-settings">
        <label class="flex items-center gap-3">
          <input type="checkbox" class="h-5 w-5 accent-orange-500" bind:checked={form.sound} />
          <span class="font-semibold">Sound</span>
        </label>
        <label class="flex items-center gap-3">
          <span class="w-16 text-sm text-dim">Volume</span>
          <input type="range" min="0" max="1" step="0.05" class="w-56 accent-orange-500" bind:value={form.volume} data-testid="volume" />
          <span class="w-10 text-sm text-dim tabular-nums">{Math.round(form.volume * 100)}%</span>
        </label>
        <div class="flex flex-col gap-1 rounded-xl border border-line bg-bg p-3">
          {#each soundRows as [key, label, hint]}
            <div class="flex items-center gap-3 py-1">
              <label class="flex flex-1 items-center gap-3">
                <input type="checkbox" class="h-5 w-5 accent-orange-500" bind:checked={form.sounds[key]} data-testid="sound-{key}" />
                <span class="font-medium">{label}</span>
                <span class="text-sm text-dim">{hint}</span>
              </label>
              <button type="button" class="btn btn-ghost !px-3 !py-1 text-sm" onclick={() => preview(key, Number(form.volume))} data-testid="test-{key}">test</button>
            </div>
          {/each}
        </div>
      </div>
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
