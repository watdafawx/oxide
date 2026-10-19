<script lang="ts">
  import { page } from "$app/state";
  import { api } from "$lib/api";
  import { app } from "$lib/store.svelte";
  import { renderMarkdown } from "$lib/markdown";
  import { kindClass, kindLabel, mmss } from "$lib/ui";
  import type { CheckResult, ObjectiveResult, QuestDetail } from "$lib/types";

  const id = $derived(page.params.id ?? "");

  let detail = $state<QuestDetail | null>(null);
  let busy = $state(false);
  let copied = $state(false);
  let expanded = $state<Record<string, boolean>>({});
  let answers = $state<number[]>([]);
  let quizMsg = $state("");

  $effect(() => {
    const qid = id;
    detail = null;
    expanded = {};
    answers = [];
    quizMsg = "";
    api.getQuest(qid).then((d) => { if (qid === id) detail = d; }).catch((e) => app.error(e));
  });

  const q = $derived(detail?.quest ?? null);
  const check = $derived(app.checks[id]);
  const running = $derived(check?.running ?? false);
  const result = $derived<CheckResult | null>(check?.result ?? detail?.last_check ?? null);
  const started = $derived(!!detail?.workspace);
  const done = $derived(detail?.completed ?? false);
  const lesson = $derived(detail ? renderMarkdown(detail.lesson) : "");
  const focusHere = $derived(app.focus && app.focus.quest_id === id ? app.focus : null);

  // Refresh counters (attempts, completed) after each result.
  $effect(() => {
    void check?.tick;
    if (check?.tick) api.getQuest(id).then((d) => { if (d.quest.id === id) detail = d; }).catch(() => {});
  });

  const rows = $derived<ObjectiveResult[]>(
    result?.objectives ?? (q?.objectives ?? []).map((o) => ({ ...o, status: "notrun" as const, output: null })),
  );
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
  const start = () => wrap(async () => { detail = await api.startQuest(id); });
  const reset = () => wrap(async () => {
    if (!confirm("Reset your copy of this quest? Progress stays, your code is wiped.")) return;
    delete app.checks[id];
    detail = await api.resetQuest(id);
  });
  const openEditor = () => wrap(() => api.openEditor(id));
  const runChecks = () => wrap(async () => { await api.runChecks(id); });
  const hint = () => wrap(async () => {
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
    app.ding();
    if (detail) detail.completed = true;
    if (c.first_time) app.celebrate(c);
  });

  const hintsShown = $derived(q ? q.hints.slice(0, detail?.hints_used ?? 0) : []);
  const quizReady = $derived(!!q && q.quiz.length > 0 && q.quiz.every((_, i) => answers[i] !== undefined));
  const icon = { passed: "✓", failed: "✕", notrun: "•" } as const;
  const tone = { passed: "border-ok/50 bg-ok/10 text-ok", failed: "border-bad/50 bg-bad/10 text-bad", notrun: "border-line bg-panel-2 text-dim" } as const;
</script>

{#if !q || !detail}
  <div class="mx-auto max-w-6xl px-6 py-10"><div class="h-96 animate-pulse rounded-3xl bg-panel"></div></div>
{:else}
  <div class="mx-auto grid max-w-6xl gap-8 px-6 py-8 lg:grid-cols-[minmax(0,1fr)_24rem]">
    <!-- lesson -->
    <article class="min-w-0">
      <div class="mb-3 flex flex-wrap items-center gap-3 text-sm">
        <span class="rounded-full border px-3 py-0.5 font-semibold {kindClass[q.kind]}">{kindLabel[q.kind]}</span>
        <span class="text-dim">{q.minutes} min</span>
        <span class="font-bold text-rust-hi">+{q.xp} XP</span>
        {#if done}<span class="rounded-full bg-ok/15 px-3 py-0.5 font-semibold text-ok">Cleared ✓</span>{/if}
      </div>
      <h1 class="mb-6 text-4xl font-extrabold">{q.title}</h1>
      <div class="prose-ox text-lg">{@html lesson}</div>
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
            <p class="font-semibold text-ok">Cleared ✓</p>
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
              <button class="btn btn-primary w-full" disabled={busy} onclick={openEditor}>Open in editor</button>
              <div class="flex gap-2">
                <button class="btn btn-ghost flex-1" disabled={busy || running} onclick={runChecks}>Run checks</button>
                <button class="btn btn-ghost" disabled={busy || running} onclick={reset}>Reset</button>
              </div>
            </div>
            <div class="mt-3 flex items-center gap-2 text-xs text-dim">
              <code class="min-w-0 flex-1 truncate font-mono" title={detail.workspace}>{detail.workspace}</code>
              <button class="rounded-md border border-line px-2 py-0.5 hover:text-ink" onclick={copy}>{copied ? "Copied" : "Copy"}</button>
            </div>
          {/if}
        </section>

        {#if started}
          <section class="rounded-2xl border border-line bg-panel p-5" data-testid="objectives">
            <div class="mb-3 flex items-center justify-between">
              <h2 class="text-lg font-bold">Objectives</h2>
              <span class="text-sm text-dim tabular-nums">{passedCount}/{rows.length}</span>
            </div>

            {#if running}
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
              {#each rows as r, i (r.test + (check?.tick ?? 0))}
                <li class="rounded-xl border {tone[r.status]} {running ? 'opacity-60' : ''}" data-status={r.status}>
                  <button
                    class="flex w-full items-start gap-3 px-3 py-2.5 text-left {r.status === 'failed' && r.output ? 'cursor-pointer' : 'cursor-default'}"
                    onclick={() => r.output && (expanded[r.test] = !expanded[r.test])}
                  >
                    <span class="mt-0.5 flex h-6 w-6 shrink-0 items-center justify-center rounded-full text-sm font-bold {r.status === 'passed' ? 'tick bg-ok text-black' : r.status === 'failed' ? 'bg-bad text-black' : 'bg-line text-dim'}" style="animation-delay:{i * 260}ms">{icon[r.status]}</span>
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
            <div class="mt-3 text-xs text-dim" data-testid="help">{q.runner === "lua" ? "Edit src/*.lua, tests.lua is the spec. Save and checks run automatically." : "Edit src/lib.rs, save, and checks run automatically."}</div>
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
          <button class="btn btn-ghost w-full" disabled={busy || detail.hints_used >= q.hints.length} onclick={hint}>
            {detail.hints_used >= q.hints.length ? "No more hints" : "Show next hint"}
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
  .tick { animation: tick 0.35s ease-out both; }
  @keyframes tick { from { transform: scale(0.2); opacity: 0 } 60% { transform: scale(1.3) } to { transform: scale(1); opacity: 1 } }
</style>
