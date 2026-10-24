<script lang="ts">
  import { onDestroy, onMount, untrack } from "svelte";
  import { EditorState, StateEffect, StateField, type Extension } from "@codemirror/state";
  import { Decoration, type DecorationSet, EditorView, WidgetType, drawSelection, highlightActiveLine, highlightActiveLineGutter, hoverTooltip, keymap, lineNumbers } from "@codemirror/view";
  import { bracketMatching, HighlightStyle, indentOnInput, StreamLanguage, syntaxHighlighting } from "@codemirror/language";
  import { tags as t } from "@lezer/highlight";
  import { defaultKeymap, history, historyKeymap, indentWithTab } from "@codemirror/commands";
  import { lintGutter, setDiagnostics, type Diagnostic as CmDiag } from "@codemirror/lint";
  import { autocompletion, closeBrackets, closeBracketsKeymap, completionKeymap } from "@codemirror/autocomplete";
  import { completionSource, docDom, explain, langOfPath, lookup, rich, type Lang } from "$lib/assist";
  import { rust } from "@codemirror/lang-rust";
  import { java } from "@codemirror/lang-java";
  import { lua } from "@codemirror/legacy-modes/mode/lua";
  import { toml } from "@codemirror/legacy-modes/mode/toml";
  import { api } from "$lib/api";
  import { app } from "$lib/store.svelte";
  import type { Diagnostic, FileEntry } from "$lib/types";

  let {
    id, diagnostics: rawDiagnostics = [], checking = false, todo = null, tasks = 0, hot = false, onsave, onlesson, fill = false, list = true,
  }: {
    /** grow to fill the parent's height instead of the drag-to-resize height */
    fill?: boolean;
    /** show the list of compiler messages under the editor */
    list?: boolean;
    id: string; diagnostics?: Diagnostic[]; checking?: boolean;
    /** highlight (and scroll to) the Nth TODO comment of the open, editable file; null = none */
    todo?: number | null;
    /** how many tasks the learner sees; extra tasks beyond the TODO count ("Open the file", "Save") shift the match */
    tasks?: number;
    /** stronger highlight: the learner looks stuck */
    hot?: boolean;
    /** called after a file was written to disk */
    onsave?: () => void;
    /** shows a "Lesson" tab at the end of the tab row */
    onlesson?: () => void;
  } = $props();

  // ---- TODO highlight ----
  const setTodoLine = StateEffect.define<{ pos: number; hot: boolean } | null>();
  const todoField = StateField.define<DecorationSet>({
    create: () => Decoration.none,
    update(v, tr) {
      v = v.map(tr.changes);
      for (const e of tr.effects) {
        if (!e.is(setTodoLine)) continue;
        v = e.value ? Decoration.set([Decoration.line({ class: e.value.hot ? "cm-todo cm-todo-hot" : "cm-todo" }).range(e.value.pos)]) : Decoration.none;
      }
      return v;
    },
    provide: (f) => EditorView.decorations.from(f),
  });
  const TODO_RE = /(?:\/\/|--|#)\s*TODO\b/;

  const ls = {
    get(k: string): string | null { try { return localStorage.getItem(k); } catch { return null; } },
    set(k: string, v: string) { try { localStorage.setItem(k, v); } catch { /* private mode */ } },
  };
  const AUTOSAVE_MS = 1500;
  /** the engine sometimes leaves file/line empty; the rendered text still has "src/lib.rs:3:5" */
  const diagnostics = $derived(rawDiagnostics.map((d) => {
    if (d.file && d.line) return d;
    const m = /([\w./\\-]+\.(?:rs|java|kts|lua|toml)):(\d+)/.exec(d.rendered ?? "");
    if (!m) return d;
    const msg = /^\s*(?:error|warning)(?:\[\w+\])?: (.+)$/m.exec(d.rendered ?? "")?.[1];
    return { ...d, file: d.file ?? m[1], line: d.line ?? Number(m[2]), message: msg ?? d.message };
  }));
  const autosaveEnabled = () => ls.get("oxide:autosave") !== "0";

  let host = $state<HTMLElement | null>(null);
  let view: EditorView | null = null;
  let files = $state<FileEntry[]>([]);
  let active = $state("");
  let loading = $state(true);
  let saving = $state(false);
  /** text last written to / read from disk, per path */
  const saved: Record<string, string> = {};
  const states: Record<string, EditorState> = {};
  let dirty = $state<Record<string, boolean>>({});
  let timer: ReturnType<typeof setTimeout> | null = null;

  // ---- height (drag handle, remembered) ----
  const clampH = (h: number) => Math.max(180, Math.min(h, Math.round(window.innerHeight * 0.9)));
  let height = $state(clampH(Number(ls.get("oxide:editor-h")) || Math.round(window.innerHeight * 0.6)));
  function drag(e: PointerEvent) {
    e.preventDefault();
    const y0 = e.clientY, h0 = height;
    const move = (m: PointerEvent) => { height = clampH(h0 + m.clientY - y0); };
    const up = () => {
      window.removeEventListener("pointermove", move);
      window.removeEventListener("pointerup", up);
      ls.set("oxide:editor-h", String(height));
    };
    window.addEventListener("pointermove", move);
    window.addEventListener("pointerup", up);
  }
  function dragKey(e: KeyboardEvent) {
    if (e.key !== "ArrowUp" && e.key !== "ArrowDown") return;
    e.preventDefault();
    height = clampH(height + (e.key === "ArrowDown" ? 40 : -40));
    ls.set("oxide:editor-h", String(height));
  }

  // ---- theme: colors come from the look (app.css) ----
  const theme = EditorView.theme({
    "&": { height: "100%", backgroundColor: "var(--color-code)", color: "var(--color-ink)", fontSize: "15px" },
    ".cm-scroller": { fontFamily: "var(--font-mono)", lineHeight: "1.6" },
    ".cm-content": { caretColor: "var(--color-rust)", padding: "8px 0" },
    ".cm-cursor": { borderLeftColor: "var(--color-rust)", borderLeftWidth: "2px" },
    "&.cm-focused": { outline: "none" },
    ".cm-gutters": { backgroundColor: "var(--color-code)", color: "var(--cm-gutter)", border: "none", borderRight: "1px solid var(--color-line)" },
    ".cm-activeLine": { backgroundColor: "var(--cm-active)" },
    ".cm-activeLineGutter": { backgroundColor: "var(--cm-active)", color: "var(--color-ink)" },
    ".cm-selectionBackground, &.cm-focused .cm-selectionBackground, ::selection": { backgroundColor: "var(--cm-select)" },
    "&.cm-focused .cm-matchingBracket": { backgroundColor: "var(--cm-select)", outline: "1px solid var(--color-rust)" },
    ".cm-tooltip": { backgroundColor: "var(--color-panel-2)", border: "1px solid var(--color-line)", color: "var(--color-ink)", borderRadius: "8px", boxShadow: "0 14px 34px rgba(0,0,0,.3)" },
    ".cm-tooltip-autocomplete > ul": { fontFamily: "var(--font-mono)", maxHeight: "16em" },
    ".cm-tooltip-autocomplete > ul > li": { padding: "3px 10px 3px 6px" },
    ".cm-tooltip-autocomplete > ul > li[aria-selected]": { backgroundColor: "var(--color-tint)", color: "var(--color-ink)" },
    ".cm-completionMatchedText": { textDecoration: "none", color: "var(--color-rust)", fontWeight: "700" },
    ".cm-completionDetail": { color: "var(--color-dim)", fontStyle: "normal", marginLeft: "1em" },
    ".cm-completionInfo": { padding: "8px 12px", maxWidth: "320px" },
    ".cm-completionIcon": { opacity: "0.7" },
    ".cm-tooltip-hover": { padding: "8px 12px", maxWidth: "360px" },
    ".cm-todo": { backgroundColor: "color-mix(in srgb, var(--color-rust) 16%, transparent)", boxShadow: "inset 3px 0 0 var(--color-rust)" },
    ".cm-todo-hot": { backgroundColor: "color-mix(in srgb, var(--color-rust) 32%, transparent)", outline: "1px solid var(--color-rust)" },
    ".cm-diagnostic": { padding: "4px 8px", fontFamily: "var(--font-sans)", fontSize: "14px" },
    ".cm-lintRange-error": { backgroundImage: "none", textDecoration: "underline wavy var(--color-bad)", textUnderlineOffset: "4px", textDecorationSkipInk: "none" },
  });
  const highlight = HighlightStyle.define([
    { tag: [t.keyword, t.controlKeyword, t.definitionKeyword, t.modifier, t.operatorKeyword, t.self], color: "var(--syn-k)" },
    { tag: [t.function(t.variableName), t.function(t.propertyName), t.function(t.definition(t.variableName))], color: "var(--syn-f)" },
    { tag: [t.macroName], color: "var(--syn-m)" },
    { tag: [t.string, t.special(t.string), t.character], color: "var(--syn-s)" },
    { tag: [t.typeName, t.className, t.namespace], color: "var(--syn-t)" },
    { tag: [t.comment, t.lineComment, t.blockComment, t.docComment], color: "var(--syn-c)", fontStyle: "italic" },
    { tag: [t.number, t.bool, t.null, t.atom], color: "var(--syn-n)" },
    { tag: [t.attributeName, t.meta, t.annotation, t.processingInstruction], color: "var(--syn-a)" },
    { tag: [t.variableName, t.propertyName], color: "var(--syn-v)" },
    { tag: [t.invalid], color: "var(--color-bad)" },
  ]);

  // ---- plain-English error notes, drawn as a box under the broken line ----
  class ExplainWidget extends WidgetType {
    d: Diagnostic;
    lang: Lang;
    constructor(d: Diagnostic, lang: Lang) { super(); this.d = d; this.lang = lang; }
    eq(o: ExplainWidget) { return o.d.message === this.d.message && o.d.line === this.d.line && o.d.rendered === this.d.rendered; }
    toDOM(v: EditorView) {
      const x = explain(this.d, this.lang);
      const box = document.createElement("div");
      box.className = "ox-explain";
      box.dataset.testid = "explain";
      // keep the box inside the visible code area so it never scrolls sideways
      const room = v.scrollDOM.clientWidth - (v.dom.querySelector(".cm-gutters") as HTMLElement | null)?.offsetWidth! - 32;
      if (room > 200) box.style.maxWidth = `${Math.min(room, 736)}px`;
      const head = document.createElement("div");
      head.className = "ox-explain-head";
      const b = document.createElement("b");
      b.textContent = x.title;
      const code = /error\[(E\d{4})\]/.exec(this.d.rendered ?? "")?.[1];
      const small = document.createElement("small");
      small.textContent = code ? `${code} · ${this.d.message}` : this.d.message === x.body ? "" : this.d.message;
      head.append(b, small);
      const p = document.createElement("p");
      rich(p, x.body);
      box.append(head, p);
      if (x.tip) { const tp = document.createElement("p"); tp.className = "ox-explain-tip"; rich(tp, x.tip); box.append(tp); }
      const acts = document.createElement("div");
      acts.className = "ox-explain-acts";
      const raw = document.createElement("pre");
      raw.className = "ox-explain-raw";
      raw.textContent = this.d.rendered || this.d.message;
      raw.hidden = true;
      if (x.fix) {
        const fx = x.fix;
        const btn = document.createElement("button");
        btn.className = "ox-pri";
        btn.textContent = fx.label;
        btn.dataset.testid = "explain-fix";
        btn.onclick = () => {
          if (fx.apply(v)) { p.textContent = "Done. It checks again when it saves."; btn.remove(); v.focus(); }
          else btn.textContent = "Couldn't apply it here: edit by hand";
        };
        acts.append(btn);
      }
      const show = document.createElement("button");
      show.textContent = "Compiler output";
      show.onclick = () => { raw.hidden = !raw.hidden; show.textContent = raw.hidden ? "Compiler output" : "Hide compiler output"; };
      acts.append(show);
      box.append(acts, raw);
      return box;
    }
    ignoreEvent() { return true; }
  }
  const setExplain = StateEffect.define<DecorationSet>();
  const explainField = StateField.define<DecorationSet>({
    create: () => Decoration.none,
    update(v, tr) {
      v = v.map(tr.changes);
      for (const e of tr.effects) if (e.is(setExplain)) v = e.value;
      return v;
    },
    provide: (f) => EditorView.decorations.from(f),
  });

  /** hover a known word: a short note */
  const hoverHelp = (lang: Lang) => hoverTooltip((v, pos) => {
    const line = v.state.doc.lineAt(pos);
    const rel = pos - line.from;
    const m = [...line.text.matchAll(/[A-Za-z_]\w*!?/g)].find((w) => w.index! <= rel && rel <= w.index! + w[0].length);
    if (!m) return null;
    const e = lookup(lang, m[0], line.text[m.index! - 1] === "." || line.text[m.index! - 1] === ":");
    if (!e) return null;
    return { pos: line.from + m.index!, end: line.from + m.index! + m[0].length, above: true, create: () => ({ dom: docDom(e) as HTMLElement }) };
  }, { hoverTime: 450 });

  const langFor = (path: string): Extension => {
    if (path.endsWith(".rs")) return rust();
    if (path.endsWith(".java") || path.endsWith(".kts") || path.endsWith(".kt")) return java();
    if (path.endsWith(".lua")) return StreamLanguage.define(lua);
    if (path.endsWith(".toml")) return StreamLanguage.define(toml);
    return [];
  };

  function mkState(path: string, text: string, ro: boolean): EditorState {
    const lang = langOfPath(path);
    const opts = app.profile?.settings;
    return EditorState.create({
      doc: text,
      extensions: [
        lineNumbers(), highlightActiveLineGutter(), highlightActiveLine(), drawSelection(), history(),
        indentOnInput(), bracketMatching(), closeBrackets(), lintGutter(),
        syntaxHighlighting(highlight), theme, langFor(path), todoField, explainField,
        autocompletion({ override: [completionSource(lang)], activateOnTyping: opts?.complete ?? true, icons: true, closeOnBlur: true }),
        hoverHelp(lang),
        keymap.of([
          { key: "Mod-s", preventDefault: true, run: () => { void save(); return true; } },
          ...completionKeymap, indentWithTab, ...closeBracketsKeymap, ...defaultKeymap, ...historyKeymap,
        ]),
        EditorState.readOnly.of(ro), EditorView.editable.of(!ro),
        EditorView.updateListener.of((u) => {
          if (!u.docChanged || u.state !== view?.state) return;
          const p = active;
          dirty[p] = u.state.doc.toString() !== saved[p];
          if (timer) clearTimeout(timer);
          if (dirty[p] && autosaveEnabled()) timer = setTimeout(() => void save(p), AUTOSAVE_MS);
        }),
      ],
    });
  }

  const entry = (p: string) => files.find((f) => f.path === p);
  const norm = (p: string) => p.replaceAll("\\", "/").replace(/^\.\//, "");
  const matches = (path: string, file: string | null) => {
    if (!file) return false;
    const a = norm(path), b = norm(file);
    return a === b || a.endsWith("/" + b) || b.endsWith("/" + a);
  };

  function applyDiagnostics() {
    if (!view) return;
    const doc = view.state.doc;
    const out: CmDiag[] = [];
    for (const d of diagnostics) {
      if (!d.line || !matches(active, d.file) || d.line > doc.lines) continue;
      const l = doc.line(d.line);
      const lead = l.text.length - l.text.trimStart().length;
      const from = l.from + lead, to = Math.max(from + 1, l.to);
      out.push({ from, to: Math.min(to, doc.length), severity: d.level === "error" ? "error" : "warning", message: d.message });
    }
    view.dispatch(setDiagnostics(view.state, out));
    // plain-English boxes: errors only, one per line, at most three
    const boxes = [];
    const lines = new Set<number>();
    if (app.profile?.settings.explain ?? true)
      for (const d of diagnostics) {
        if (d.level !== "error" || !d.line || !matches(active, d.file) || d.line > doc.lines || lines.has(d.line) || lines.size >= 3) continue;
        lines.add(d.line);
        boxes.push(Decoration.widget({ widget: new ExplainWidget(d, langOfPath(active)), block: true, side: 1 }).range(doc.line(d.line).to));
      }
    view.dispatch({ effects: setExplain.of(Decoration.set(boxes, true)) });
  }

  /** put the TODO highlight on the `todo`-th TODO line of the open file (editable files only) */
  function applyTodo(scroll: boolean) {
    if (!view) return;
    const doc = view.state.doc;
    let pos: number | null = null;
    if (todo !== null && !activeRO) {
      const found: number[] = [];
      for (let n = 1; n <= doc.lines; n++) if (TODO_RE.test(doc.line(n).text)) found.push(doc.line(n).from);
      // Nth task <-> Nth TODO, except that "Open the file" style tasks beyond the TODO count push the match back
      const at = todo - Math.max(0, tasks - found.length);
      if (at >= 0 && at < found.length) pos = found[at];
    }
    view.dispatch({ effects: setTodoLine.of(pos === null ? null : { pos, hot }) });
    // scroll the editor's own viewport only; scrollIntoView would drag the whole page along
    if (scroll && pos !== null) {
      const v = view;
      v.requestMeasure({
        read: () => v.lineBlockAt(pos).top - v.scrollDOM.clientHeight / 3,
        write: (top) => { v.scrollDOM.scrollTop = Math.max(0, top); },
      });
    }
  }

  async function open(path: string) {
    if (!view) return;
    if (active) states[active] = view.state;
    active = path;
    if (!states[path]) {
      try {
        const text = await api.readFile(id, path);
        saved[path] = text;
        states[path] = mkState(path, text, entry(path)?.read_only ?? false);
      } catch (e) { app.error(e); return; }
    }
    if (active !== path) return;
    view.setState(states[path]);
    applyDiagnostics();
    applyTodo(true);
  }

  async function save(path = active) {
    if (!view || !path) return;
    if (timer) { clearTimeout(timer); timer = null; }
    const st = path === active ? view.state : states[path];
    if (!st || entry(path)?.read_only) return;
    const text = st.doc.toString();
    if (text === saved[path]) { dirty[path] = false; return; }
    saving = true;
    try {
      await api.writeFile(id, path, text);
      saved[path] = text;
      onsave?.();
      const now = path === active ? view.state.doc.toString() : states[path].doc.toString();
      dirty[path] = now !== text;
    } catch (e) {
      app.error(e);
    } finally {
      saving = false;
    }
  }

  async function load() {
    loading = true;
    try {
      files = (await api.listFiles(id)).filter((f) => !/(^|\/)(target|build|\.gradle|node_modules)\//.test(f.path)).sort((a, b) => Number(b.primary) - Number(a.primary));
    } catch (e) {
      files = [];
      app.error(e);
    }
    loading = false;
    const first = files.find((f) => f.primary) ?? files.find((f) => !f.read_only) ?? files[0];
    if (first) await open(first.path);
  }

  const setText = (t: string) => {
    if (!view) return;
    view.dispatch({ changes: { from: 0, to: view.state.doc.length, insert: t } });
  };

  onMount(() => {
    view = new EditorView({ state: EditorState.create({ doc: "" }), parent: host! });
    if (import.meta.env.DEV) {
      (window as unknown as { __oxideEditor: unknown }).__oxideEditor = {
        setText, getText: () => view?.state.doc.toString() ?? "", save: () => save(), get file() { return active; },
        todoLine: () => { const el = host?.querySelector(".cm-todo"); return el ? { text: el.textContent, hot: el.classList.contains("cm-todo-hot") } : null; },
      };
    }
    void load();
  });
  onDestroy(() => {
    if (timer) { clearTimeout(timer); void save(); }
    view?.destroy();
    if (import.meta.env.DEV) delete (window as unknown as { __oxideEditor?: unknown }).__oxideEditor;
  });

  // new diagnostics arrive -> markers
  $effect(() => {
    void diagnostics;
    void active;
    untrack(() => applyDiagnostics());
  });

  // current task / stuck changed -> move the highlight
  let lastTodo: number | null | undefined;
  let lastHot = false;
  $effect(() => {
    const t = todo, h = hot;
    void active;
    untrack(() => {
      if (!view || !active) return;
      applyTodo(t !== lastTodo || (h && !lastHot));
      lastTodo = t;
      lastHot = h;
    });
  });

  const anyDirty = $derived(Object.values(dirty).some(Boolean));
  const activeRO = $derived(entry(active)?.read_only ?? false);
  const base = (p: string) => p.split("/").pop() ?? p;
  function jump(path: string, line: number | null) {
    const go = async () => {
      if (path !== active) await open(path);
      if (!view || !line || line > view.state.doc.lines) return;
      const l = view.state.doc.line(line);
      view.dispatch({ selection: { anchor: l.from }, scrollIntoView: true });
      view.focus();
    };
    void go();
  }
</script>

<section class="overflow-hidden rounded-[var(--radius-box)] border border-line bg-panel {fill ? 'flex h-full min-h-0 flex-col' : ''}" data-testid="editor">
  <header class="flex flex-wrap items-center gap-2 border-b border-line px-3 py-2">
    <h2 class="mr-1 text-xs font-bold tracking-widest text-dim uppercase">Code</h2>
    <div class="flex min-w-0 flex-1 flex-wrap gap-1" role="tablist">
      {#each files as f (f.path)}
        <button
          role="tab" aria-selected={f.path === active} title={f.path}
          class="flex items-center gap-1.5 rounded-lg border px-3 py-1 font-mono text-sm {f.path === active ? 'border-rust bg-rust/10 text-ink' : 'border-line text-dim hover:text-ink'}"
          onclick={() => open(f.path)} data-testid="tab" data-path={f.path}
        >
          {#if f.read_only}<span title="read only" aria-label="read only">🔒</span>{/if}
          {base(f.path)}
          {#if dirty[f.path]}<span class="h-2 w-2 rounded-full bg-rust-hi" title="unsaved" data-testid="dirty"></span>{/if}
        </button>
      {/each}
      {#if onlesson}
        <button class="ml-auto flex items-center gap-1.5 rounded-lg border border-line px-3 py-1 text-sm text-ink hover:border-rust" onclick={onlesson} data-testid="lesson-tab">📖 Lesson</button>
      {/if}
    </div>
    {#if checking}
      <span class="flex items-center gap-2 text-sm text-amber-200" data-testid="editor-checking">
        <span class="inline-block h-3.5 w-3.5 animate-spin rounded-full border-2 border-amber-300 border-t-transparent"></span>Checking…
      </span>
    {:else if saving}
      <span class="text-sm text-dim">Saving…</span>
    {:else if !anyDirty}
      <span class="text-sm text-dim" data-testid="saved">Saved</span>
    {/if}
    <button class="btn btn-primary !rounded-lg !px-3 !py-1 text-sm" disabled={!anyDirty || saving || activeRO} onclick={() => save()} data-testid="save">Save <span class="font-normal opacity-70">Ctrl+S</span></button>
  </header>
  <div style={fill ? "" : `height:${height}px`} class="relative {fill ? 'min-h-0 flex-1' : ''}">
    <div bind:this={host} class="h-full" data-testid="cm"></div>
    {#if loading}<div class="absolute inset-0 animate-pulse bg-panel/80"></div>{/if}
    {#if activeRO}<div class="pointer-events-none absolute right-3 bottom-3 rounded-lg border border-line bg-panel-2 px-3 py-1 text-xs text-dim">🔒 read only: this is the spec</div>{/if}
  </div>
  {#if !fill}<div
    class="h-2.5 cursor-row-resize border-t border-line bg-panel-2 hover:bg-line focus:bg-rust/40 focus:outline-none" role="slider" aria-orientation="vertical" aria-valuenow={height} aria-valuemin={180} aria-label="Editor height" tabindex="0"
    onpointerdown={drag} onkeydown={dragKey} data-testid="resize"
  ></div>{/if}
  {#if list && diagnostics.length}
    <ul class="flex max-h-40 flex-col overflow-auto border-t border-line text-sm" data-testid="diag-list">
      {#each diagnostics as d}
        {@const target = files.find((f) => matches(f.path, d.file))}
        <li>
          <button class="flex w-full items-baseline gap-3 px-4 py-1.5 text-left hover:bg-panel-2 disabled:cursor-default" disabled={!target} onclick={() => target && jump(target.path, d.line)}>
            <span class="font-bold {d.level === 'error' ? 'text-bad' : 'text-amber-300'}">{d.level === "error" ? "✕" : "▲"}</span>
            <span class="font-mono text-xs text-dim">{d.file ?? "compiler"}{d.line ? `:${d.line}` : ""}</span>
            <span class="min-w-0 flex-1 truncate text-ink">{d.message}</span>
          </button>
        </li>
      {/each}
    </ul>
  {/if}
</section>

<style>
  :global(.ox-explain) { margin: 4px 16px 8px 4px; border-left: 3px solid var(--color-bad); background: color-mix(in srgb, var(--color-bad) 9%, var(--color-code)); border-radius: 0 8px 8px 0; padding: 9px 13px; font: 14px/1.5 var(--font-sans); color: var(--color-ink); max-width: 46rem; white-space: normal; }
  :global(.ox-explain-head) { display: flex; gap: 10px; align-items: baseline; flex-wrap: wrap; }
  :global(.ox-explain-head b) { color: var(--color-bad); }
  :global(.ox-explain-head small) { font-family: var(--font-mono); color: var(--color-dim); font-size: 12px; }
  :global(.ox-explain p) { margin: 3px 0 7px; }
  :global(.ox-explain-tip) { color: var(--color-dim); }
  :global(.ox-explain code, .ox-info code) { font-family: var(--font-mono); font-size: .9em; background: var(--color-panel-2); padding: 0 5px; border-radius: 4px; }
  :global(.ox-explain-acts) { display: flex; gap: 8px; flex-wrap: wrap; }
  :global(.ox-explain-acts button) { border: 1px solid var(--color-line); background: var(--color-panel); color: var(--color-ink); border-radius: 6px; padding: 2px 10px; font-size: 13px; cursor: pointer; }
  :global(.ox-explain-acts button.ox-pri) { background: var(--color-rust); border-color: var(--color-rust); color: var(--color-on-rust); font-weight: 600; }
  :global(.ox-explain-raw) { margin: 8px 0 0; max-height: 14rem; overflow: auto; background: var(--color-code); border: 1px solid var(--color-line); border-radius: 6px; padding: 8px 10px; font: 12px/1.5 var(--font-mono); white-space: pre-wrap; }
  :global(.ox-info) { font: 13px/1.45 var(--font-sans); }
  :global(.ox-info p) { margin: 6px 0 0; color: var(--color-ink); }
  :global(.ox-sig) { font-weight: 600; }
  :global(.ox-eg) { margin-top: 6px; font: 12px var(--font-mono); background: var(--color-code); border-radius: 6px; padding: 5px 8px; color: var(--color-dim); }
</style>
