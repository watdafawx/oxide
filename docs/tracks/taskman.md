# Task manager in Rust (GPUI by default, egui as a port)

Planning doc for a new project track, `taskman`, nested under Rust like the GPUI track
(`parent = "rust"`). Format and rules: `docs/CONTENT.md`; the GPUI testing method this track reuses:
`docs/tracks/gpui.md` section 1. Research and measurements: 2026-10-25.

## 0. What we build, and why a world just for the GUI

**Project spine:** `taskman`, a Windows task manager that opens at once, lists every process,
updates every second, sorts, searches and ends tasks, and stays lighter than the thing it
replaces. Names come from the project: `Process`, `ProcessRow`, `TaskList`, `SortBy`,
`Snapshot`, `ProcessTree`, `EndTask`, `Sampler`.

Rust has three serious ways to draw that window, and they change how every later screen is
written. So the track opens with a world that builds the **same row in egui and in GPUI**,
explains the web route, shows measured numbers, and only then commits. That world is built
(`content/tracks/taskman/worlds/00-pick-a-gui`, 12 quests). Worlds 01-08 are planned here.

## 1. The three routes

| | egui (+ eframe) | GPUI | Web: Tauri (+ Svelte, or Leptos / Dioxus) |
|---|---|---|---|
| model | immediate mode: one `ui` function draws everything on every run | views (structs GPUI keeps) that describe themselves in `render`; elements are rebuilt when a view calls `cx.notify()` | a web page in the system web view (WebView2); Rust behind `#[tauri::command]` and events |
| a click | `if ui.button("CPU").clicked() { .. }` | `.on_click(cx.listener(\|view, _, _, cx\| { ..; cx.notify() }))` | JS handler calls `invoke("sort")` |
| state lives | in your struct; egui keeps only bookkeeping (`Memory`: scroll, focus, column widths) | in entities GPUI owns (`Entity<T>`) | JS store + Rust state behind commands |
| layout | one pass, top to bottom (sometimes a second, discarded sizing pass); `horizontal`, `columns`, `TableBuilder` | flexbox and grid (Taffy); Tailwind-style methods (`.flex().gap_4()`) | CSS |
| long lists | `TableBuilder::body.rows(h, n, ..)` draws visible rows only | `uniform_list` draws visible rows only | virtual-scroll component, or the DOM holds every row |
| widgets in the box | many in egui and its companion crates: buttons, sliders, text edits, tables (`egui_extras`), plots (`egui_plot`), docking (third-party `egui_dock`) | few: boxes, text, images, lists, `canvas`; no text input; buttons, inputs and tables are yours (or `gpui-component` <= 0.5, the last line on gpui 0.2.2; 0.6+ moved to `gpui-pre` 0.3) | the whole web |
| keyboard | per-frame `input()` checks | actions + keymaps, focus handles, built in | DOM events |
| testing | `egui_kittest`: find by label, click, step frames, no window | `TestAppContext` / `VisualTestContext`: `debug_bounds`, `simulate_click`, fake clock | browser test tools; Rust side as plain unit tests |
| maturity | stable, many releases (0.37), large community | pre-1.0 (crates.io `0.2.2` is from Oct 2025; Zed's `main` moves faster), docs thin, API churn | Tauri 2 stable; Leptos / Dioxus pre-1.0 |
| other targets | web (wasm) from the same code | macOS, Linux, Windows | the same page also runs in a normal browser; Leptos runs as wasm in the web view, Dioxus desktop runs native Rust and drives the web view |

**Pick egui** for tools, debug panels, settings screens, in-game overlays, prototypes; when the
fastest path from idea to working window matters more than polish.
**Pick GPUI** for an app you live in for hours, with long lists, lots of text and keyboard
control (an editor, a task manager); when low memory and keyboard-first control are part of the feature.
**Pick the web route** when the look matters most, the team knows CSS, there is already a web
UI (Oxide is Tauri + Svelte for that reason), or the same screen must open in a phone browser.

**Why the track defaults to GPUI, honestly.** In the spike below egui with `glow` opened about
3x faster and idled a few points lower; GPUI had the smallest working set (private memory was
close). The big gaps are the web view and egui's default wgpu renderer, not egui against GPUI.
GPUI wins the pick on what a long-lived, keyboard-driven tool needs: kept views, actions,
keymaps and focus built in, `uniform_list`, text quality, and the learner already has a GPUI
build cache from the Kinetik track. Quest 11 says this trade out loud. egui stays first-class
as a port (world 08) because the model is GUI-free from day one.

## 2. Measured (spike, 2026-10-25)

Same job in each build: every process from `sysinfo 0.39.6` (about 330 on this PC),
refreshed every second with the same `refresh_processes_specifics` call, sorted by CPU. egui
and GPUI draw a virtual list (`TableBuilder::body.rows`, `uniform_list`); the Tauri page is
**not** virtualised and writes all rows as HTML every second, so the comparison is not quite
like for like on CPU. Release builds, `strip = true`, rustc 1.98.1 MSVC, on FOX: Ryzen 7 9800X3D
(8 cores / 16 threads), RTX 5070 Ti plus the CPU's Radeon iGPU (eframe's wgpu asks for the
high-performance adapter; glow and GPUI take what Windows gives them; adapters not recorded).
Memory sums the app and every child process (WebView2 runs six; their working sets double-count
shared DLL pages, so private is the fairer Tauri total). Idle CPU over 30 s with nobody touching
the window; the `sysinfo`-only baseline (no window, same refresh on the same timer) is the cost
of reading the process list. Sources: `docs/spikes/gui-2026-10-25/`.

| build | release exe | first window | working set | private | processes | idle CPU (% of one core) | cold release build |
|---|---|---|---|---|---|---|---|
| egui, eframe default (wgpu) | 14.6 MB | 0.23-0.44 s | 241-242 MB | 478 MB | 1 | 8.1-9.6 | 85 s |
| egui, `glow` renderer | 7.5 MB | 0.23-0.33 s | 79-139 MB | 74-128 MB | 1 | 5.6-6.2 | (shares the egui cache) |
| GPUI 0.2.2 | 11.0 MB | 0.71-0.88 s | 61-64 MB | 74-76 MB | 1 | 9.4-10.7 | 138 s |
| Tauri 2 + plain HTML | 8.6 MB | 0.21-0.33 s | 428-440 MB | 242-253 MB | 7 | 17.1-17.7 | 102 s |
| baseline: sysinfo only, no window | - | - | 10-11 MB | 2 MB | 1 | 7.6-7.8 | - |

Three passes each (two for the baseline), 15 s settle, 30 s sample. "First window" is when
Windows reports a main window handle; for Tauri the page still loads after that.

What it says:
- egui's memory is mostly its **renderer**: eframe 0.37 defaults to wgpu, whose setup commits
  about 480 MB private on this PC (mostly graphics-driver memory, not RAM in use); the `glow`
  (OpenGL) renderer is 74-128 MB.
  Pick the renderer on purpose (`default-features = false, features = ["glow", ...]`).
- GPUI has the smallest working set; private memory ties with egui-glow's low end. It is the
  slowest to show its first frame (it sets up DirectX 11 and DirectWrite first) and idles about
  2-3 points above the no-window baseline. egui-glow idled *below* the baseline: CPU numbers at
  this size are within run-to-run noise of each other, so read them as "a few percent", not a ranking.
- Tauri shows its window fastest (the page is tiny) but needs WebView2's six processes,
  about 245 MB private (430 MB summed working set), and its idle CPU is the highest; part of
  that is the un-virtualised page rebuilding 330 rows of HTML every second.
- Build time is a wash for a learner: every route's first build is one to two and a half minutes.

### 2a. Rebuilding the spike

`egui_tm` (eframe 0.37 + egui_extras 0.37), `egui_glow_tm` (same, `glow` renderer),
`gpui_tm` (gpui `=0.2.2`), `tauri_tm` (tauri 2, a plain `index.html`, `withGlobalTauri`),
`baseline_tm` (sysinfo only), and `measure.ps1`, all in `docs/spikes/gui-2026-10-25/`. Each
app is ~60 lines. `measure.ps1 -Exe <path> -Settle 15 -Window 30` starts the exe, waits for
`MainWindowHandle`, then samples the process tree's `WorkingSet64`, `PrivateMemorySize64` and
`TotalProcessorTime`. Build with one `CARGO_TARGET_DIR` in a temp folder and delete it after
(about 5 GB for all five).

## 3. Engine

No engine change for world 00 or the GPUI worlds. An "egui instead" switch per learner would
need a second flavour axis (today flavours are one global story setting, `plain` / `games`);
not planned: the egui port is its own world (08) that requires the GPUI worlds' model, so a
learner who prefers egui can go 00 → 01 → 08.

## 4. Worlds

Sizing per `docs/CONTENT.md` (exercise 2-8 min / 20-60 XP / 1-3 objectives, read 15 XP,
boss 30-90 min / 300-500 XP). **runs** as in `docs/tracks/gpui.md` (unit, gpui, visual, fake,
live, look).

### World 00: Pick a GUI (`00-pick-a-gui`), built

| slug | new | objectives | min | XP | kind | runs |
|---|---|---|---|---|---|---|
| 01-why-a-new-one | the project; three routes in one line each | quiz: 2 | 3 | 15 | read | - |
| 02-the-row-model | view model, `{:.1}`, bytes to MB | `cpu_text_has_one_decimal`, `memory_text_is_whole_megabytes` | 4 | 25 | exercise | unit |
| 03-immediate-mode | egui: draw all every run, a click is an answer, repaint on demand | quiz: 2 | 3 | 15 | read | - |
| 04-egui-row | `ui.label`, `ui.horizontal`, egui_kittest `get_by_label`, `.rect()` | `row_shows_name_cpu_and_memory`, `cells_sit_side_by_side` | 8 | 40 | exercise | kittest |
| 05-egui-sort-button | `ui.button(..).clicked()` as an `if` | `clicking_cpu_puts_busiest_first` | 5 | 30 | exercise | kittest |
| 06-views-and-notify | GPUI: kept views, `render`, `cx.notify()`, `on_click` closures, `uniform_list`, the costs | quiz: 2 | 4 | 15 | read | - |
| 07-gpui-row | `impl Render`, `div`, `.child`, `.flex().gap_4()`, `debug_selector` | `row_draws_all_three_cells`, `cells_sit_left_to_right` | 8 | 45 | exercise | visual |
| 08-gpui-click | `.id`, `.on_click(cx.listener(..))`, `cx.notify()` (and why a click hides a missing one) | `clicking_cpu_puts_busiest_first`, `new_numbers_redraw_the_list` | 7 | 35 | exercise | visual |
| 09-the-web-route | Tauri: WebView2, commands as JSON, costs; Leptos / Dioxus | quiz: 2 | 4 | 15 | read | - |
| 10-the-numbers | the spike table | quiz: 2 | 3 | 15 | read | - |
| 11-pick-one | when each wins; why this track goes on in GPUI | quiz: 2 | 3 | 15 | read | - |
| 12-boss-one-model-two-faces | `SortBy`, `visible()` (filter + sort), the same list in egui and GPUI | 7 staged | 45 | 350 | boss | unit, kittest, visual |

Requires: `rust/05-traits/21-boss-shape-renderer` (closures, traits, `impl Trait`), same as the
GPUI track. GPUI quests note the MSVC toolchain and the one-time ~2 min / 2 GB build.

### World 01: Processes (`01-processes`), project "The process model", ~12 quests, no GUI

`sysinfo::System`, `refresh_processes_specifics` with only CPU and memory, why CPU% needs two
samples a second apart, CPU% per core vs of the whole machine (`cpu_usage() / cores`), working
set vs private bytes, `Snapshot` diffing (started, exited, by pid), a `ProcessTree` from parent
pids, a `Sampler` trait with a fake for tests, grouping by name (`chrome.exe` x 23).
Naming traps worth a card each: on Windows sysinfo's `memory()` is the working set and
`virtual_memory()` is private bytes; Task Manager's default Memory column is "active private
working set", which matches neither. Whole-machine CPU% is `cpu_usage() / sys.cpus().len()`
(needs a CPU refresh first). Boss: a
`Snapshot` → `Vec<ProcessRow>` pipeline that sorts, filters, groups, and keeps the same rows
stable across refreshes. runs: unit + one live.

### World 02: The table (`02-table`, GPUI), ~13 quests

`uniform_list` with 400 rows, fixed-width columns, header click sorts and shows an arrow,
selection by pid (not index), keyboard actions (`up`, `down`, `home`, `end`, `delete`) with
`actions!` and a keymap, a search box with focus, highlight matching text. GPUI 0.2.2 has no
text-input element (Zed's `examples/input.rs` is ~750 lines: `EntityInputHandler`, selection,
IME), so the search box is a 2-3 quest arc on a starter-provided input, or `gpui-component` 0.5's
`Input`; decide in a spike first. Boss: the full list
window, driven only by the keyboard in a `simulate_keystrokes` test.

### World 03: Live and light (`03-live`, GPUI), ~12 quests

Sampling on `background_spawn`, `cx.spawn` to bring results back, notify only on change,
selection survives a refresh, refresh rate setting, pause when minimised (GPUI 0.2.2 has no
`is_minimized`: Win32 `IsIconic` on the raw handle, or pause on `observe_window_activation`),
budgets: idle CPU and memory are machine-dependent, so they are `live` objectives measured
against a baseline taken in the same run, never hard numbers; first frame time measured. Boss: a live
list that stays under budget.

### World 04: End task (`04-actions`), ~11 quests

`TerminateProcess` behind an `EndTask` trait (fake in tests), a confirm dialog, access denied
and why (elevation, protected processes), open file location (`explorer /select,`), copy
command line, set priority, the "kill the hung `java.exe` server" story. Boss: the context
menu with every action, graded through the fake.

### World 05: Details (`05-details`), ~12 quests

A side panel for the selected process: CPU and memory graphs (`canvas`, history ring buffer),
threads, command line, start time, user, GPU per process (PDH "GPU Engine" counters: a rate,
so two collects; instances `pid_N_..._engtype_X` come and go; Task Manager shows the max across
engine types, not the sum). Boss:
the details panel with live graphs.

### World 06: Startup and services (`06-startup`), ~10 quests

Startup entries (HKCU / HKLM Run keys, WOW6432Node too, via `winreg`, and the Startup folder);
enable / disable the way Task Manager does, never by deleting Run values: binary values under
`...\Explorer\StartupApproved\Run` (`Run32`, `StartupFolder`), first byte `02` enabled, `03`
disabled; HKLM needs elevation. A fake registry in tests, services list (`windows` service control manager APIs), status and
start type. Boss: a Startup tab.

### World 07: Ship (`07-ship`), ~10 quests

Single instance, a global hotkey to summon it (`global-hotkey`; Ctrl+Shift+Esc stays Windows'),
tray icon (GPUI has no tray API; `tray-icon` and `global-hotkey` deliver on channels polled from a
`cx.spawn` timer, as in the Kinetik track: spike it first), start minimised, gpui's
`test-support` moved to `[dev-dependencies]` so the shipped exe has no test code, release profile (thin LTO, `strip`, `panic = "abort"`), measure
cold start against Task Manager, a zip. Boss: release build that opens under the world 03 budget.

### World 08: The egui port (`08-egui-port`), ~10 quests, requires world 01

Same `taskman-model` crate, egui face: `eframe` with the `glow` renderer chosen on purpose,
`TableBuilder` with `body.rows` for 400 rows, `request_repaint_after(1 s)`, header buttons,
search with `text_edit_singleline`, keyboard via `ctx.input`, `egui_kittest` throughout. Boss:
the egui task manager, and the learner measures it against their GPUI build.

## 5. Risks

- GPUI 0.2.2 on crates.io lags Zed `main`; pin `=0.2.2` (shared cache with the Kinetik track).
- GPUI 0.2.2 redraws a window after a mouse-down on any element with `on_click` or an active
  style (`div.rs:2143`), even without `cx.notify()`, so a test of notify must change the view
  outside an event (quest 08 uses `set_processes`), and a missing notify is a bug that comes and goes.
- `debug_bounds` is never cleared in GPUI 0.2.2: test "is gone" on state or a fresh window
  (the boss does the latter for `row-2`).
- egui's renderer choice changes memory by about 6x; world 08 teaches it, world 00 only mentions it.
- Numbers in quest 10 are from one PC (named in the lesson) and CPU figures are near noise;
  re-run the spike (section 2a) before changing them.
- content-check reuses a test binary when a starter and its solution share a package name in
  one `CARGO_TARGET_DIR`; it already uses two dirs, so keep it that way (copies made by hand need
  their own name or dir).
