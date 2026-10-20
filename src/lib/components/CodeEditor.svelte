<script lang="ts">
  import { onDestroy, onMount, untrack } from "svelte";
  import { EditorState, type Extension } from "@codemirror/state";
  import { EditorView, drawSelection, highlightActiveLine, highlightActiveLineGutter, keymap, lineNumbers } from "@codemirror/view";
  import { bracketMatching, indentOnInput, StreamLanguage, syntaxHighlighting } from "@codemirror/language";
  import { defaultKeymap, history, historyKeymap, indentWithTab } from "@codemirror/commands";
  import { lintGutter, setDiagnostics, type Diagnostic as CmDiag } from "@codemirror/lint";
  import { closeBrackets, closeBracketsKeymap } from "@codemirror/autocomplete";
  import { oneDarkHighlightStyle } from "@codemirror/theme-one-dark";
  import { rust } from "@codemirror/lang-rust";
  import { java } from "@codemirror/lang-java";
  import { lua } from "@codemirror/legacy-modes/mode/lua";
  import { toml } from "@codemirror/legacy-modes/mode/toml";
  import { api } from "$lib/api";
  import { app } from "$lib/store.svelte";
  import type { Diagnostic, FileEntry } from "$lib/types";

  let { id, diagnostics: rawDiagnostics = [], checking = false }: { id: string; diagnostics?: Diagnostic[]; checking?: boolean } = $props();

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

  // ---- theme ----
  const theme = EditorView.theme({
    "&": { height: "100%", backgroundColor: "#0d0c0b", color: "#f2ebe4", fontSize: "15px" },
    ".cm-scroller": { fontFamily: "var(--font-mono)", lineHeight: "1.6" },
    ".cm-content": { caretColor: "#fb923c", padding: "8px 0" },
    ".cm-cursor": { borderLeftColor: "#fb923c" },
    "&.cm-focused": { outline: "none" },
    ".cm-gutters": { backgroundColor: "#0d0c0b", color: "#6f655b", border: "none", borderRight: "1px solid #38332e" },
    ".cm-activeLine": { backgroundColor: "#ffffff08" },
    ".cm-activeLineGutter": { backgroundColor: "#ffffff0d", color: "#f2ebe4" },
    ".cm-selectionBackground, &.cm-focused .cm-selectionBackground, ::selection": { backgroundColor: "#f9731640" },
    "&.cm-focused .cm-matchingBracket": { backgroundColor: "#f9731655", outline: "1px solid #f97316aa" },
    ".cm-tooltip": { backgroundColor: "#262320", border: "1px solid #38332e", color: "#f2ebe4", borderRadius: "8px" },
    ".cm-diagnostic": { padding: "4px 8px", fontFamily: "var(--font-sans)", fontSize: "14px" },
  }, { dark: true });

  const langFor = (path: string): Extension => {
    if (path.endsWith(".rs")) return rust();
    if (path.endsWith(".java") || path.endsWith(".kts") || path.endsWith(".kt")) return java();
    if (path.endsWith(".lua")) return StreamLanguage.define(lua);
    if (path.endsWith(".toml")) return StreamLanguage.define(toml);
    return [];
  };

  function mkState(path: string, text: string, ro: boolean): EditorState {
    return EditorState.create({
      doc: text,
      extensions: [
        lineNumbers(), highlightActiveLineGutter(), highlightActiveLine(), drawSelection(), history(),
        indentOnInput(), bracketMatching(), closeBrackets(), lintGutter(),
        syntaxHighlighting(oneDarkHighlightStyle), theme, langFor(path),
        keymap.of([
          { key: "Mod-s", preventDefault: true, run: () => { void save(); return true; } },
          indentWithTab, ...closeBracketsKeymap, ...defaultKeymap, ...historyKeymap,
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

<section class="overflow-hidden rounded-2xl border border-line bg-panel" data-testid="editor">
  <header class="flex flex-wrap items-center gap-2 border-b border-line px-3 py-2">
    <h2 class="mr-1 text-sm font-bold tracking-widest text-dim uppercase">Code</h2>
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
  <div style="height:{height}px" class="relative">
    <div bind:this={host} class="h-full" data-testid="cm"></div>
    {#if loading}<div class="absolute inset-0 animate-pulse bg-panel/80"></div>{/if}
    {#if activeRO}<div class="pointer-events-none absolute right-3 bottom-3 rounded-lg bg-black/70 px-3 py-1 text-xs text-dim">🔒 read only: this is the spec</div>{/if}
  </div>
  <div
    class="h-2.5 cursor-row-resize border-t border-line bg-panel-2 hover:bg-line focus:bg-rust/40 focus:outline-none" role="slider" aria-orientation="vertical" aria-valuenow={height} aria-valuemin={180} aria-label="Editor height" tabindex="0"
    onpointerdown={drag} onkeydown={dragKey} data-testid="resize"
  ></div>
  {#if diagnostics.length}
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
