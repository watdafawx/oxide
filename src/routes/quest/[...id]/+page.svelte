<script lang="ts">
  import { page } from "$app/state";
  import { api } from "$lib/api";
  import { app } from "$lib/store.svelte";
  import { play } from "$lib/sound";
  import { langOf, lessonCards, readingMinutes, renderMarkdown, splitLesson } from "$lib/markdown";
  import QuestScene from "$lib/components/QuestScene.svelte";
  import CodeEditor from "$lib/components/CodeEditor.svelte";
  import TypedCode from "$lib/components/TypedCode.svelte";
  import PointStep from "$lib/components/PointStep.svelte";
  import { kindClass, kindLabel, mmss } from "$lib/ui";
  import { onDestroy, tick, untrack } from "svelte";
  import { explain, langOfPath, type Lang } from "$lib/assist";
  import type { CheckResult, ObjectiveResult, QuestDetail } from "$lib/types";

  type Stage = "check" | "build";
  type Step = { kind: "card"; i: number } | { kind: "point"; pi: number };

  const id = $derived(page.params.id ?? "");

  let detail = $state<QuestDetail | null>(null);
  let busy = $state(false);
  let copied = $state(false);
  let codeCopied = $state(false);
  let resetN = $state(0);
  let expanded = $state<Record<string, boolean>>({});
  let answers = $state<number[]>([]);
  let quizMsg = $state("");

  const ls = {
    get(k: string): string | null { try { return localStorage.getItem(k); } catch { return null; } },
    set(k: string, v: string) { try { localStorage.setItem(k, v); } catch { /* private mode */ } },
  };
  let ticks = $state<number[]>([]);
  let ideaOpen = $state(false);
  const loadList = (k: string): number[] => { try { const v = JSON.parse(ls.get(k) ?? "[]"); return Array.isArray(v) ? v.filter((x) => typeof x === "number") : []; } catch { return []; } };
  const toggleTick = (i: number) => {
    if (!ticks.includes(i)) void credit(100 + i);
    ticks = ticks.includes(i) ? ticks.filter((x) => x !== i) : [...ticks, i];
    ls.set(`oxide:ticks:${id}`, JSON.stringify(ticks));
  };
  const setIdea = (v: boolean) => { ideaOpen = v; ls.set(`oxide:idea:${id}`, v ? "1" : "0"); };

  // a derived dedupes: XP events replace `app.profile` but must not reset the page
  const flavourNow = $derived(app.profile?.settings.flavour);
  $effect(() => {
    const qid = id;
    try { ticks = JSON.parse(ls.get(`oxide:ticks:${qid}`) ?? "[]"); } catch { ticks = []; }
    ideaOpen = ls.get(`oxide:idea:${qid}`) === "1";
    void flavourNow;
    detail = null;
    seenTick = untrack(() => app.checks[qid]?.tick ?? 0);
    expanded = {};
    answers = [];
    quizMsg = "";
    lessonShut = ls.get(`oxide:lesson:${qid}`) === "0";
    redRun = {};
    chkQ = 0; chkWrong = []; chkRight = null;
    seen = loadList(`oxide:read:${qid}`);
    solved = Object.fromEntries(loadList(`oxide:solved:${qid}`).map((n) => [n, true]));
    freeUsed = ls.get(`oxide:freehint:${qid}`) === "1";
    api.getQuest(qid).then((d) => { if (qid === id) { detail = d; initStage(d); } }).catch((e) => app.error(e));
  });

  const q = $derived(detail?.quest ?? null);
  const check = $derived(app.checks[id]);
  const checking = $derived(check?.running ?? false);
  const result = $derived<CheckResult | null>(check?.result ?? detail?.last_check ?? null);
  const started = $derived(!!detail?.workspace);
  const done = $derived(detail?.completed ?? false);
  const lang = $derived(langOf(detail?.quest.runner));
  const lesson = $derived(detail ? renderMarkdown(detail.lesson, lang) : "");
  const parts = $derived(detail && detail.quest.kind !== "read" ? splitLesson(detail.lesson, lang) : null);
  const readMin = $derived(detail ? readingMinutes(detail.lesson) : 1);
  const cards = $derived(detail && detail.quest.kind !== "read" ? lessonCards(detail.lesson, lang) : null);
  /** exercises and bosses with a "Your turn" list run as Learn / Check / Build; read quests keep one page */
  const staged = $derived(!!cards && !!parts);
  const firstOpen = $derived(parts ? parts.tasks.findIndex((_, i) => !ticks.includes(i)) : -1);

  // the first red result after the idea is still collapsed opens it once; the starter run does not count
  let seenTick = 0;
  $effect(() => {
    const qid = id;
    const c = app.checks[qid];
    const t = c?.tick ?? 0;
    if (t === seenTick) return;
    seenTick = t;
    if (!c?.result) return;
    if (c.result.ok) { redRun = {}; return; }
    if (app.fresh.delete(qid)) return;
    if (c.result.compiled) {
      const next: Record<string, number> = {};
      for (const o of c.result.objectives) next[o.test] = o.status === "failed" ? (untrack(() => redRun)[o.test] ?? 0) + 1 : 0;
      redRun = next;
    }
    if (ls.get(`oxide:idea:${qid}`) === null) untrack(() => setIdea(true));
  });
  const focusHere = $derived(app.focus && app.focus.quest_id === id ? app.focus : null);

  // Refresh counters (attempts, completed) after each result.
  $effect(() => {
    void check?.tick;
    if (check?.tick) api.getQuest(id).then((d) => { if (d.quest.id === id) detail = d; }).catch(() => {});
  });

  // rows tick over one at a time after a result (store.onResult drives `shown`)
  const rows = $derived<ObjectiveResult[]>(
    result?.objectives.map((r, i) => (i < (check?.shown ?? Infinity) ? r : { ...r, status: "notrun" as const, output: null })) ??
      (q?.objectives ?? []).map((o) => ({ ...o, status: "notrun" as const, output: null })),
  );
  let flash = $state(false);
  let flashSeen = app.critFlash;
  $effect(() => {
    if (app.critFlash === flashSeen) return;
    flashSeen = app.critFlash;
    flash = true;
    setTimeout(() => (flash = false), 70);
  });
  // a cleared quest shows its finished scene even when no check result is loaded
  const sceneRows = $derived<ObjectiveResult[]>(done ? rows.map((r) => ({ ...r, status: "passed" as const })) : rows);
  const errors = $derived(result && !result.compiled ? result.diagnostics : []);
  const passedCount = $derived(rows.filter((r) => r.status === "passed").length);

  async function wrap<T>(fn: () => Promise<T>): Promise<T | undefined> {
    busy = true;
    try {
      return await fn();
    } catch (e) {
      app.error(e);
    } finally {
      busy = false;
    }
  }
  const start = () => wrap(async () => { detail = await app.beginQuest(id); });
  const reset = () => wrap(async () => {
    if (!confirm("Reset your copy of this quest? Progress stays, your code is wiped.")) return;
    delete app.checks[id];
    detail = await api.resetQuest(id);
    resetN++;
  });
  const externalName = $derived.by(() => {
    const e = (app.profile?.settings.editor ?? "").trim().split(/[\/]/).pop()?.replace(/\.(exe|cmd|bat)$/i, "").toLowerCase() ?? "";
    return e === "code" || e === "code-insiders" ? "VS Code" : e || "editor";
  });
  const openEditor = () => wrap(() => api.openEditor(id));
  const runChecks = () => wrap(async () => { await api.runChecks(id); });
  async function copyCode(e: MouseEvent) {
    const pre = (e.currentTarget as HTMLElement).parentElement?.querySelector("pre");
    try { await navigator.clipboard.writeText(pre?.textContent ?? ""); codeCopied = true; setTimeout(() => (codeCopied = false), 1500); } catch { app.toast("error", "Copy failed"); }
  }
  const hint = () => wrap(async () => {
    if (freeHintReady) { freeUsed = true; ls.set(`oxide:freehint:${id}`, "1"); return; }
    const n = await api.useHint(id);
    if (detail) detail.hints_used = n;
  });
  const focus = () =>
    wrap(async () => {
      if (focusHere) { await api.stopFocus(false); app.focus = null; }
      else await api.startFocus(app.profile?.settings.focus_minutes ?? 25, id);
    });
  async function copy() {
    if (!detail?.workspace) return;
    try { await navigator.clipboard.writeText(detail.workspace); copied = true; setTimeout(() => (copied = false), 1500); } catch { app.toast("error", "Copy failed"); }
  }
  const checkQuiz = () => wrap(async () => {
    const c = await api.answerQuiz(id, answers);
    if (!c) { quizMsg = "Not quite. Re-read and try again."; return; }
    quizMsg = "";
    play("clear");
    if (detail) detail.completed = true;
    if (c.first_time) app.celebrate(c);
  });

  // ---- stages: Check / Build. The lesson sits on the Build page, above the task. ----
  let stage = $state<Stage>("build");
  let lessonShut = $state(false);
  let lessonEl = $state<HTMLElement | null>(null);
  let objEl = $state<HTMLElement | null>(null);
  let seen = $state<number[]>([]);
  let solved = $state<Record<number, boolean>>({});
  let xpToast = $state<{ n: number; key: number } | null>(null);
  let chkQ = $state(0);
  let chkWrong = $state<number[]>([]);
  let chkRight = $state<number | null>(null);
  let redRun = $state<Record<string, number>>({});
  let freeUsed = $state(false);
  let lastSaveAt = $state(Date.now());
  let nowMs = $state(Date.now());

  /** lesson cards with the points slotted in after the card they follow */
  const steps = $derived.by<Step[]>(() => {
    if (!cards || !q) return [];
    const out: Step[] = [];
    const n = cards.cards.length;
    const at = (a: number) => q.points.forEach((p, pi) => { if (Math.min(p.after, n) === a) out.push({ kind: "point", pi }); });
    at(0);
    cards.cards.forEach((_, i) => { out.push({ kind: "card", i }); at(i + 1); });
    return out;
  });
  const hasLearn = $derived(steps.length > 0);
  const stageList = $derived<Stage[]>([...(q && q.quiz.length ? ["check" as const] : []), "build" as const]);
  const stageNames = { check: "Check", build: "Build" } as const;
  const stageXp = $derived<Record<Stage, string>>({ check: "+5 XP", build: `+${q?.xp ?? 0} XP` });
  const lessonVisible = $derived(staged && stage === "build" && hasLearn && !lessonShut);
  /** an open lesson gets its own column beside the code; folded, the code gets the width back */
  const lessonSide = $derived(hasLearn && !lessonShut && app.look !== "notebook");

  function initStage(d: QuestDetail) {
    const saved = ls.get(`oxide:stage:${d.quest.id}`);
    const returning = saved !== null || d.completed || d.hints_used > 0 || (!!d.workspace && !app.fresh.has(d.quest.id));
    stage = d.quest.quiz.length && !returning && saved !== "build" ? "check" : saved === "check" && d.quest.quiz.length ? "check" : "build";
  }
  function goStage(s: Stage) {
    stage = s;
    ls.set(`oxide:stage:${id}`, s);
  }
  function setLesson(open: boolean) {
    lessonShut = !open;
    ls.set(`oxide:lesson:${id}`, open ? "1" : "0");
  }
  async function showLesson() {
    setLesson(true);
    await tick();
    lessonEl?.scrollIntoView({ behavior: "smooth", block: "start" });
  }
  async function credit(index: number) {
    try {
      const xp = await api.readPoint(id, index);
      if (xp > 0) {
        xpToast = { n: xp, key: Date.now() };
        play("objective");
        const k = xpToast.key;
        setTimeout(() => { if (xpToast?.key === k) xpToast = null; }, 900);
      }
    } catch { /* XP is a bonus; never block reading */ }
  }
  // an open lesson counts as read, card by card
  $effect(() => {
    if (!lessonVisible || !cards) return;
    const all = cards.cards.map((_, i) => i);
    untrack(() => {
      const fresh = all.filter((i) => !seen.includes(i));
      if (!fresh.length) return;
      seen = [...seen, ...fresh];
      ls.set(`oxide:read:${id}`, JSON.stringify(seen));
      for (const i of fresh) void credit(i);
    });
  });
  function solvePoint(pi: number) {
    if (solved[pi]) return;
    solved[pi] = true;
    ls.set(`oxide:solved:${id}`, JSON.stringify(Object.keys(solved).map(Number)));
    void credit(50 + pi);
  }
  // the editor already shows the primary file, so an "Open ..." first step ticks itself (no XP, once)
  $effect(() => {
    if (!(staged && stage === "build" && started && parts)) return;
    untrack(() => {
      if (ls.get(`oxide:ticks:${id}`) !== null || ticks.includes(0)) return;
      if (/^\s*Open/i.test((parts?.tasks[0] ?? "").replace(/<[^>]+>/g, ""))) { ticks = [0]; ls.set(`oxide:ticks:${id}`, JSON.stringify(ticks)); }
    });
  });
  const pointsLeft = $derived(q ? q.points.filter((_, pi) => !solved[pi]).length : 0);

  // check stage
  const cq = $derived(q?.quiz[chkQ]);
  function pickAnswer(ci: number) {
    if (!cq || chkRight !== null) return;
    if (ci === cq.answer) { chkRight = ci; void credit(200 + chkQ); play("clear"); }
    else if (!chkWrong.includes(ci)) chkWrong = [...chkWrong, ci];
  }
  function afterCheck() {
    if (q && chkQ < q.quiz.length - 1) { chkQ++; chkWrong = []; chkRight = null; }
    else goStage("build");
  }

  // build stage: current task <-> TODO line, and the stuck timer
  const todoIdx = $derived(staged && stage === "build" && firstOpen >= 0 ? firstOpen : null);
  const ghostCount = $derived(parts ? parts.tasks.filter((_, i) => !ticks.includes(i) && i !== firstOpen).length : 0);
  const stuckS = $derived.by(() => {
    if (import.meta.env.DEV) { const v = Number(page.url.searchParams.get("stuck")); if (v > 0) return v; }
    return 90;
  });
  const buildLive = $derived(staged && stage === "build" && started && !done);
  $effect(() => {
    if (!buildLive) return;
    const h = setInterval(() => (nowMs = Date.now()), 1000);
    return () => clearInterval(h);
  });
  $effect(() => {
    if (staged && stage === "build" && started) untrack(() => { lastSaveAt = Date.now(); nowMs = Date.now(); });
  });
  const stuckNow = $derived(buildLive && (nowMs - lastSaveAt >= stuckS * 1000 || Object.values(redRun).some((n) => n >= 2)));
  const onEditorSave = () => { lastSaveAt = Date.now(); nowMs = lastSaveAt; };

  // ---- next quest ----
  const nextHere = $derived(app.next && app.next.from === id ? app.next : null);
  $effect(() => { if (done && app.next?.from !== id) void app.refreshNext(id); });
  function onKey(e: KeyboardEvent) {
    if (e.key !== "Enter" || e.ctrlKey || e.metaKey || e.altKey || e.shiftKey || !done || !nextHere?.quest) return;
    const el = e.target as HTMLElement | null;
    if (el && (el.isContentEditable || ["INPUT", "TEXTAREA", "SELECT", "BUTTON", "A"].includes(el.tagName))) return;
    if (document.querySelector('[role="dialog"]')) return;
    e.preventDefault();
    void app.goNext();
  }

  // ---- run pane ----
  const run = $derived(app.runs[id]);
  const running = $derived(run?.running ?? false);
  let consoleEl = $state<HTMLElement | null>(null);
  let panelEl = $state<HTMLElement | null>(null);
  let stdin = $state("");
  const runProgram = async () => {
    app.startRun(id);
    openBuild = {};
    wbTab = "console";
    consoleOpen = true;
    stdin = "";
    await tick();
    panelEl?.scrollIntoView({ behavior: "smooth", block: "nearest" });
    try {
      await api.runProgram(id);
    } catch (e) {
      app.runs[id].running = false;
      app.error(e);
    }
  };
  const stopProgram = () => api.stopProgram().catch((e) => app.error(e));
  async function sendInput() {
    const text = stdin;
    stdin = "";
    app.runLine(id, { stream: "input", line: text });
    try { await api.runInput(text); } catch (e) { app.error(e); }
  }
  $effect(() => {
    void run?.lines.length;
    if (consoleEl) consoleEl.scrollTop = consoleEl.scrollHeight;
  });
  let openBuild = $state<Record<number, boolean>>({});
  /** consecutive "build" lines fold into one collapsible row */
  const consoleRows = $derived.by(() => {
    const out: ({ kind: "line"; l: { stream: keyof typeof lineClass; line: string } } | { kind: "build"; at: number; lines: string[] })[] = [];
    (run?.lines ?? []).forEach((l, i) => {
      const last = out[out.length - 1];
      if (l.stream !== "build") out.push({ kind: "line", l });
      else if (last?.kind === "build") last.lines.push(l.line);
      else out.push({ kind: "build", at: i, lines: [l.line] });
    });
    return out;
  });
  const lineClass = { build: "text-dim", stdout: "text-ink", stderr: "text-dim", system: "text-rust-hi", input: "text-ok" } as const;

  const freeHintReady = $derived(stuckNow && !freeUsed && (detail?.hints_used ?? 0) === 0 && !!q?.hints.length);
  const hintsShown = $derived(q ? q.hints.slice(0, (detail?.hints_used ?? 0) + (freeUsed ? 1 : 0)) : []);
  // ---- per-look bits ----
  type WbTab = "problems" | "objectives" | "hints" | "console";
  let wbTab = $state<WbTab>("objectives");
  let consoleOpen = $state(true);
  const allDiags = $derived(result?.diagnostics ?? []);
  const runnerLang: Record<string, Lang> = { cargo: "rust", lua: "lua", gradle: "java" };
  const diagLang = (d: { file: string | null }): Lang => (d.file ? langOfPath(d.file) : "other") === "other" ? (runnerLang[q?.runner ?? ""] ?? "other") : langOfPath(d.file!);
  const firstError = $derived.by(() => { const d = allDiags.find((x) => x.level === "error"); return d ? explain(d, diagLang(d)) : null; });
  const worldTitle = $derived(q ? (app.catalog[q.track_id]?.find((w) => w.id === q.world_id)?.title ?? "") : "");
  const wbTabs = $derived<{ id: WbTab; label: string; n: number }[]>([
    { id: "problems", label: "Problems", n: allDiags.length },
    { id: "objectives", label: "Objectives", n: rows.length ? passedCount : 0 },
    { id: "hints", label: "Hints", n: hintsShown.length },
    ...(detail?.has_program ? [{ id: "console" as const, label: "Console", n: 0 }] : []),
  ]);
  // a new result picks the useful tab: problems when it doesn't build, objectives otherwise
  let tabSeen = 0;
  $effect(() => {
    const t = check?.tick ?? 0;
    if (t === tabSeen || !check?.result) return;
    tabSeen = t;
    untrack(() => { if (wbTab !== "hints") wbTab = check!.result!.compiled ? "objectives" : "problems"; });
  });
  // feed the Workbench status bar
  $effect(() => {
    const errs = allDiags.filter((d) => d.level === "error").length;
    const note = checking ? "Checking…" : !result ? (started ? "Save to run the checks" : "Start the quest") : result.ok ? "All green" : !result.compiled ? (firstError?.title ?? "Doesn't build") : `${passedCount}/${rows.length} objectives`;
    app.status = { errors: errs, warnings: allDiags.length - errs, note };
  });
  onDestroy(() => (app.status = null));

  const quizReady = $derived(!!q && q.quiz.length > 0 && q.quiz.every((_, i) => answers[i] !== undefined));
  const icon = { passed: "✓", failed: "✕", notrun: "•" } as const;
  const tone = { passed: "border-ok/50 bg-ok/10 text-ok", failed: "border-bad/50 bg-bad/10 text-bad", notrun: "border-line bg-panel-2 text-dim" } as const;
</script>

<svelte:window onkeydown={onKey} />

{#snippet nextBtn()}
  {#if nextHere?.quest}
    <button class="btn btn-primary !rounded-lg !px-4 !py-1.5 text-sm" onclick={() => app.goNext()} data-testid="next-quest">{nextHere.label} →</button>
  {:else if nextHere}
    <a class="btn btn-ghost !rounded-lg !px-3 !py-1.5 text-center text-sm" href="/map" data-testid="next-none">Nothing unlocked here; open the Map</a>
  {/if}
{/snippet}

{#if !q || !detail}
  <div class="mx-auto max-w-6xl px-6 py-10"><div class="h-96 animate-pulse rounded-3xl bg-panel"></div></div>
{:else if staged && parts}
{#snippet toolbar()}
    <div class="flex flex-wrap items-center gap-2" data-testid="toolbar">
      {#if done}
        {@render nextBtn()}
      {:else if app.look !== "arcade"}
        <button class="btn btn-primary !rounded-lg !px-4 !py-1.5 text-sm" disabled={busy || checking} onclick={runChecks}>Run checks</button>
      {/if}
      {#if detail!.has_program}
        <button class="btn btn-ghost !rounded-lg !px-3 !py-1.5 text-sm" disabled={busy || running} onclick={runProgram} data-testid="run">{running ? "Running…" : "▶ Run"}</button>
      {/if}
      {#if q!.hints.length}
        <button class="btn {freeHintReady ? 'btn-primary' : 'btn-ghost'} !rounded-lg !px-3 !py-1.5 text-sm" disabled={busy || hintsShown.length >= q!.hints.length} onclick={hint} data-testid="hint">
          💡 {hintsShown.length >= q!.hints.length ? "No more hints" : freeHintReady ? "Free hint" : "Hint"} <span class="text-dim">{detail!.hints_used}/{q!.hints.length}</span>
        </button>
      {/if}
      <button class="btn {focusHere ? 'btn-primary' : 'btn-ghost'} !rounded-lg !px-3 !py-1.5 font-mono text-sm" disabled={busy} onclick={focus} title="Focus timer">
        {#if focusHere}⏱ {mmss(focusHere.remaining)} · stop{:else}⏱ {app.profile?.settings.focus_minutes ?? 25}m{/if}
      </button>
      {#if app.profile?.settings.editor}
        <button class="btn btn-ghost !rounded-lg !px-3 !py-1.5 text-sm" disabled={busy} onclick={openEditor} data-testid="open-external">Open in {externalName}</button>
      {/if}
      <button class="btn btn-ghost !rounded-lg !px-3 !py-1.5 text-sm" onclick={copy} title={detail!.workspace} data-testid="copy-path">{copied ? "Copied" : "Copy path"}</button>
      <button class="btn btn-ghost !rounded-lg !px-3 !py-1.5 text-sm" disabled={busy || checking || running} onclick={reset}>Reset</button>
    </div>
{/snippet}

{#snippet lessonBlock()}
  <section bind:this={lessonEl} class="scroll-mt-20 rounded-2xl border border-line bg-panel {lessonSide ? '' : 'mb-6'}" data-testid="learn">
    <button class="flex w-full items-center gap-3 text-left {lessonSide ? 'sticky top-0 z-10 rounded-t-2xl bg-panel px-5 py-3' : 'px-6 py-3'}" aria-expanded={!lessonShut} onclick={() => setLesson(lessonShut)} data-testid="lesson-toggle">
      <span class="text-dim">{lessonShut ? "▸" : "▾"}</span>
      <h2 class="text-sm font-bold tracking-widest text-dim uppercase">Lesson</h2>
      <span class="text-sm whitespace-nowrap text-dim">· {cards?.cards.length ?? 0} {cards?.cards.length === 1 ? "card" : "cards"}{#if q?.points.length}{" · "}{q.points.length - pointsLeft}/{q.points.length} points{/if}</span>
      {#if !lessonSide}<span class="ml-auto text-sm text-rust-hi">{#if q?.points.length}+2 XP per point{/if}</span>{/if}
    </button>
    {#if !lessonShut}
      <div class="border-t border-line px-5 py-5">
        {#if cards?.why}<div class="prose-ox mb-5 rounded-xl border-l-4 border-rust bg-tint px-4 py-3 text-base text-dim" data-testid="why">{@html cards.why}</div>{/if}
        <div class="flex flex-col gap-6" data-testid="lesson-list">
          {#each steps as st}
            {#if st.kind === "card" && cards}
              {@const card = cards.cards[st.i]}
              <div data-testid="lesson-card">
                {#if card.title}<div class="mb-1 text-sm font-bold tracking-widest text-dim uppercase" data-testid="card-title">{card.title}</div>{/if}
                <div class="prose-ox point text-lg leading-snug">{@html card.html}</div>
                {#each card.codes as c, ci}<div class="mt-3"><TypedCode code={c} playKey="{id}:{st.i}:{ci}" animate={false} /></div>{/each}
              </div>
            {:else if st.kind === "point" && q}
              <div class="rounded-xl border border-dashed border-rust/50 bg-tint/60 px-4 py-4">
                <div class="mb-2 text-xs font-bold tracking-widest text-rust-hi uppercase">Quick point · +2 XP</div>
                <PointStep point={q.points[st.pi]} {lang} compact solved={!!solved[st.pi]} onsolve={() => solvePoint(st.pi)} />
              </div>
            {/if}
          {/each}
        </div>
      </div>
    {/if}
  </section>
{/snippet}

{#snippet stageBar()}
  {#if stageList.length > 1}
    <div class="flex gap-2" data-testid="stage-bar">
      {#each stageList as s, k}
        {@const here = stageList.indexOf(stage)}
        {@const isDone = k < here || (done && s === "build" && s !== stage)}
        <button
          class="flex items-center gap-2 rounded-lg border px-3 py-1 text-sm font-semibold {s === stage ? 'border-rust bg-tint text-ink' : 'border-line bg-panel text-dim hover:text-ink'}"
          onclick={() => goStage(s)} data-testid="stage" data-stage={s} data-state={s === stage ? "on" : isDone ? "done" : ""}
        >
          <span class="flex h-5 w-5 items-center justify-center rounded-full border text-xs {s === stage ? 'border-rust bg-rust text-on-rust' : isDone ? 'border-ok bg-ok text-bg' : 'border-line'}">{k + 1}</span>
          {stageNames[s]}
          <span class="font-normal text-rust-hi">{stageXp[s]}</span>
        </button>
      {/each}
    </div>
  {/if}
{/snippet}

{#snippet objPill()}
  <button
    class="flex items-center gap-2 rounded-lg border px-3 py-1 text-sm font-semibold {checking ? 'border-amber-300/40 bg-amber-300/10 text-amber-200' : !result ? 'border-line bg-panel text-dim' : passedCount === rows.length ? 'border-ok/50 bg-ok/10 text-ok' : 'border-bad/50 bg-bad/10 text-bad'}"
    onclick={() => { if (app.look === "workbench") wbTab = "objectives"; else objEl?.scrollIntoView({ behavior: "smooth", block: "start" }); }} data-testid="obj-pill" title="Objectives"
  >
    {#if checking}<span class="inline-block h-3.5 w-3.5 animate-spin rounded-full border-2 border-amber-300 border-t-transparent"></span>Checking…
    {:else}<span class="flex h-4 w-4 items-center justify-center rounded-full text-[10px] font-bold text-bg {!result ? 'bg-line' : passedCount === rows.length ? 'bg-ok' : 'bg-bad'}">{!result ? "•" : passedCount === rows.length ? "✓" : "✕"}</span>{passedCount}/{rows.length} objectives{/if}
  </button>
{/snippet}

{#snippet badges()}
  <span class="rounded-full border px-3 py-0.5 text-xs font-semibold {kindClass[q!.kind]}">{kindLabel[q!.kind]}</span>
  <span class="text-sm text-dim">{q!.minutes} min</span>
  <span class="text-sm font-bold text-rust-hi">+{q!.xp} XP</span>
  {#if done}<span class="rounded-full bg-ok/15 px-3 py-0.5 text-sm font-semibold text-ok">Cleared ✓</span>{/if}
{/snippet}

{#snippet taskBox(arcade: boolean)}
  <section class="{arcade ? '' : 'rounded-[var(--radius-box)] border border-rust/40 bg-panel p-4'}" data-testid="task">
    {#if !arcade}<h2 class="mb-2 text-xs font-bold tracking-widest text-rust-hi uppercase">Your turn</h2>{/if}
    <ul class="flex flex-col gap-2">
      {#each parts!.tasks as t, i}
        {#if ticks.includes(i) || i === firstOpen}
          <li>
            <label class="flex cursor-pointer items-start gap-3 border px-3 py-2.5 text-base {arcade ? 'rounded-xl border-2' : 'rounded-[var(--radius-box)]'} {i === firstOpen ? 'border-rust bg-tint' : 'border-line bg-panel-2'} {ticks.includes(i) ? 'opacity-60' : ''}" data-testid="task-row" data-state={i === firstOpen ? "current" : "done"}>
              <input type="checkbox" class="mt-1 h-5 w-5 shrink-0 accent-[var(--color-rust)]" checked={ticks.includes(i)} onchange={() => toggleTick(i)} data-testid="task-tick" />
              <span class="prose-ox min-w-0 {ticks.includes(i) ? 'line-through' : ''}">{@html t}</span>
            </label>
          </li>
        {/if}
      {/each}
      {#if ghostCount > 0}
        <li class="flex items-center gap-3 rounded-[var(--radius-box)] border border-dashed border-line px-3 py-2 text-base text-dim opacity-60" data-testid="task-ghost">
          <span class="h-5 w-5 shrink-0 rounded-md border-2 border-dim"></span>{ghostCount} more {ghostCount === 1 ? "step" : "steps"}
        </li>
      {/if}
    </ul>
    {#if !started}
      <button class="btn btn-primary btn-lg mt-4 w-full" disabled={busy} onclick={start}>Start quest</button>
    {/if}
  </section>
{/snippet}

{#snippet objList()}
  <ul class="flex flex-col gap-1.5" data-testid="objectives-list">
    {#each rows as r (r.test + (check?.tick ?? 0))}
      <li class="rounded-lg border {tone[r.status]} {checking ? 'opacity-60' : ''}" data-status={r.status}>
        <button class="flex w-full items-start gap-2.5 px-2.5 py-1.5 text-left text-sm {r.status === 'failed' && r.output ? 'cursor-pointer' : 'cursor-default'}" onclick={() => r.output && (expanded[r.test] = !expanded[r.test])}>
          <span class="mt-px flex h-5 w-5 shrink-0 items-center justify-center rounded-full text-xs font-bold {r.status === 'passed' ? 'tick bg-ok text-bg' : r.status === 'failed' ? 'bg-bad text-bg' : 'bg-line text-dim'}">{icon[r.status]}</span>
          <span class="flex-1 text-ink">{r.label}</span>
          {#if r.status === "failed" && r.output}<span class="text-xs text-dim">{expanded[r.test] ? "hide" : "why?"}</span>{/if}
          {#if r.status === "notrun" && result && !result.compiled}<span class="text-xs text-dim">waits for the build</span>{/if}
        </button>
        {#if expanded[r.test] && r.output}
          <pre class="mx-2.5 mb-2.5 max-h-48 overflow-auto rounded-lg bg-code p-3 font-mono text-xs leading-relaxed whitespace-pre-wrap text-ink">{r.output}</pre>
        {/if}
      </li>
    {/each}
  </ul>
{/snippet}

{#snippet resultLine()}
  {#if checking}
    <span class="flex items-center gap-2 text-amber-200" data-testid="checking"><span class="inline-block h-3.5 w-3.5 animate-spin rounded-full border-2 border-amber-300 border-t-transparent"></span>Checking…</span>
  {:else if result}
    <span class="text-dim">{result.ok ? "All green" : result.compiled ? "Some tests failing" : q!.runner === "lua" ? "Script error" : "Doesn't build yet"} · {(result.duration_ms / 1000).toFixed(1)}s</span>
  {:else}
    <span class="text-dim" data-testid="help">Save (Ctrl+S) and checks run.</span>
  {/if}
{/snippet}

{#snippet problems()}
  {#if allDiags.length}
    <ul class="flex flex-col" data-testid="diag-list">
      {#each allDiags as d}
        {@const x = explain(d, diagLang(d))}
        <li class="flex items-baseline gap-3 px-1 py-1.5 text-sm" data-testid="diagnostic">
          <span class="font-bold {d.level === 'error' ? 'text-bad' : 'text-amber-300'}">{d.level === "error" ? "✕" : "▲"}</span>
          <span class="min-w-0 flex-1"><b class="font-semibold">{x.title}.</b> <span class="text-dim">{d.message}</span></span>
          <span class="shrink-0 font-mono text-xs text-dim">{d.file ?? "compiler"}{d.line ? `:${d.line}` : ""}</span>
        </li>
      {/each}
    </ul>
  {:else if result}
    <p class="px-1 py-2 text-sm text-dim">{result.compiled ? "No compiler problems." : "The build failed; see the compiler output in the editor."}</p>
  {:else}
    <p class="px-1 py-2 text-sm text-dim">Nothing yet. Save and the compiler looks at your code.</p>
  {/if}
{/snippet}

{#snippet hintBox(note: boolean)}
  {#if q!.hints.length}
    {#if freeHintReady}
      <div class="pop mb-2 rounded-[var(--radius-box)] border border-rust bg-tint px-3 py-2 text-sm" data-testid="stuck-box">
        Stuck for a bit? The next hint is free: it won't count, so your no-hint badge and combo are safe.
      </div>
    {/if}
    <ol class="flex flex-col gap-2">
      {#each q!.hints as h, i}
        {@const open = i < hintsShown.length}
        <li class="rounded-[var(--radius-box)] border px-3 py-2 text-sm {open ? 'pop border-amber-300/60 bg-amber-300/10' : 'border-line bg-panel-2 text-dim'}">
          <div class="flex justify-between text-xs font-bold tracking-wider uppercase {open ? 'text-amber-300' : ''}"><span>{note ? `Hint ${i + 1} of ${q!.hints.length}` : ["Nudge", "Clue", "Show the fix"][i] ?? `Hint ${i + 1}`}</span></div>
          {#if open}<div class="mt-1 text-ink">{@html renderMarkdown(h.text, lang).replace(/^<p>|<\/p>\s*$/g, "")}</div>{/if}
        </li>
      {/each}
    </ol>
    <button class="btn {freeHintReady ? 'btn-primary' : 'btn-ghost'} mt-2 w-full !rounded-lg !py-1.5 text-sm" disabled={busy || hintsShown.length >= q!.hints.length} onclick={hint} data-testid="hint">
      {hintsShown.length >= q!.hints.length ? "No more hints" : freeHintReady ? "Show next hint (free)" : "Show next hint"} <span class="text-dim">{detail!.hints_used}/{q!.hints.length}</span>
    </button>
  {:else}
    <p class="text-sm text-dim">This quest has no hints. Hover a word in the code for a short note, or press Ctrl+Space for ideas.</p>
  {/if}
{/snippet}

{#snippet consoleBody(tall: boolean)}
  <div bind:this={consoleEl} class="{tall ? 'min-h-0 flex-1' : 'max-h-80 min-h-32'} overflow-auto px-4 py-3 font-mono text-sm leading-relaxed" data-testid="console-lines">
    {#each consoleRows as r}
      {#if r.kind === "build"}
        <button class="block w-full text-left text-dim/80 hover:text-dim" data-testid="build-fold" onclick={() => (openBuild[r.at] = !openBuild[r.at])}>
          {openBuild[r.at] ? "▾" : "▸"} build output ({r.lines.length} {r.lines.length === 1 ? "line" : "lines"})
        </button>
        {#if openBuild[r.at]}
          {#each r.lines as bl}<div class="whitespace-pre-wrap break-words pl-4 text-dim/70">{bl}</div>{/each}
        {/if}
      {:else}
        <div class="whitespace-pre-wrap break-words {lineClass[r.l.stream]}">{r.l.stream === "input" ? "› " : ""}{r.l.line}</div>
      {/if}
    {:else}
      <div class="text-dim">Press ▶ Run to start the program.</div>
    {/each}
  </div>
  <form class="flex items-center gap-2 border-t border-line px-4 py-2 font-mono text-sm" onsubmit={(e) => { e.preventDefault(); if (running) sendInput(); }}>
    <span class="text-ok">›</span>
    <input class="min-w-0 flex-1 bg-transparent text-ink outline-none placeholder:text-dim/50 disabled:opacity-40" placeholder={running ? "type a line, press Enter" : "program not running"} disabled={!running} bind:value={stdin} data-testid="stdin" autocomplete="off" spellcheck="false" />
    {#if running}<button type="button" class="btn btn-ghost !rounded-lg !px-3 !py-0.5 font-sans text-xs" onclick={stopProgram} data-testid="stop">Stop</button>{/if}
  </form>
{/snippet}

{#snippet consoleCard()}
  <section bind:this={panelEl} class="mt-6 overflow-hidden rounded-[var(--radius-box)] border border-line bg-code" data-testid="console">
    <header class="flex items-center justify-between border-b border-line px-4 py-2">
      <h2 class="text-sm font-bold text-dim">Console{#if running}<span class="ml-2 inline-block h-2 w-2 animate-pulse rounded-full bg-ok"></span>{/if}</h2>
    </header>
    {@render consoleBody(false)}
  </section>
{/snippet}

{#snippet editor(fill: boolean, list: boolean)}
  {#key `${detail!.workspace}|${resetN}`}
    <CodeEditor {id} diagnostics={result?.diagnostics ?? []} {checking} todo={todoIdx} tasks={parts!.tasks.length} hot={stuckNow} onsave={onEditorSave} onlesson={hasLearn && lessonShut ? showLesson : undefined} {fill} {list} />
  {/key}
{/snippet}

{#snippet checkStage()}
  <div class="max-w-[68rem]">
    <div class="rounded-[var(--radius-box)] border border-line bg-panel px-8 py-7" data-testid="check-stage">
      {#if cq}
        <p class="mb-5 text-2xl leading-snug font-semibold" data-testid="check-q">{cq.question}</p>
        {#each cq.choices as c, ci}
          <button
            class="mb-2.5 flex w-full items-center gap-4 rounded-xl border px-5 py-3.5 text-left text-lg {chkRight === ci ? 'border-ok bg-ok/10' : chkWrong.includes(ci) ? 'border-bad bg-bad/10' : 'border-line bg-panel-2 hover:border-rust'}"
            onclick={() => pickAnswer(ci)} data-testid="choice" data-state={chkRight === ci ? "right" : chkWrong.includes(ci) ? "wrong" : ""}
          >
            <span class="flex h-7 w-7 shrink-0 items-center justify-center rounded-lg border border-line text-sm font-bold text-dim">{"ABCDEFGH"[ci]}</span>{c}
          </button>
        {/each}
        {#if chkWrong.length && chkRight === null}<p class="mt-3 text-dim">Not that one. Guessing is free.</p>{/if}
      {/if}
      <div class="mt-6 flex items-center gap-4">
        <span class="flex-1"></span>
        <button class="btn btn-primary" disabled={chkRight === null} onclick={afterCheck} data-testid="check-next">{q && chkQ < q.quiz.length - 1 ? "Next question" : "Build it →"}</button>
      </div>
    </div>
  </div>
{/snippet}

{#if stage === "check"}
  <div class="mx-auto max-w-5xl px-6 py-8">
    <header class="mb-6 flex flex-wrap items-center gap-x-4 gap-y-2" data-testid="quest-head">
      <h1 class="font-[family-name:var(--font-display)] text-3xl font-extrabold">{q.title}</h1>
      {@render badges()}
      <div class="ml-auto">{@render stageBar()}</div>
    </header>
    {@render checkStage()}
  </div>
{:else if app.look === "workbench"}
  <!-- Workbench: lesson on the left, editor in the middle, panel with tabs below -->
  <div class="grid h-full min-h-0 grid-cols-[minmax(17rem,25rem)_minmax(0,1fr)]" data-layout="workbench">
    <aside class="flex min-h-0 flex-col overflow-y-auto border-r border-line bg-panel" data-testid="lesson-col">
      <header class="flex flex-col gap-2 border-b border-line px-4 py-3" data-testid="quest-head">
        <h1 class="text-lg leading-tight font-bold">{q.title}</h1>
        <div class="flex flex-wrap items-center gap-2">{@render badges()}</div>
        {@render stageBar()}
      </header>
      <div class="p-3">{@render taskBox(false)}</div>
      {#if hasLearn}<div class="px-3 pb-3">{@render lessonBlock()}</div>{:else if cards?.why}<div class="prose-ox mx-3 mb-3 rounded-xl border-l-4 border-rust bg-tint px-4 py-3 text-base text-dim" data-testid="why">{@html cards.why}</div>{/if}
    </aside>
    <div class="flex min-h-0 min-w-0 flex-col">
      {#if started}
        <div class="flex flex-wrap items-center gap-2 border-b border-line bg-panel px-3 py-1.5">
          {@render objPill()}
          <span class="flex-1"></span>
          {@render toolbar()}
        </div>
        <div class="min-h-0 flex-1 p-2">{@render editor(true, false)}</div>
        <section class="flex h-60 shrink-0 flex-col border-t border-line bg-panel {flash ? '!bg-white' : ''}" bind:this={objEl} data-testid="objectives">
          <div class="flex items-center gap-5 border-b border-line px-4 text-xs font-bold tracking-wider uppercase" role="tablist">
            {#each wbTabs as tb}
              <button role="tab" aria-selected={wbTab === tb.id} class="py-2 {wbTab === tb.id ? 'text-ink shadow-[inset_0_-2px_0_var(--color-rust)]' : 'text-dim hover:text-ink'}" onclick={() => (wbTab = tb.id)} data-testid="panel-tab-{tb.id}">
                {tb.label}{#if tb.n}<span class="ml-1.5 rounded-full px-1.5 text-[10px] {tb.id === 'problems' ? 'bg-bad text-bg' : 'bg-panel-2 text-dim'}">{tb.n}</span>{/if}
              </button>
            {/each}
            <span class="ml-auto text-xs font-normal tracking-normal normal-case">{@render resultLine()}</span>
          </div>
          <div class="min-h-0 flex-1 {wbTab === 'console' ? 'flex flex-col' : 'overflow-y-auto px-4 py-3'}">
            {#if wbTab === "problems"}{@render problems()}
            {:else if wbTab === "objectives"}{@render objList()}
            {:else if wbTab === "hints"}<div class="max-w-2xl">{@render hintBox(false)}</div>
            {:else}<div bind:this={panelEl} class="flex min-h-0 flex-1 flex-col bg-code" data-testid="console">{@render consoleBody(true)}</div>{/if}
          </div>
        </section>
      {:else}
        <div class="grid flex-1 place-items-center p-8 text-center text-dim">
          <div><div class="mb-2 text-4xl">✎</div>Start the quest on the left and your code opens here.</div>
        </div>
      {/if}
    </div>
  </div>
{:else if app.look === "notebook"}
  <!-- Notebook: one calm column, hints in the margin -->
  <div class="mx-auto grid max-w-[78rem] gap-10 px-8 pt-10 pb-24 lg:grid-cols-[minmax(0,48rem)_17rem]" data-layout="notebook">
    <article class="min-w-0">
      <header class="mb-6" data-testid="quest-head">
        <div class="mb-1 text-xs font-bold tracking-[0.14em] text-rust uppercase">{worldTitle} · {q.minutes} min · +{q.xp} XP</div>
        <h1 class="mb-3 font-[family-name:var(--font-display)] text-5xl leading-tight font-bold">{q.title}</h1>
        <div class="flex flex-wrap items-center gap-3">
          <span class="rounded-full border px-3 py-0.5 text-xs font-semibold {kindClass[q.kind]}">{kindLabel[q.kind]}</span>
          {#if done}<span class="rounded-full bg-ok/15 px-3 py-0.5 text-sm font-semibold text-ok">Cleared ✓</span>{/if}
          {#if started}{@render objPill()}{/if}
          <span class="ml-auto">{@render stageBar()}</span>
        </div>
      </header>
      {#if hasLearn}<div class="mb-6">{@render lessonBlock()}</div>{:else if cards?.why}<div class="prose-ox mb-6 rounded-xl border-l-4 border-rust bg-tint px-5 py-3 text-lg text-dim" data-testid="why">{@html cards.why}</div>{/if}
      <div class="mb-6">{@render taskBox(false)}</div>
      {#if started}
        <div class="mb-2 text-xs font-bold tracking-[0.14em] text-dim uppercase">Your code</div>
        <div class="shadow-[0_14px_34px_-20px_rgba(60,40,20,.45)]">{@render editor(false, true)}</div>
        <section bind:this={objEl} class="mt-3 rounded-[var(--radius-box)] border p-4 {flash ? 'border-white bg-white' : result && !result.ok ? 'border-bad/40 bg-bad/5' : result?.ok ? 'border-ok/40 bg-ok/5' : 'border-line bg-panel'}" data-testid="objectives">
          <div class="mb-2 flex items-center gap-2 text-sm"><span class="font-bold">Checks</span><span class="text-dim tabular-nums">{passedCount}/{rows.length}</span><span class="ml-auto text-xs">{@render resultLine()}</span></div>
          {@render objList()}
        </section>
        <div class="mt-4">{@render toolbar()}</div>
        {#if q.flavour === "games" && rows.length}<div class="mt-6"><QuestScene quest={q} objectives={sceneRows} /></div>{/if}
      {/if}
      {#if run}{@render consoleCard()}{/if}
    </article>
    <aside class="flex flex-col gap-4 lg:sticky lg:top-6 lg:self-start" data-testid="margin">
      <div class="rounded-[var(--radius-box)] border border-[#ecd7a6] bg-[#fdf4dc] p-4">
        <div class="mb-2 text-xs font-bold tracking-[0.12em] text-amber-300 uppercase">Hints</div>
        {@render hintBox(true)}
      </div>
      <div class="rounded-[var(--radius-box)] border border-line bg-panel p-4 text-sm">
        <div class="mb-1 text-xs font-bold tracking-[0.12em] text-dim uppercase">While you type</div>
        Press <kbd class="rounded border border-line px-1 font-mono text-xs">Ctrl</kbd>+<kbd class="rounded border border-line px-1 font-mono text-xs">Space</kbd> for ideas. Hover a word to see what it does. Red lines get a note in plain words underneath.
      </div>
      <button class="btn {focusHere ? 'btn-primary' : 'btn-ghost'} w-full font-mono text-sm" disabled={busy} onclick={focus}>
        {#if focusHere}⏱ {mmss(focusHere.remaining)} · stop{:else}⏱ Focus {app.profile?.settings.focus_minutes ?? 25} min{/if}
      </button>
    </aside>
  </div>
{:else}
  <!-- Arcade: quest log, editor with a big RUN, Ferris helps, challenges below -->
  <div class="grid h-full min-h-0 grid-cols-[19rem_minmax(0,1fr)_17rem] grid-rows-[minmax(0,1fr)_auto] gap-3 p-3" data-layout="arcade">
    <section class="flex min-h-0 flex-col overflow-hidden rounded-[var(--radius-box)] border-2 border-line bg-panel" data-testid="lesson-col">
      <div class="flex items-center justify-between border-b-2 border-line px-4 py-2.5 text-xs font-bold tracking-widest text-dim uppercase"><span>Quest log</span><span>{worldTitle}</span></div>
      <div class="min-h-0 flex-1 overflow-y-auto">
        <div class="p-3">{@render taskBox(true)}</div>
        {#if stageList.length > 1}<div class="px-3 pb-3">{@render stageBar()}</div>{/if}
        {#if hasLearn}<div class="border-t-2 border-line p-3">{@render lessonBlock()}</div>{/if}
      </div>
    </section>
    <section class="flex min-h-0 min-w-0 flex-col gap-2.5">
      <header class="flex flex-wrap items-center gap-3" data-testid="quest-head">
        <h1 class="text-2xl font-bold">{q.title}</h1>
        {@render badges()}
        <span class="flex-1"></span>
        {#if started && !done}
          <button class="rounded-xl bg-rust px-5 py-2 font-extrabold text-on-rust shadow-[0_4px_0_color-mix(in_srgb,var(--color-rust)_55%,black)] active:translate-y-0.5 disabled:opacity-50" disabled={busy || checking} onclick={runChecks} data-testid="run-checks">▶ RUN <span class="font-normal opacity-70">Ctrl+S</span></button>
        {/if}
      </header>
      {#if started}
        <div class="min-h-0 flex-1">{@render editor(true, false)}</div>
        <div class="flex flex-wrap items-center gap-2">{@render toolbar()}</div>
      {:else}
        <div class="grid flex-1 place-items-center rounded-[var(--radius-box)] border-2 border-dashed border-line text-center text-dim">
          <div><div class="mb-2 text-4xl">🗺️</div>Accept the quest in the log to open your code.</div>
        </div>
      {/if}
    </section>
    <aside class="flex min-h-0 flex-col overflow-hidden rounded-[var(--radius-box)] border-2 border-line bg-panel" data-testid="companion">
      <div class="border-b-2 border-line px-4 py-2.5 text-xs font-bold tracking-widest text-dim uppercase">Ferris</div>
      <div class="flex min-h-0 flex-1 flex-col gap-3 overflow-y-auto p-3">
        <svg width="58" height="42" viewBox="0 0 56 44" aria-hidden="true" class="{checking ? 'animate-bounce' : ''}"><ellipse cx="28" cy="28" rx="22" ry="13" fill="var(--color-rust)"/><circle cx="21" cy="17" r="5" fill="#fff"/><circle cx="35" cy="17" r="5" fill="#fff"/><circle cx="22" cy="18" r="2.3" fill="#111"/><circle cx="36" cy="18" r="2.3" fill="#111"/><path d="M6 24 2 14M50 24l4-10" stroke="var(--color-rust)" stroke-width="4" stroke-linecap="round"/></svg>
        <div class="rounded-[14px_14px_14px_4px] border-2 border-line bg-panel-2 px-3 py-2.5 text-sm leading-relaxed" data-testid="ferris-says">
          {#if checking}Checking your code…
          {:else if done}Quest cleared! {#if nextHere?.quest}Ready for the next one?{/if}
          {:else if firstError}<b class="text-amber-300">{firstError.title.replace(/`/g, "")}.</b> {firstError.body.replace(/`/g, "")}
          {:else if result && !result.ok}{passedCount} of {rows.length} passing. Open a red challenge below to see why.
          {:else if result?.ok}All green. Nice work!
          {:else if started}Edit the code, then hit RUN (or just save). I'll tell you what the compiler says.
          {:else}New quest! Read the log on the left, then accept it.{/if}
        </div>
        {@render hintBox(false)}
        <button class="btn {focusHere ? 'btn-primary' : 'btn-ghost'} w-full font-mono text-sm" disabled={busy} onclick={focus}>
          {#if focusHere}⏱ {mmss(focusHere.remaining)} · stop{:else}⏱ Focus {app.profile?.settings.focus_minutes ?? 25} min{/if}
        </button>
      </div>
    </aside>
    <div class="col-span-3 grid grid-cols-[19rem_minmax(0,1fr)_17rem] gap-3">
      <div class="min-w-0">{#if q.flavour === "games" && rows.length && started}<QuestScene quest={q} objectives={sceneRows} />{/if}</div>
      <section bind:this={objEl} class="rounded-[var(--radius-box)] border-2 p-3 {flash ? 'border-white bg-white' : 'border-line bg-panel'}" data-testid="objectives">
        <div class="mb-2 flex items-center gap-2 text-xs font-bold tracking-widest text-dim uppercase"><span>Challenges {passedCount}/{rows.length}</span><span class="ml-auto font-normal tracking-normal normal-case">{@render resultLine()}</span></div>
        {#if result && !result.compiled}
          <div class="mb-2 rounded-xl border-2 border-bad bg-bad/10 px-3 py-1.5 text-sm"><b class="text-bad">✕ Build</b> · {allDiags.filter((d) => d.level === "error").length} error{allDiags.filter((d) => d.level === "error").length === 1 ? "" : "s"}{#if allDiags[0]?.line}, line {allDiags[0].line}{/if}</div>
        {/if}
        {@render objList()}
      </section>
      <div class="flex flex-col justify-center rounded-[var(--radius-box)] border-2 border-line bg-panel px-4 py-3">
        <div class="text-xs font-bold tracking-widest text-dim uppercase">Reward</div>
        <div class="text-2xl font-bold">+{q.xp} XP</div>
        {#if run}<button class="mt-2 text-left text-sm text-rust-hi underline" onclick={() => (consoleOpen = !consoleOpen)}>{consoleOpen ? "Hide" : "Show"} console</button>{/if}
      </div>
    </div>
  </div>
  {#if run && consoleOpen}
    <div class="fixed right-6 bottom-6 z-40 flex h-80 w-[36rem] max-w-[calc(100vw-3rem)] flex-col overflow-hidden rounded-[var(--radius-box)] border-2 border-line bg-code shadow-2xl" bind:this={panelEl} data-testid="console">
      <header class="flex items-center justify-between border-b-2 border-line px-4 py-2 text-sm font-bold text-dim">Console{#if running}<span class="ml-2 inline-block h-2 w-2 animate-pulse rounded-full bg-ok"></span>{/if}<button class="ml-auto text-dim hover:text-ink" onclick={() => (consoleOpen = false)} aria-label="Close console">✕</button></header>
      {@render consoleBody(true)}
    </div>
  {/if}
{/if}

  {#if xpToast}
    {#key xpToast.key}<div class="pop fixed top-20 right-7 z-50 rounded-xl border border-rust bg-tint px-4 py-2 font-extrabold text-rust-hi" data-testid="xp-toast">+{xpToast.n} XP</div>{/key}
  {/if}
{:else}
  <div class="mx-auto grid max-w-[96rem] gap-8 px-6 py-8 lg:grid-cols-[minmax(0,1fr)_24rem]">
    <!-- lesson -->
    <article class="min-w-0">
      <div class="mb-3 flex flex-wrap items-center gap-3 text-sm">
        <span class="rounded-full border px-3 py-0.5 font-semibold {kindClass[q.kind]}">{kindLabel[q.kind]}</span>
        <span class="text-dim">{q.minutes} min</span>
        <span class="font-bold text-rust-hi">+{q.xp} XP</span>
        {#if done}<span class="rounded-full bg-ok/15 px-3 py-0.5 font-semibold text-ok">Cleared ✓</span>{/if}
      </div>
      <h1 class="mb-6 text-4xl font-extrabold">{q.title}</h1>
      {#if parts}
        <section class="mb-6 rounded-2xl border border-rust/40 bg-panel p-5" data-testid="task">
          <h2 class="mb-3 text-sm font-bold tracking-widest text-rust-hi uppercase">Your turn</h2>
          <ul class="flex flex-col gap-2">
            {#each parts.tasks as t, i}
              <li>
                <label class="flex cursor-pointer items-start gap-3 rounded-xl border px-4 py-3 text-lg {i === firstOpen ? 'border-rust bg-rust/10' : 'border-line bg-panel-2'} {ticks.includes(i) ? 'opacity-60' : ''}">
                  <input type="checkbox" class="mt-1.5 h-5 w-5 shrink-0 accent-[var(--color-rust)]" checked={ticks.includes(i)} onchange={() => toggleTick(i)} data-testid="task-tick" />
                  <span class="prose-ox min-w-0 {ticks.includes(i) ? 'line-through' : ''}">{@html t}</span>
                </label>
              </li>
            {/each}
          </ul>
        </section>
        {#if parts.code}
          <div class="group relative mb-6">
            <div class="prose-ox big-code">{@html parts.code}</div>
            <button class="absolute top-2 right-2 rounded-md border border-line bg-panel-2 px-2.5 py-1 text-xs text-dim hover:text-ink" onclick={copyCode} data-testid="copy-code">{codeCopied ? "Copied" : "Copy"}</button>
          </div>
        {/if}
        {#if parts.why}<div class="prose-ox mb-6 rounded-xl border-l-4 border-line bg-panel/60 px-4 py-3 text-base text-dim" data-testid="why">{@html parts.why}</div>{/if}
        {#if parts.idea}
          <section class="rounded-2xl border border-line bg-panel" data-testid="idea-section">
            <button class="flex w-full items-center gap-3 px-5 py-3 text-left font-semibold" aria-expanded={ideaOpen} onclick={() => setIdea(!ideaOpen)} data-testid="idea-toggle">
              <span class="text-dim">{ideaOpen ? "▾" : "▸"}</span>
              <span>The idea</span>
              <span class="text-sm font-normal text-dim">· {parts.points} {parts.points === 1 ? "point" : "points"} · ~{parts.minutes} min</span>
            </button>
            {#if ideaOpen}<div class="prose-ox border-t border-line px-5 py-4 text-lg" data-testid="idea">{@html parts.idea}</div>{/if}
          </section>
        {/if}
        {#if started}
          <div class="mt-6">
            {#key `${detail.workspace}|${resetN}`}
              <CodeEditor {id} diagnostics={result?.diagnostics ?? []} {checking} />
            {/key}
          </div>
        {/if}
      {:else}
        {#if q.kind === "read"}<div class="mb-4 text-sm text-dim" data-testid="read-time">~{readMin} min read</div>{/if}
        <div class="prose-ox text-lg">{@html lesson}</div>
      {/if}

      {#if run}
        <section bind:this={panelEl} class="mt-10 overflow-hidden rounded-2xl border border-line bg-code" data-testid="console">
          <header class="flex items-center justify-between border-b border-line px-4 py-2">
            <h2 class="text-sm font-bold text-dim">Console{#if running}<span class="ml-2 inline-block h-2 w-2 animate-pulse rounded-full bg-ok"></span>{/if}</h2>
            <button class="btn btn-ghost !rounded-lg !px-3 !py-1 text-sm" disabled={!running} onclick={stopProgram} data-testid="stop">Stop</button>
          </header>
          <div bind:this={consoleEl} class="max-h-80 min-h-32 overflow-auto px-4 py-3 font-mono text-sm leading-relaxed" data-testid="console-lines">
            {#each consoleRows as r}
              {#if r.kind === "build"}
                <button class="block w-full text-left text-dim/80 hover:text-dim" data-testid="build-fold" onclick={() => (openBuild[r.at] = !openBuild[r.at])}>
                  {openBuild[r.at] ? "▾" : "▸"} build output ({r.lines.length} {r.lines.length === 1 ? "line" : "lines"})
                </button>
                {#if openBuild[r.at]}
                  {#each r.lines as bl}<div class="whitespace-pre-wrap break-words pl-4 text-dim/70">{bl}</div>{/each}
                {/if}
              {:else}
                <div class="whitespace-pre-wrap break-words {lineClass[r.l.stream]}">{r.l.stream === "input" ? "› " : ""}{r.l.line}</div>
              {/if}
            {/each}
          </div>
          <form class="flex items-center gap-2 border-t border-line px-4 py-2 font-mono text-sm" onsubmit={(e) => { e.preventDefault(); if (running) sendInput(); }}>
            <span class="text-ok">›</span>
            <input class="min-w-0 flex-1 bg-transparent text-ink outline-none placeholder:text-dim/50 disabled:opacity-40" placeholder={running ? "type a line, press Enter" : "program not running"} disabled={!running} bind:value={stdin} data-testid="stdin" autocomplete="off" spellcheck="false" />
          </form>
        </section>
      {/if}
    </article>

    <!-- side panel -->
    <aside class="flex flex-col gap-4 lg:sticky lg:top-6 lg:self-start">
      {#if q.kind === "read"}
        <section class="rounded-2xl border border-line bg-panel p-5" data-testid="quiz">
          <h2 class="mb-4 text-lg font-bold">Quick check</h2>
          {#each q.quiz as qz, qi}
            <fieldset class="mb-5">
              <legend class="mb-2 font-semibold">{qz.question}</legend>
              {#each qz.choices as c, ci}
                <label class="mb-1.5 flex cursor-pointer items-center gap-3 rounded-xl border px-3 py-2 {answers[qi] === ci ? 'border-rust bg-rust/10' : 'border-line hover:border-dim'}">
                  <input type="radio" name="q{qi}" class="accent-[var(--color-rust)]" checked={answers[qi] === ci} onchange={() => { answers[qi] = ci; quizMsg = ""; }} />
                  <span>{c}</span>
                </label>
              {/each}
            </fieldset>
          {/each}
          {#if quizMsg}<p class="mb-3 text-bad">{quizMsg}</p>{/if}
          {#if done}
            <p class="mb-3 font-semibold text-ok">Cleared ✓</p>
            {@render nextBtn()}
          {:else}
            <button class="btn btn-primary w-full" disabled={busy || !quizReady} onclick={checkQuiz}>Check</button>
          {/if}
        </section>
      {:else}
        <section class="rounded-2xl border border-line bg-panel p-5">
          {#if !started}
            <button class="btn btn-primary btn-lg w-full" disabled={busy} onclick={start}>Start quest</button>
          {:else}
            <div class="flex flex-col gap-2">
              
              <div class="flex gap-2">
                {#if done}
                  {@render nextBtn()}
                {:else}
                  <button class="btn btn-ghost flex-1" disabled={busy || checking} onclick={runChecks}>Run checks</button>
                {/if}
                {#if detail.has_program}
                  <button class="btn btn-ghost flex-1" disabled={busy || running} onclick={runProgram} data-testid="run">{running ? "Running…" : "Run"}</button>
                {/if}
                <button class="btn btn-ghost" disabled={busy || checking || running} onclick={reset}>Reset</button>
              </div>
              {#if app.profile?.settings.editor}
                <button class="btn btn-ghost w-full" disabled={busy} onclick={openEditor} data-testid="open-external">Open in {externalName}</button>
              {/if}
            </div>
            <div class="mt-3 flex items-center gap-2 text-xs text-dim">
              <code class="min-w-0 flex-1 truncate font-mono" title={detail.workspace}>{detail.workspace}</code>
              <button class="rounded-md border border-line px-2 py-0.5 hover:text-ink" onclick={copy}>{copied ? "Copied" : "Copy"}</button>
            </div>
          {/if}
        </section>

        {#if q.flavour === "games" && rows.length}
          <QuestScene quest={q} objectives={sceneRows} />
        {/if}

        {#if started}
          <section class="rounded-2xl border p-5 {flash ? 'border-white bg-white' : 'border-line bg-panel'}" data-testid="objectives">
            <div class="mb-3 flex items-center justify-between">
              <h2 class="text-lg font-bold">Objectives</h2>
              <span class="text-sm text-dim tabular-nums">{passedCount}/{rows.length}</span>
            </div>

            {#if checking}
              <div class="mb-3 flex items-center gap-3 rounded-xl border border-amber-300/40 bg-amber-300/10 px-4 py-2 text-amber-200" data-testid="checking">
                <span class="inline-block h-4 w-4 animate-spin rounded-full border-2 border-amber-300 border-t-transparent"></span>
                Checking…
              </div>
            {/if}

            {#each errors as d}
              <div class="mb-3 rounded-xl border border-bad/50 bg-bad/10 p-3" data-testid="diagnostic">
                <div class="mb-2 font-mono text-sm font-bold {d.level === 'error' ? 'text-bad' : 'text-amber-300'}">
                  {d.file ?? "compiler"}{d.line ? `:${d.line}` : ""} <span class="font-sans font-normal text-dim">{d.message}</span>
                </div>
                <pre class="max-h-56 overflow-auto rounded-lg bg-code p-3 font-mono text-xs leading-relaxed whitespace-pre-wrap">{d.rendered}</pre>
              </div>
            {/each}

            <ul class="flex flex-col gap-2">
              {#each rows as r (r.test + (check?.tick ?? 0))}
                <li class="rounded-xl border {tone[r.status]} {checking ? 'opacity-60' : ''}" data-status={r.status}>
                  <button
                    class="flex w-full items-start gap-3 px-3 py-2.5 text-left {r.status === 'failed' && r.output ? 'cursor-pointer' : 'cursor-default'}"
                    onclick={() => r.output && (expanded[r.test] = !expanded[r.test])}
                  >
                    <span class="mt-0.5 flex h-6 w-6 shrink-0 items-center justify-center rounded-full text-sm font-bold {r.status === 'passed' ? 'tick bg-ok text-bg' : r.status === 'failed' ? 'bg-bad text-bg' : 'bg-line text-dim'}">{icon[r.status]}</span>
                    <span class="flex-1 text-ink">{r.label}</span>
                    {#if r.status === "failed" && r.output}<span class="text-xs text-dim">{expanded[r.test] ? "hide" : "why?"}</span>{/if}
                  </button>
                  {#if expanded[r.test] && r.output}
                    <pre class="mx-3 mb-3 max-h-48 overflow-auto rounded-lg bg-code p-3 font-mono text-xs leading-relaxed whitespace-pre-wrap text-ink">{r.output}</pre>
                  {/if}
                </li>
              {/each}
            </ul>

            {#if result}
              <div class="mt-3 text-xs text-dim">
                {result.ok ? "All green" : result.compiled ? "Some tests failing" : q.runner === "lua" ? "Script error" : "Doesn't compile"} · {(result.duration_ms / 1000).toFixed(1)}s
              </div>
            {/if}
            <div class="mt-3 text-xs text-dim" data-testid="help">{q.runner === "lua" ? "Edit src/*.lua, tests.lua is the spec. Save (Ctrl+S) and checks run automatically." : "Edit the code on the left, save (Ctrl+S), and checks run automatically."}</div>
          </section>
        {/if}
      {/if}

      <!-- hints + focus -->
      <section class="rounded-2xl border border-line bg-panel p-5">
        {#if q.hints.length}
          <div class="mb-2 flex items-center justify-between">
            <h2 class="text-lg font-bold">Hints</h2>
            <span class="text-sm text-dim tabular-nums">{detail.hints_used} of {q.hints.length} used</span>
          </div>
          <ol class="mb-3 flex flex-col gap-2">
            {#each hintsShown as h, i}
              <li class="pop rounded-xl border border-line bg-panel-2 px-3 py-2 text-sm"><span class="mr-1 font-bold text-rust-hi">{i + 1}.</span>{@html renderMarkdown(h.text, lang).replace(/^<p>|<\/p>\s*$/g, "")}</li>
            {/each}
          </ol>
          {#if freeHintReady}
            <div class="pop mb-3 rounded-xl border border-rust bg-rust/10 px-4 py-3 text-sm" data-testid="stuck-box">
              Stuck for a bit? Hint 1 costs nothing now. It will not count as a used hint, so your no-hint badge and combo are safe.
            </div>
          {/if}
          <button class="btn btn-ghost w-full" disabled={busy || hintsShown.length >= q.hints.length} onclick={hint} data-testid="hint">
            {hintsShown.length >= q.hints.length ? "No more hints" : freeHintReady ? "Show next hint (this one is free)" : "Show next hint"}
          </button>
        {/if}
        <button class="btn {focusHere ? 'btn-primary' : 'btn-ghost'} mt-3 w-full font-mono" disabled={busy} onclick={focus}>
          {#if focusHere}⏱ {mmss(focusHere.remaining)} · stop{:else}⏱ Focus {app.profile?.settings.focus_minutes ?? 25} min{/if}
        </button>
      </section>
    </aside>
  </div>
{/if}

<style>
  .big-code :global(pre) { padding: 1.25rem 1.4rem; }
  .big-code :global(pre code) { font-size: 1.2rem; line-height: 1.65; }
  .point :global(strong) { color: var(--color-rust-hi); }
  .tick { animation: tick 0.35s ease-out both; }
  @keyframes tick { from { transform: scale(0.2); opacity: 0 } 60% { transform: scale(1.3) } to { transform: scale(1); opacity: 1 } }
</style>
