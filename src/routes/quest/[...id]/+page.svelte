<script lang="ts">
  import { page } from "$app/state";
  import { api } from "$lib/api";
  import { app } from "$lib/store.svelte";
  import { play } from "$lib/sound";
  import { lessonCards, readingMinutes, renderMarkdown, splitLesson } from "$lib/markdown";
  import QuestScene from "$lib/components/QuestScene.svelte";
  import CodeEditor from "$lib/components/CodeEditor.svelte";
  import TypedCode from "$lib/components/TypedCode.svelte";
  import PointStep from "$lib/components/PointStep.svelte";
  import { kindClass, kindLabel, mmss } from "$lib/ui";
  import { tick, untrack } from "svelte";
  import type { CheckResult, ObjectiveResult, QuestDetail } from "$lib/types";

  type Stage = "learn" | "check" | "build";
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
    step = 0;
    lessonOpen = false;
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
  const lesson = $derived(detail ? renderMarkdown(detail.lesson) : "");
  const parts = $derived(detail && detail.quest.kind !== "read" ? splitLesson(detail.lesson) : null);
  const readMin = $derived(detail ? readingMinutes(detail.lesson) : 1);
  const cards = $derived(detail && detail.quest.kind !== "read" ? lessonCards(detail.lesson) : null);
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

  // ---- stages: Learn / Check / Build ----
  let stage = $state<Stage>("learn");
  let step = $state(0);
  let lessonOpen = $state(false);
  let seen = $state<number[]>([]);
  let solved = $state<Record<number, boolean>>({});
  let returning = $state(false);
  let xpToast = $state<{ n: number; key: number } | null>(null);
  let chkQ = $state(0);
  let chkWrong = $state<number[]>([]);
  let chkRight = $state<number | null>(null);
  let redRun = $state<Record<string, number>>({});
  let freeUsed = $state(false);
  let lastSaveAt = $state(Date.now());
  let nowMs = $state(Date.now());

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
  const stageList = $derived<Stage[]>([...(hasLearn ? ["learn" as const] : []), ...(q && q.quiz.length ? ["check" as const] : []), "build" as const]);
  const stageNames = { learn: "Learn", check: "Check", build: "Build" } as const;
  const stageXp = $derived<Record<Stage, string>>({ learn: "+2 XP per point", check: "+5 XP", build: `+${q?.xp ?? 0} XP` });
  const learnVisible = $derived(staged && (stage === "learn" || lessonOpen));
  const cur = $derived(steps[Math.min(step, Math.max(0, steps.length - 1))]);
  const curSolved = $derived(cur?.kind === "point" ? !!solved[cur.pi] : true);
  const lastStep = $derived(step >= steps.length - 1);
  const nextStage = $derived<Stage>(stageList[stageList.indexOf("learn") + 1] ?? "build");

  function initStage(d: QuestDetail) {
    const c = d.quest.kind !== "read" ? lessonCards(d.lesson) : null;
    const list: Stage[] = [];
    if (c && (c.cards.length || d.quest.points.length)) list.push("learn");
    if (d.quest.quiz.length) list.push("check");
    list.push("build");
    const saved = ls.get(`oxide:stage:${d.quest.id}`) as Stage | null;
    returning = saved !== null || d.completed || d.hints_used > 0 || (!!d.workspace && !app.fresh.has(d.quest.id));
    stage = saved && list.includes(saved) ? saved : returning ? "build" : list[0];
  }
  function goStage(s: Stage) {
    stage = s;
    lessonOpen = false;
    ls.set(`oxide:stage:${id}`, s);
    if (s === "learn") step = 0;
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
  // the card on screen counts as read
  $effect(() => {
    const c = cur;
    if (!learnVisible || !c || c.kind !== "card") return;
    untrack(() => {
      if (seen.includes(c.i)) return;
      seen = [...seen, c.i];
      ls.set(`oxide:read:${id}`, JSON.stringify(seen));
      void credit(c.i);
    });
  });
  function solvePoint(pi: number) {
    if (solved[pi]) return;
    solved[pi] = true;
    ls.set(`oxide:solved:${id}`, JSON.stringify(Object.keys(solved).map(Number)));
    void credit(50 + pi);
  }
  function go(d: number) {
    if (d > 0 && !curSolved) return;
    if (d > 0 && lastStep) { lessonOpen ? (lessonOpen = false) : goStage(nextStage); return; }
    step = Math.max(0, Math.min(steps.length - 1, step + d));
  }
  const goLesson = () => goStage("learn");
  const stepDone = (s: Step) => (s.kind === "card" ? seen.includes(s.i) : !!solved[s.pi]);
  const stepsRead = $derived(steps.filter(stepDone).length);

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
  const buildLive = $derived(staged && stage === "build" && started && !done && !lessonOpen);
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
    if ((e.key === "ArrowRight" || e.key === "ArrowLeft") && learnVisible && !e.ctrlKey && !e.metaKey && !e.altKey && !e.shiftKey) {
      const el = e.target as HTMLElement | null;
      if (el && (el.isContentEditable || ["INPUT", "TEXTAREA", "SELECT"].includes(el.tagName))) return;
      e.preventDefault();
      go(e.key === "ArrowRight" ? 1 : -1);
      return;
    }
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
  const quizReady = $derived(!!q && q.quiz.length > 0 && q.quiz.every((_, i) => answers[i] !== undefined));
  const icon = { passed: "✓", failed: "✕", notrun: "•" } as const;
  const tone = { passed: "border-ok/50 bg-ok/10 text-ok", failed: "border-bad/50 bg-bad/10 text-bad", notrun: "border-line bg-panel-2 text-dim" } as const;
</script>

<svelte:window onkeydown={onKey} />

{#snippet nextBtn()}
  {#if nextHere?.quest}
    <button class="btn btn-primary flex-1" onclick={() => app.goNext()} data-testid="next-quest">{nextHere.label}</button>
  {:else if nextHere}
    <a class="btn btn-ghost flex-1 text-center" href="/map" data-testid="next-none">Nothing unlocked here; open the Map</a>
  {/if}
{/snippet}

{#if !q || !detail}
  <div class="mx-auto max-w-6xl px-6 py-10"><div class="h-96 animate-pulse rounded-3xl bg-panel"></div></div>
{:else if staged && parts}
{#snippet consolePane()}
        <section bind:this={panelEl} class="mt-6 overflow-hidden rounded-2xl border border-line bg-[#0d0c0b]" data-testid="console">
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
{/snippet}

{#snippet sidePanel()}
    <aside class="flex flex-col gap-4 lg:sticky lg:top-6 lg:self-start">
      {#if q.kind === "read"}
        <section class="rounded-2xl border border-line bg-panel p-5" data-testid="quiz">
          <h2 class="mb-4 text-lg font-bold">Quick check</h2>
          {#each q.quiz as qz, qi}
            <fieldset class="mb-5">
              <legend class="mb-2 font-semibold">{qz.question}</legend>
              {#each qz.choices as c, ci}
                <label class="mb-1.5 flex cursor-pointer items-center gap-3 rounded-xl border px-3 py-2 {answers[qi] === ci ? 'border-rust bg-rust/10' : 'border-line hover:border-dim'}">
                  <input type="radio" name="q{qi}" class="accent-orange-500" checked={answers[qi] === ci} onchange={() => { answers[qi] = ci; quizMsg = ""; }} />
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
                {#if detail!.has_program}
                  <button class="btn btn-ghost flex-1" disabled={busy || running} onclick={runProgram} data-testid="run">{running ? "Running…" : "Run"}</button>
                {/if}
                <button class="btn btn-ghost" disabled={busy || checking || running} onclick={reset}>Reset</button>
              </div>
              {#if app.profile?.settings.editor}
                <button class="btn btn-ghost w-full" disabled={busy} onclick={openEditor} data-testid="open-external">Open in {externalName}</button>
              {/if}
            </div>
            <div class="mt-3 flex items-center gap-2 text-xs text-dim">
              <code class="min-w-0 flex-1 truncate font-mono" title={detail!.workspace}>{detail!.workspace}</code>
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
                <pre class="max-h-56 overflow-auto rounded-lg bg-black/40 p-3 font-mono text-xs leading-relaxed whitespace-pre-wrap">{d.rendered}</pre>
              </div>
            {/each}

            <ul class="flex flex-col gap-2">
              {#each rows as r (r.test + (check?.tick ?? 0))}
                <li class="rounded-xl border {tone[r.status]} {checking ? 'opacity-60' : ''}" data-status={r.status}>
                  <button
                    class="flex w-full items-start gap-3 px-3 py-2.5 text-left {r.status === 'failed' && r.output ? 'cursor-pointer' : 'cursor-default'}"
                    onclick={() => r.output && (expanded[r.test] = !expanded[r.test])}
                  >
                    <span class="mt-0.5 flex h-6 w-6 shrink-0 items-center justify-center rounded-full text-sm font-bold {r.status === 'passed' ? 'tick bg-ok text-black' : r.status === 'failed' ? 'bg-bad text-black' : 'bg-line text-dim'}">{icon[r.status]}</span>
                    <span class="flex-1 text-ink">{r.label}</span>
                    {#if r.status === "failed" && r.output}<span class="text-xs text-dim">{expanded[r.test] ? "hide" : "why?"}</span>{/if}
                  </button>
                  {#if expanded[r.test] && r.output}
                    <pre class="mx-3 mb-3 max-h-48 overflow-auto rounded-lg bg-black/40 p-3 font-mono text-xs leading-relaxed whitespace-pre-wrap text-ink">{r.output}</pre>
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
            <span class="text-sm text-dim tabular-nums">{detail!.hints_used} of {q.hints.length} used</span>
          </div>
          <ol class="mb-3 flex flex-col gap-2">
            {#each hintsShown as h, i}
              <li class="pop rounded-xl border border-line bg-panel-2 px-3 py-2 text-sm"><span class="mr-1 font-bold text-rust-hi">{i + 1}.</span>{@html renderMarkdown(h.text).replace(/^<p>|<\/p>\s*$/g, "")}</li>
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
{/snippet}

{#snippet learnView(inBuild: boolean)}
  <div class="rounded-2xl border border-line bg-panel px-8 py-7" data-testid="learn">
    {#if inBuild}
      <div class="mb-4 flex items-center justify-between">
        <h2 class="text-sm font-bold tracking-widest text-dim uppercase">Lesson</h2>
        <button class="btn btn-ghost !rounded-lg !px-3 !py-1 text-sm" onclick={() => (lessonOpen = false)} data-testid="lesson-close">← Back to code</button>
      </div>
    {/if}
    {#if cards?.why}<div class="prose-ox mb-5 rounded-xl border-l-4 border-rust bg-[#221a15] px-5 py-3 text-base text-dim" data-testid="why">{@html cards.why}</div>{/if}
    {#key `${step}|${lessonOpen}`}
      <div class="min-h-44" data-testid="card" data-step={step}>
        {#if cur?.kind === "card" && cards}
          {@const card = cards.cards[cur.i]}
          <div class="prose-ox point text-2xl leading-snug">{@html card.html}</div>
          {#each card.codes as c, ci}
            <div class="mt-5"><TypedCode code={c} playKey="{id}:{cur.i}:{ci}" /></div>
          {/each}
        {:else if cur?.kind === "point" && q}
          <PointStep point={q.points[cur.pi]} solved={!!solved[cur.pi]} onsolve={() => solvePoint(cur.pi)} />
        {/if}
      </div>
    {/key}
    <div class="mt-6 flex items-center gap-4">
      <span class="text-dim tabular-nums" data-testid="count">{step + 1} of {steps.length}</span>
      <div class="flex gap-1.5" aria-hidden="true">
        {#each steps as s, k}
          <i class="h-2.5 w-2.5 rounded-full {k === step ? 'bg-rust' : stepDone(s) ? 'bg-ok' : 'bg-line'}" data-testid="dot" data-state={k === step ? "on" : stepDone(s) ? "read" : ""}></i>
        {/each}
      </div>
      <span class="flex-1"></span>
      {#if returning && !inBuild}
        <button class="text-sm text-dim underline decoration-dotted hover:text-ink" onclick={() => goStage(nextStage)} data-testid="skip">Skip, I know this</button>
      {/if}
      <button class="btn btn-ghost" disabled={step === 0} onclick={() => go(-1)} data-testid="back">Back</button>
      <button class="btn btn-primary" disabled={!curSolved} onclick={() => go(1)} data-testid="next">
        {#if !lastStep}Next{:else if inBuild}Back to code{:else}Got it → {stageNames[nextStage]}{/if}
      </button>
    </div>
  </div>
{/snippet}

  <div class="mx-auto max-w-[96rem] px-6 py-8">
    <div>
      <div class="mb-3 flex flex-wrap items-center gap-3 text-sm">
        <span class="rounded-full border px-3 py-0.5 font-semibold {kindClass[q.kind]}">{kindLabel[q.kind]}</span>
        <span class="text-dim">{q.minutes} min</span>
        <span class="font-bold text-rust-hi">+{q.xp} XP</span>
        {#if done}<span class="rounded-full bg-ok/15 px-3 py-0.5 font-semibold text-ok">Cleared ✓</span>{/if}
      </div>
      <h1 class="mb-6 text-4xl font-extrabold">{q.title}</h1>
      <div class="mb-6 flex gap-2" data-testid="stage-bar">
        {#each stageList as s, k}
          {@const here = stageList.indexOf(stage)}
          {@const isDone = k < here || (done && s === "build" && s !== stage)}
          <button
            class="flex flex-1 items-center gap-3 rounded-xl border px-4 py-2.5 text-left font-semibold {s === stage ? 'border-rust bg-[#2a1d14] text-ink' : 'border-line bg-panel text-dim hover:text-ink'}"
            onclick={() => goStage(s)} data-testid="stage" data-stage={s} data-state={s === stage ? "on" : isDone ? "done" : ""}
          >
            <span class="flex h-7 w-7 items-center justify-center rounded-full border text-sm {s === stage ? 'border-rust bg-rust text-black' : isDone ? 'border-ok bg-ok text-black' : 'border-line'}">{k + 1}</span>
            {stageNames[s]}
            <span class="ml-auto text-sm font-normal text-rust-hi">{stageXp[s]}</span>
          </button>
        {/each}
      </div>
    </div>

    {#if stage === "learn"}
      <div class="max-w-[68rem]">{@render learnView(false)}</div>
    {:else if stage === "check"}
      <div class="max-w-[68rem]">
        <div class="rounded-2xl border border-line bg-panel px-8 py-7" data-testid="check-stage">
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
            {#if chkWrong.length && chkRight === null}<p class="mt-3 text-dim">Not that one. Re-read the points if you need to; guessing is free.</p>{/if}
          {/if}
          <div class="mt-6 flex items-center gap-4">
            {#if hasLearn}<button class="btn btn-ghost" onclick={() => goStage("learn")}>← Points</button>{/if}
            <span class="flex-1"></span>
            <button class="btn btn-primary" disabled={chkRight === null} onclick={afterCheck} data-testid="check-next">{q && chkQ < q.quiz.length - 1 ? "Next question" : "Build it →"}</button>
          </div>
        </div>
      </div>
    {:else}
      <div class="grid gap-8 lg:grid-cols-[minmax(0,1fr)_24rem]">
        <article class="min-w-0">
          {#if !hasLearn && cards?.why}<div class="prose-ox mb-5 rounded-xl border-l-4 border-rust bg-[#221a15] px-5 py-3 text-base text-dim" data-testid="why">{@html cards.why}</div>{/if}
          <section class="mb-6 rounded-2xl border border-rust/40 bg-panel p-5" data-testid="task">
            <h2 class="mb-3 text-sm font-bold tracking-widest text-rust-hi uppercase">Your turn</h2>
            <ul class="flex flex-col gap-2">
              {#each parts.tasks as t, i}
                {#if ticks.includes(i) || i === firstOpen}
                  <li>
                    <label class="flex cursor-pointer items-start gap-3 rounded-xl border px-4 py-3 text-lg {i === firstOpen ? 'border-rust bg-rust/10' : 'border-line bg-panel-2'} {ticks.includes(i) ? 'opacity-60' : ''}" data-testid="task-row" data-state={i === firstOpen ? "current" : "done"}>
                      <input type="checkbox" class="mt-1.5 h-5 w-5 shrink-0 accent-orange-500" checked={ticks.includes(i)} onchange={() => toggleTick(i)} data-testid="task-tick" />
                      <span class="prose-ox min-w-0 {ticks.includes(i) ? 'line-through' : ''}">{@html t}</span>
                    </label>
                  </li>
                {/if}
              {/each}
              {#if ghostCount > 0}
                <li class="flex items-center gap-3 rounded-xl border border-dashed border-line px-4 py-3 text-lg text-dim opacity-60" data-testid="task-ghost">
                  <span class="h-5 w-5 shrink-0 rounded-md border-2 border-dim"></span>{ghostCount} more {ghostCount === 1 ? "step" : "steps"}
                </li>
              {/if}
            </ul>
          </section>
          {#if started}
            <div class:hidden={lessonOpen}>
              {#key `${detail.workspace}|${resetN}`}
                <CodeEditor {id} diagnostics={result?.diagnostics ?? []} {checking} todo={todoIdx} tasks={parts.tasks.length} hot={stuckNow} onsave={onEditorSave} onlesson={hasLearn ? () => (lessonOpen = true) : undefined} />
              {/key}
            </div>
            {#if lessonOpen}{@render learnView(true)}{/if}
          {:else if hasLearn}
            <button class="btn btn-ghost" onclick={goLesson}>📖 Lesson</button>
          {/if}
          {#if run}{@render consolePane()}{/if}
        </article>
        {@render sidePanel()}
      </div>
    {/if}
  </div>

  {#if xpToast}
    {#key xpToast.key}<div class="pop fixed top-20 right-7 z-50 rounded-xl border border-rust bg-[#2a1d14] px-4 py-2 font-extrabold text-rust-hi" data-testid="xp-toast">+{xpToast.n} XP</div>{/key}
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
                  <input type="checkbox" class="mt-1.5 h-5 w-5 shrink-0 accent-orange-500" checked={ticks.includes(i)} onchange={() => toggleTick(i)} data-testid="task-tick" />
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
        <section bind:this={panelEl} class="mt-10 overflow-hidden rounded-2xl border border-line bg-[#0d0c0b]" data-testid="console">
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
                  <input type="radio" name="q{qi}" class="accent-orange-500" checked={answers[qi] === ci} onchange={() => { answers[qi] = ci; quizMsg = ""; }} />
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
                <pre class="max-h-56 overflow-auto rounded-lg bg-black/40 p-3 font-mono text-xs leading-relaxed whitespace-pre-wrap">{d.rendered}</pre>
              </div>
            {/each}

            <ul class="flex flex-col gap-2">
              {#each rows as r (r.test + (check?.tick ?? 0))}
                <li class="rounded-xl border {tone[r.status]} {checking ? 'opacity-60' : ''}" data-status={r.status}>
                  <button
                    class="flex w-full items-start gap-3 px-3 py-2.5 text-left {r.status === 'failed' && r.output ? 'cursor-pointer' : 'cursor-default'}"
                    onclick={() => r.output && (expanded[r.test] = !expanded[r.test])}
                  >
                    <span class="mt-0.5 flex h-6 w-6 shrink-0 items-center justify-center rounded-full text-sm font-bold {r.status === 'passed' ? 'tick bg-ok text-black' : r.status === 'failed' ? 'bg-bad text-black' : 'bg-line text-dim'}">{icon[r.status]}</span>
                    <span class="flex-1 text-ink">{r.label}</span>
                    {#if r.status === "failed" && r.output}<span class="text-xs text-dim">{expanded[r.test] ? "hide" : "why?"}</span>{/if}
                  </button>
                  {#if expanded[r.test] && r.output}
                    <pre class="mx-3 mb-3 max-h-48 overflow-auto rounded-lg bg-black/40 p-3 font-mono text-xs leading-relaxed whitespace-pre-wrap text-ink">{r.output}</pre>
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
              <li class="pop rounded-xl border border-line bg-panel-2 px-3 py-2 text-sm"><span class="mr-1 font-bold text-rust-hi">{i + 1}.</span>{@html renderMarkdown(h.text).replace(/^<p>|<\/p>\s*$/g, "")}</li>
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
