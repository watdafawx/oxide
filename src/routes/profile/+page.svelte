<script lang="ts">
  import { api } from "$lib/api";
  import { app } from "$lib/store.svelte";
  import XpBar from "$lib/components/XpBar.svelte";
  import { preview } from "$lib/sound";
  import { LOOKS } from "$lib/store.svelte";
  import type { Look, Settings, SoundKey } from "$lib/types";

  let dates = $state<Set<string>>(new Set());
  let autosave = $state((() => { try { return localStorage.getItem("oxide:autosave") !== "0"; } catch { return true; } })());
  const setAutosave = (v: boolean) => { autosave = v; try { localStorage.setItem("oxide:autosave", v ? "1" : "0"); } catch { /* private mode */ } };
  let form = $state<Settings>({ editor: "", sound: true, focus_minutes: 25, track: "", flavour: "games", volume: 0.6, sounds: { objective: true, clear: true, level: true, achievement: true, crit: true }, look: "workbench", complete: true, explain: true });
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

  const lookInfo: Record<Look, { name: string; blurb: string; bg: string; panel: string; line: string; acc: string; ink: string }> = {
    workbench: { name: "Workbench", blurb: "Like a code editor: lesson on the side, code in the middle, problems and hints below.", bg: "#0f1318", panel: "#151a21", line: "#263040", acc: "#37c6b0", ink: "#e3e9f0" },
    notebook: { name: "Notebook", blurb: "Light and calm. Reads like a book, with hints as notes in the margin.", bg: "#f6f1e8", panel: "#fffdf8", line: "#e2d9ca", acc: "#c24e1c", ink: "#1f1b16" },
    arcade: { name: "Arcade", blurb: "Game first: quest log, big Run button, and Ferris gives the hints.", bg: "#0e0b1a", panel: "#17122b", line: "#2e2550", acc: "#ff7a3d", ink: "#f1edff" },
  };
  /** the editor switches apply right away, like the look */
  async function setEditorOpt(key: "complete" | "explain", v: boolean) {
    if (!app.profile) return;
    try {
      await api.setSettings({ ...app.profile.settings, [key]: v });
      app.profile = await api.getProfile();
      form[key] = v;
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
  <h1 class="font-[family-name:var(--font-display)] text-3xl font-extrabold">Profile</h1>
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

      <section class="flex flex-col gap-2" data-testid="look-picker">
    <h2 class="text-xl font-bold">Look</h2>
    <div class="grid gap-3 sm:grid-cols-3">
      {#each LOOKS as l}
        {@const k = lookInfo[l]}
        <button type="button" class="flex flex-col overflow-hidden rounded-[var(--radius-box)] border-2 text-left transition {app.look === l ? 'border-rust' : 'border-line hover:border-dim'}" onclick={() => app.setLook(l)} data-look-option={l} aria-pressed={app.look === l}>
          <!-- tiny preview of the layout -->
          <span class="flex h-20 gap-1 p-1.5" style="background:{k.bg}">
            {#if l === "workbench"}
              <span class="w-2 rounded-sm" style="background:{k.panel}"></span>
              <span class="w-8 rounded-sm" style="background:{k.panel}"></span>
              <span class="flex flex-1 flex-col gap-1"><span class="flex-1 rounded-sm border" style="border-color:{k.line}"></span><span class="h-4 rounded-sm" style="background:{k.panel}"></span><span class="h-1 rounded-sm" style="background:{k.acc}"></span></span>
            {:else if l === "notebook"}
              <span class="mx-auto flex w-24 flex-col gap-1 pt-1"><span class="h-2 w-16 rounded-sm" style="background:{k.ink}"></span><span class="h-1 rounded-sm" style="background:{k.line}"></span><span class="h-1 rounded-sm" style="background:{k.line}"></span><span class="flex-1 rounded-sm border" style="background:{k.panel};border-color:{k.line}"></span></span>
              <span class="mt-6 h-6 w-8 rounded-sm" style="background:#fdf4dc"></span>
            {:else}
              <span class="flex w-8 flex-col gap-1"><span class="h-4 rounded" style="border:1.5px solid {k.acc}"></span><span class="h-4 rounded" style="background:{k.panel}"></span></span>
              <span class="flex flex-1 flex-col gap-1"><span class="flex-1 rounded border" style="background:{k.panel};border-color:{k.line}"></span><span class="h-3 rounded" style="background:{k.panel}"></span></span>
              <span class="w-8 rounded" style="background:{k.panel}"><span class="m-1 block h-3 w-3 rounded-full" style="background:{k.acc}"></span></span>
            {/if}
          </span>
          <span class="flex flex-col gap-0.5 bg-panel px-3 py-2">
            <span class="flex items-center justify-between font-semibold">{k.name}{#if app.look === l}<span class="text-sm text-rust">✓</span>{/if}</span>
            <span class="text-xs leading-snug text-dim">{k.blurb}</span>
          </span>
        </button>
      {/each}
    </div>
    <span class="text-sm text-dim">Applies right away, and is saved with this profile. Ctrl+K also switches looks.</span>
  </section>
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
      <div class="flex flex-col gap-2" data-testid="editor-help">
        <span class="font-semibold">Code editor help</span>
        <label class="flex items-center gap-3">
          <input type="checkbox" class="h-5 w-5 accent-[var(--color-rust)]" checked={app.profile?.settings.complete ?? true} onchange={(e) => setEditorOpt("complete", e.currentTarget.checked)} data-testid="opt-complete" />
          <span class="font-medium whitespace-nowrap">Suggestions while typing</span>
          <span class="text-sm text-dim">Names, methods and keywords with a short note. Ctrl+Space shows them any time.</span>
        </label>
        <label class="flex items-center gap-3">
          <input type="checkbox" class="h-5 w-5 accent-[var(--color-rust)]" checked={app.profile?.settings.explain ?? true} onchange={(e) => setEditorOpt("explain", e.currentTarget.checked)} data-testid="opt-explain" />
          <span class="font-medium whitespace-nowrap">Explain errors in plain words</span>
          <span class="text-sm text-dim">A note under the broken line, with a fix button when there is an easy one.</span>
        </label>
      </div>
      <div class="flex flex-col gap-1.5">
        <span class="font-semibold">Style</span>
        <div class="flex w-fit gap-1 rounded-xl border border-line bg-bg p-1" data-testid="flavour">
          {#each flavours as f}
            <button type="button" class="rounded-lg px-4 py-1.5 font-medium {app.profile?.settings.flavour === f ? 'bg-rust text-on-rust' : 'text-dim hover:text-ink'}" onclick={() => setFlavour(f)}>{flavourLabel(f)}</button>
          {/each}
        </div>
        <span class="text-sm text-dim">Game tells a story around each quest. Plain is just the lesson. Applies right away.</span>
      </div>
      <label class="flex items-center gap-3">
        <input type="checkbox" class="h-5 w-5 accent-[var(--color-rust)]" checked={autosave} onchange={(e) => setAutosave(e.currentTarget.checked)} data-testid="autosave" />
        <span class="font-semibold">Autosave code</span>
        <span class="text-sm text-dim">1.5 s after you stop typing. Ctrl+S always works.</span>
      </label>
      <div class="flex flex-col gap-3" data-testid="sound-settings">
        <label class="flex items-center gap-3">
          <input type="checkbox" class="h-5 w-5 accent-[var(--color-rust)]" bind:checked={form.sound} />
          <span class="font-semibold">Sound</span>
        </label>
        <label class="flex items-center gap-3">
          <span class="w-16 text-sm text-dim">Volume</span>
          <input type="range" min="0" max="1" step="0.05" class="w-56 accent-[var(--color-rust)]" bind:value={form.volume} data-testid="volume" />
          <span class="w-10 text-sm text-dim tabular-nums">{Math.round(form.volume * 100)}%</span>
        </label>
        <div class="flex flex-col gap-1 rounded-xl border border-line bg-bg p-3">
          {#each soundRows as [key, label, hint]}
            <div class="flex items-center gap-3 py-1">
              <label class="flex flex-1 items-center gap-3">
                <input type="checkbox" class="h-5 w-5 accent-[var(--color-rust)]" bind:checked={form.sounds[key]} data-testid="sound-{key}" />
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
