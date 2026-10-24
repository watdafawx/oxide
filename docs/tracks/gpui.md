# Kinetik in Rust (GPUI)

Planning doc for a new track that rebuilds **Kinetik Stats** (github.com/WastedDesigner/Kinetik)
in Rust with **GPUI**, Zed's UI library. Not content. Format and rules: `docs/CONTENT.md`;
runners and tracks: `docs/TRACKS.md`. Research date: 2026-10-22.

## 0. What we are rebuilding

Kinetik Stats v4.0.0 is a C# WinForms app (one 705 KB `KinetikStats.cs` plus `SuiteSupport.cs`,
.NET Framework 4.7.2). It puts a strip of live stats on the Windows taskbar and adds a desktop
widget, a game overlay with an FPS counter, a dashboard, screen time, notifications and a
self-updater. Its 44 stats come from performance counters, NVML (NVIDIA), LibreHardwareMonitor +
PawnIO (CPU temperature, AMD/Intel GPUs), ETW (frame times, the PresentMon method) and WinRT
(media, Bluetooth batteries).

**Project spine:** the whole track is one project, `kinetik`, that grows from a 360x120 window
to a release build of Kinetik Stats. Every world adds one visible part of the real app; every
quest adds one feature to that part. Names come from Kinetik: `StatKind`, `Reading`, `Pill`,
`StatsBar`, `History`, `SensorHub`, `HoverCard`, `Widget`, `Overlay`, `FrameTimes`, `Settings`.

| Kinetik feature | world | Rust / GPUI idea it teaches |
|---|---|---|
| a window at all | 00 Setup | `Application`, `open_window`, `Render` |
| 44 stats, `--°C`, `8.0 GB`, warning colours, min/avg/max | 01 Readings | plain Rust model, `VecDeque`, no UI |
| the bar's look: pills, dividers, one/two lines, colours | 02 Views | `div()`, flex, `px`, `rgb`, `RenderOnce`, `.children` |
| it updates every second, only redraws on change | 03 Live | `Entity`, `cx.notify`, `cx.spawn`, timers, `WeakEntity` |
| CPU, RAM, network, disk, GPU, battery, ping, temps | 04 Sensors | `sysinfo`, `nvml-wrapper`, `windows`, `wmi`, `background_spawn` |
| sits on the taskbar, matches light/dark, hides in fullscreen | 05 Taskbar | `WindowOptions`, raw HWND, Win32 behind a trait |
| click, hover card + graph, right-click menu, tray icon | 06 Interaction | `on_click`, `tooltip`, `canvas`, `actions!`, `deferred` + `anchored` |
| Settings window, toggles, sliders, Arrange drag and drop, profiles | 07 Settings | second window, `Global`, `observe_global`, `uniform_list`, `on_drag`/`on_drop`, serde |
| desktop widget: bars, graphs, snapping, click-through | 08 Widget | `relative`, `start_window_move`, extended window styles |
| game overlay, FPS, 1% lows, frame-time graph, hotkeys, CSV | 09 Overlay | frame-time maths, ETW, `global-hotkey` |
| dashboard window + web dashboard with a secret key | 10 Dashboard | `grid`, `PathBuilder` arcs, axum on its own runtime |
| screen time, hot-for-30-seconds notifications | 11 Screen time | idle detection, day files, debounce, toasts |
| single instance, startup, themes, updater with checksum, zip | 12 Ship | named mutex, `winreg`, `semver`, `sha2`, `zip` |

Kept for a later side world (section 6): Now playing, Bluetooth batteries, Timeline, Crashes,
companion shared memory, multi-taskbar edge cases.

## 1. Toolchain and runner

### Pins

| piece | pin | note |
|---|---|---|
| Rust | stable, `x86_64-pc-windows-msvc` | GPUI needs MSVC + Windows SDK (DirectX 11, DirectWrite) |
| gpui | **`=0.2.2`** (crates.io, 2025-10-22, newest published) | edition 2024. Sub-crates are published as `gpui_collections`, `gpui_util`, `gpui-macros`, ... Zed's git `main` has moved on; we stay on crates.io for a reproducible offline cache (risk R4). |
| gpui features | `["test-support"]` in `[dependencies]` of every quest | one feature set for every quest, so the shared build cache builds gpui once. `test-support` pulls `wayland`/`x11` features, which are Linux-only deps and compile to nothing on Windows. |
| sysinfo | 0.39.6 | CPU, RAM, disks, networks, processes, uptime |
| nvml-wrapper | 0.13.0 | NVIDIA GPU, no admin |
| windows | 0.62.2 | Win32: taskbar rect, window styles, power status, foreground window, mutex |
| wmi | 0.18.4 | ACPI thermal zone fallback for CPU temperature |
| ferrisetw | 1.2.0 | ETW present events (admin) |
| tray-icon | 0.26.1 | tray icon and its menu |
| global-hotkey | 0.9.0 | Ctrl+Shift+F10 / F11 from inside a game |
| tauri-winrt-notification | 0.8.1 | Windows toasts |
| serde + serde_json | 1.x / 1.0.154 | settings, profiles, screen-time day files, dashboard JSON |
| axum + tokio | 0.8.9 / 1.54.1 | web dashboard (world 10 only) |
| winreg | 0.56.0 | run at startup (HKCU Run key) |
| semver, sha2, zip, tempfile | 1.0.28, 0.11.1, 9.0.5, 3.27.0 | updater, packaging, tests |

Crates enter a starter only in the world that teaches them (CONTENT.md: "no external crates
unless the quest is about them"), so world 01 builds in seconds and world 00 is the only cold
gpui build a learner waits for.

### Runner

The existing `cargo` runner. No engine change needed for worlds 00-03 and 06-08.

- `track.toml`: `title = "Kinetik in Rust"`, `blurb = "Rebuild Kinetik Stats with GPUI: a taskbar bar, a widget, a game overlay, shipped."`, `runner = "cargo"`.
- Package names `kinetik_<world>_<quest>` (`kinetik_03_spawn_a_loop`), unique across the catalog.
- Every starter is a full copy of the growing project: world N's quest 01 starts from world N-1's
  boss solution plus the new world's files. A skipped world never blocks the next one.
- Measured (spike, 2026-10-22, rustc 1.98.1 MSVC, 16 threads, debug): first `cargo build` with
  gpui 0.2.2 + `test-support` **1 min 40 s including downloads, 1.9 GB `target`**; adding
  `windows`, `tray-icon`, `global-hotkey` +31 s, 2.8 GB total. Warm rebuild after an edit
  **3.5 s**; warm `cargo test` **4.2 s**. A *new* quest crate on the same `CARGO_TARGET_DIR` reuses
  gpui: first test run **4-9 s**. Debug exe 26 MB. Setup quest 01 quotes "about 2 minutes, 2 GB".

### How quests are graded (the design lever)

GPUI's `test-support` gives a headless `TestPlatform`, so most UI is gradeable without a screen.
Every quest uses the cheapest row that proves its one idea:

| runs | what it is | used for |
|---|---|---|
| `unit` | plain `#[test]`, no gpui types | formatting, maths, parsers, state machines, layout maths |
| `gpui` | `#[gpui::test] fn x(cx: &mut TestAppContext)`: `cx.new(...)`, `read_with`, `update`, `dispatch_action`, `simulate_keystrokes`, `executor().advance_clock(..)`, `run_until_parked()` | entities, notify, timers, actions, keybindings, globals |
| `visual` | `cx.add_window_view(...)` returns a `VisualTestContext`; elements carry `.debug_selector(\|\| "pill-cpu".into())`; tests use `debug_bounds("pill-cpu")`, `simulate_click`, `simulate_mouse_move`, `simulate_resize` | "is it drawn", "is it right of / under", "is it half as wide", clicks and hovers |
| `fake` | Win32 / OS calls sit behind a trait (`WinApi`, `Launcher`, `Clock`); tests pass a fake that records calls | topmost, click-through, task manager, mutex, registry |
| `live` | touches this PC (sysinfo, NVML, taskbar). Tolerant asserts (`0.0..=100.0`), and a missing device prints why and passes with `skip:` in the label | first contact with each real sensor |
| `look` | after the objectives: "run it, answer 1-2 questions" quiz in the lesson's Check step | colours, transparency, the real taskbar, the overlay over a game |

Rules that follow:
- **View model first.** Each view gets a pure function that decides what to draw
  (`pills(&order, &snapshot, &limits) -> Vec<Pill>`); `render` only maps it to elements. Text is
  graded on the model, position and size on `debug_bounds`. Pixel colours are never graded.
- **Admin-only features** (ETW frame times, some temperatures) are graded on recorded fixtures
  (a PresentMon CSV in `fixtures/`), and the live version is a `live` objective that passes
  with `skip: needs administrator` when Oxide is not elevated. Oxide never asks for admin.
- `look` quizzes never gate progress alone (same rule as the Fabric track).
- GPUI 0.2.2 never clears the `debug_bounds` map between frames: an element that disappears
  still looks drawn. Test "is gone" on state, or open a fresh window with the setting already
  applied; "is drawn" and "where" are reliable.
- Opening a window whose view reads another entity, from inside that entity's update (an action
  handler), panics: defer it with `cx.defer(..)` (World 08's `toggle_widget`).
- An observer registered during app startup (`App::observe_global` in `Application::run`) does not
  hear changes made before that startup code returns. Save or apply such changes by hand (World 10's
  first-start key).
- The bar notifies only when what it draws changes, so `cx.observe(&bar, ..)` misses a value
  that holds steady. Anything that must look every tick runs its own timer (World 11's `watch_alerts`).

## 2. Requires (cross-track)

The track assumes the Rust track up to the world each part needs. Rust terms from
`docs/tracks/rust-terms.md` up to the required quest count as taught; GPUI and crate terms are
taught here and go in `docs/tracks/gpui-terms.md`.

| first quest | requires |
|---|---|
| `00-setup/01-what-gpui-is` | `rust/05-traits/21-boss-shape-renderer` (traits, closures, `impl Trait`) |
| `03-live/01-state-in-a-view` | `02-views/12-boss-static-bar`, `rust/06-smart-pointers/17-boss-tree-editor`, `rust/08-async/11-boss-mod-downloader` |
| `04-sensors/01-what-needs-admin` | `03-live/13-boss-live-bar`, `rust/07-concurrency/11-boss-parallel-file-hasher` |
| `10-dashboard/09-web-server` | `10-dashboard/08-snapshot-json`, `rust/10-apps/06-axum-json-api` |

Everything else: previous quest (default).

## 3. Worlds

Columns: **objectives** are test names (suffix-matched). **runs** from section 1. XP and minutes
follow CONTENT.md sizing (exercise 2-8 min / 20-60 XP / 1-3 objectives, read 2-5 min / 15 XP,
boss 30-90 min / 300-500 XP / 4-10 staged objectives). ⚡ = quick win (lower XP and minutes,
about every third quest). First quest of each world is under 5 minutes. **New** is the one idea
the quest introduces (that row's entry in the term ledger).

### World 00: Setup (`00-setup`), project "A window that says Kinetik", 6 quests

Setup world without a boss, finale is a `look` read (same exception as the Fabric Setup world).

| slug | new | objectives | min | XP | kind | runs |
|---|---|---|---|---|---|---|
| 01-what-gpui-is | GPUI: Rust code describes the window every frame, the GPU draws it; what the first build downloads and how long it takes (about 2 minutes, 2 GB) | quiz: 2 | 3 | 15 | read | - |
| 02-msvc-toolchain | the MSVC toolchain and Windows SDK, `rustup show` reads `x86_64-pc-windows-msvc` | quiz: 2 | 3 | 15 | read ⚡ | - |
| 03-first-build | a dependency line `gpui = { version = "=0.2.2", features = ["test-support"] }`; saving starts the long first build | `gpui_is_in_dependencies` | 4 | 20 | exercise ⚡ | unit |
| 04-hello-window | `Application::new().run(\|cx\| ...)`, `cx.open_window`, quit on close; size from `fn bar_size() -> Size<Pixels>` | `window_is_360_by_120` | 6 | 35 | exercise | unit |
| 05-a-view | `struct StatsBar`, `impl Render`, `div().child("CPU 12%")`, `.debug_selector` | `bar_draws_a_cpu_pill` | 6 | 40 | exercise | visual |
| 06-it-runs | `cargo run`, the event loop, render when needed, closing quits | quiz: 2 | 3 | 15 | read ⚡ | - |

### World 01: Readings (`01-readings`), project "The stat model", 13 quests

No gpui in this world's tests: pure Rust, builds in seconds. The boss is the model every later view draws.

| slug | new | objectives | min | XP | kind | runs |
|---|---|---|---|---|---|---|
| 01-what-a-stat-is | `enum StatKind` (9 of Kinetik's 44: CPU usage, CPU temp, RAM, GPU usage, GPU temp, down, up, ping, battery), `label()` | `gpu_kinds_say_gpu`, `net_ping_and_battery_labels` | 3 | 20 | exercise ⚡ | unit |
| 02-a-reading | `struct Reading { kind, value: Option<f64> }`; no value shows `--` | `missing_temp_shows_dashes` | 5 | 30 | exercise | unit |
| 03-units | `enum Unit`, `StatKind::unit()` | `temps_are_celsius`, `network_and_ping_units` | 5 | 30 | exercise | unit |
| 04-format-percent | `{:.0}` rounding: 12.6 shows `13%` | `percent_rounds_to_whole` | 4 | 25 | exercise ⚡ | unit |
| 05-format-bytes | divide by 1024 in a `while`: 8 589 934 592 shows `8.0 GB` | `bytes_pick_the_right_unit`, `small_counts_stay_in_bytes` | 7 | 45 | exercise | unit |
| 06-format-speed | speeds reuse `format_bytes` + `/s`, with ↓ and ↑ | `download_shows_down_arrow`, `upload_shows_up_arrow` | 5 | 30 | exercise | unit |
| 07-warning-limits | `struct Limits`, match on `(value, limit)`; CPU at 90 °C is hot | `cpu_at_90_degrees_is_hot`, `fine_or_unknown_is_not_a_warning` | 6 | 40 | exercise | unit |
| 08-low-is-bad | `enum WarnWhen { Above, Below }`: battery warns when low | `battery_at_15_warns`, `battery_at_80_is_fine` | 4 | 25 | exercise ⚡ | unit |
| 09-history | `VecDeque`, `push_back` + `pop_front` keeps the last 60 readings | `history_keeps_last_60` | 7 | 45 | exercise | unit |
| 10-min-avg-max | `History::summary() -> Option<Summary>` for the hover card | `min_avg_max_of_10_20_30`, `summary_of_empty_is_none` | 6 | 40 | exercise | unit |
| 11-bar-order | `enum Slot { Stat(StatKind), Divider(DividerStyle) }`, `Vec<Slot>` is the bar | `default_order_starts_with_cpu`, `default_order_has_three_line_dividers` | 4 | 25 | exercise ⚡ | unit |
| 12-move-a-slot | `slice.swap` with a bounds check (the Arrange page's Alt+↑) | `move_up_swaps_with_previous`, `first_slot_cannot_move_up` | 6 | 40 | exercise | unit |
| 13-boss-stat-model | `bar_items(&order, &snapshot, &limits) -> Vec<BarItem>`, `two_line_columns`, `hover_summary` | `formats_each_kind`, `missing_shows_dashes`, `hot_cpu_is_flagged`, `low_battery_is_flagged`, `dividers_pass_through`, `two_line_pairs_stats`, `hover_summary_from_history` | 45 | 350 | boss | unit |

### World 02: Views (`02-views`), project "The bar, drawn", 12 quests

Static data (a fixed `Snapshot`), real GPUI elements. Every element under test has a `debug_selector`.

| slug | new | objectives | min | XP | kind | runs |
|---|---|---|---|---|---|---|
| 01-a-row | `.flex()`, `.flex_row()`, `.gap_2()`: boxes stack by default | `ram_pill_is_right_of_cpu_pill` | 4 | 25 | exercise ⚡ | visual |
| 02-pixels | `.h(px(32.))`, `.items_center()` (*starter*) | `bar_is_32_high` | 5 | 30 | exercise | visual |
| 03-colours | `rgb(0x..)`, `.bg`, `.text_color`, `value_colour(warning)` | `warning_colour_is_kinetik_red`, `normal_colour_is_white` | 4 | 25 | exercise ⚡ | unit |
| 04-children-from-a-list | `.children(pills.iter().map(..))`, pills come from the World 01 model | `one_element_per_pill` | 6 | 40 | exercise | visual |
| 05-a-pill-component | `#[derive(IntoElement)]` + `impl RenderOnce for PillView`, named label and value | `pill_view_draws_label_then_value` | 7 | 45 | exercise | visual |
| 06-labels-or-not | `.when(show_label, \|el\| ..)`, `LabelMode` | `text_mode_draws_label`, `off_mode_draws_no_label` | 4 | 25 | exercise ⚡ | visual |
| 07-dividers | `divider_width` by style; `.into_any_element()` (*starter*) mixes pills and dividers | `dot_divider_is_4_wide`, `blank_divider_is_8_wide` | 6 | 40 | exercise | visual |
| 08-two-lines | `.flex_col()` columns, `.children(Option)` | `two_line_layout_stacks_pairs` | 7 | 45 | exercise | visual |
| 09-the-pill-background | `Rgba.a` alpha from the opacity setting; `.rounded(px(8.))` (*starter*) | `opacity_50_sets_half_alpha`, `full_opacity_is_solid` | 5 | 30 | exercise | unit |
| 10-numbers-that-sit-still | `.min_w(px(48.))`, monospace is not enough | `value_box_width_does_not_jump` | 6 | 40 | exercise | visual |
| 11-text-size | `.text_size(..)`, `f32::clamp` | `font_size_is_clamped`, `bigger_font_makes_wider_label` | 5 | 30 | exercise | unit, visual |
| 12-boss-static-bar | the whole bar from the model: pill function, one-line and two-line children | `every_pill_drawn`, `pills_in_bar_order`, `dividers_drawn`, `labels_follow_mode`, `values_sit_still`, `two_line_mode_stacks` | 45 | 350 | boss | visual |

### World 03: Live (`03-live`), project "The bar ticks", 13 quests

| slug | new | objectives | min | XP | kind | runs |
|---|---|---|---|---|---|---|
| 01-state-in-a-view | the view keeps `snapshot` and `items` in fields; `render` (now in `view.rs`) draws from them | `bar_shows_its_snapshot` | 4 | 25 | exercise ⚡ | visual |
| 02-entity | `cx.new(\|_\| ..)` gives an `Entity<StatsBar>`; `read_with` | `entity_holds_bar_state` | 6 | 40 | exercise | gpui |
| 03-update-and-notify | `entity.update(cx, \|bar, cx\| ..)` + `cx.notify()`; without it the screen stays stale | `new_reading_redraws_bar` | 6 | 40 | exercise | gpui |
| 04-a-fake-sensor | `trait Sensor { kind, read }`, a `ScriptedSensor` for tests | `scripted_sensor_plays_in_order` | 4 | 25 | exercise ⚡ | unit |
| 05-spawn-and-keep | `cx.spawn(async move \|this, cx\| ..)`; a dropped `Task` is cancelled, keep it in `_poll` | `started_bar_reads_its_sensors` | 6 | 40 | exercise | gpui |
| 06-a-timer | `cx.background_executor().timer(interval).await`, never `thread::sleep`; tests use `advance_clock` | `first_read_waits_one_interval` | 6 | 40 | exercise | gpui |
| 07-every-second | `loop` + `WeakEntity`: `this.update(..)` fails once the bar is gone, `break` | `bar_reads_every_second` | 7 | 45 | exercise | gpui |
| 08-update-interval | interval 0.5 to 5 s, `u64::clamp`, `Duration::from_millis` | `interval_clamps_to_half_second` | 4 | 25 | exercise ⚡ | unit |
| 09-redraw-only-on-change | compare new items with old, `notify` only when they differ | `same_values_do_not_redraw` | 6 | 40 | exercise | gpui |
| 10-history-per-stat | `HashMap<StatKind, History>`, `entry(..).or_insert_with(..)` | `ten_ticks_fill_ten_points` | 6 | 40 | exercise | gpui |
| 11-real-cpu | `sysinfo::System`, `refresh_cpu_usage` + `global_cpu_usage`; the first read is 0 | `cpu_usage_is_a_percentage` | 6 | 40 | exercise | live |
| 12-real-ram | `refresh_memory` + `used_memory`, in bytes | `ram_used_is_within_total` | 4 | 25 | exercise ⚡ | live |
| 13-boss-live-bar | the bar ticks on real CPU and RAM, scripted rest, clamped interval, history, no redraw when nothing changed, stops when dropped | `bar_starts_with_dashes`, `first_tick_fills_cpu`, `ticks_follow_interval`, `history_grows_per_tick`, `unchanged_tick_skips_redraw`, `closing_stops_the_loop` | 50 | 400 | boss | gpui |

### World 04: Sensors (`04-sensors`), project "SensorHub", 14 quests

| slug | new | objectives | min | XP | kind | runs |
|---|---|---|---|---|---|---|
| 01-what-needs-admin | which stats Windows gives freely, which need admin or a driver (PawnIO), which crate reads each; never show 0 for unknown | quiz: 3 | 4 | 15 | read | - |
| 02-network-speed | bytes since the last refresh over `as_secs_f64()`; a speed needs two reads | `speed_is_bytes_over_seconds` | 6 | 40 | exercise | unit |
| 03-skip-loopback | skip loopback, tunnels, `vEthernet` and filter-driver copies: `iter().any` | `loopback_is_not_counted` | 4 | 25 | exercise ⚡ | unit |
| 04-free-disk | new stat `DiskFree` (label `C:`, warns below 10 GB); `sysinfo::Disks`, `find(..)?` | `c_drive_has_free_space` | 5 | 30 | exercise | live |
| 05-gpu-via-nvml | `Nvml::init()` kept as an `Option`, chain `.ok()?`; no NVIDIA card is `None`, not a crash | `no_nvidia_means_none`, `nvidia_usage_is_a_percentage` | 8 | 60 | exercise | unit, live |
| 06-pick-the-gpu | Auto: NVIDIA first (`position`), else the most VRAM (`max_by_key`) | `auto_prefers_nvidia`, `else_most_vram_wins` | 6 | 40 | exercise | unit |
| 07-cpu-temp-fallback | ACPI tenths of kelvin to °C, ignore dummy values and impossible numbers; `ThermalSensor` queries WMI (*starter*) | `kelvin_tenths_to_celsius`, `dummy_27_8_is_ignored` | 6 | 40 | exercise | unit |
| 08-battery | `unsafe` + `GetSystemPowerStatus`; flag 128 or 255 % means no battery, bit 8 is charging | `this_pc_power_status_reads`, `no_battery_flag_means_none` | 7 | 45 | exercise | unit, live |
| 09-ping | time a TCP connect with `connect_timeout` (no admin, unlike Kinetik's ICMP); no answer is `None` | `local_server_answers`, `unreachable_host_is_none` | 7 | 45 | exercise | unit |
| 10-uptime | `/` and `%` into days, hours, minutes: `3d 5h` | `uptime_shows_days_and_hours` | 4 | 25 | exercise ⚡ | unit |
| 11-lazy-sensors | read only stats on the bar: `wanted_kinds` `HashSet`, `continue` | `hidden_gpu_is_never_read` | 6 | 40 | exercise | gpui |
| 12-off-the-ui-thread | sensors read on `background_executor().spawn`; the compiler demands `trait Sensor: Send` (starter does not compile) | `readings_arrive_from_the_background` | 8 | 60 | exercise | gpui |
| 13-why-its-missing | `enum Missing { NeedsAdmin, NoDevice, Timeout }`, `Sensor::missing()` default method, WMI access denied | `cpu_temp_without_admin_says_why` | 4 | 25 | exercise ⚡ | unit |
| 14-boss-sensor-hub | `read_sensors` (lazy, with reasons), `apply` (snapshot, history, reasons, redraw on change), `hover_text` | `hub_reads_cpu_and_ram`, `hidden_stats_cost_nothing`, `gaps_come_with_a_reason`, `bar_shows_hub_readings`, `bar_remembers_why`, `unchanged_reads_skip_redraw`, `network_speed_needs_two_reads` | 60 | 450 | boss | unit, gpui, live |

### World 05: On the taskbar (`05-taskbar`), project "Docked bar", 11 quests

Win32 calls live behind `trait WinApi`; tests use `FakeWinApi`.

| slug | new | objectives | min | XP | kind | runs |
|---|---|---|---|---|---|---|
| 01-window-options | `WindowOptions { titlebar: None, kind: WindowKind::PopUp, is_movable: false, is_resizable: false, focus: false, .. }` | `bar_window_has_no_titlebar`, `bar_window_is_popup` | 4 | 25 | exercise ⚡ | unit |
| 02-see-through | `WindowBackgroundAppearance::Transparent`, only the pill is drawn (spike R3 measured it exact) | `bar_background_is_transparent` | 4 | 25 | exercise | unit |
| 03-find-the-taskbar | `FindWindowW("Shell_TrayWnd")` + `FindWindowExW("TrayNotifyWnd")`; `GetWindowRect`, monitor and DPI (*starter*) | `taskbar_found_on_this_pc` | 7 | 45 | exercise | live |
| 04-dock-maths | bar rect left of the tray, `Shift left`; `Rect::intersect` with the monitor, under 4 px visible is auto-hide | `bar_sits_left_of_tray`, `shift_left_moves_it_left`, `auto_hidden_taskbar_hides_bar` | 7 | 45 | exercise | unit |
| 05-dpi | physical to logical pixels: divide by the monitor's scale; `main.rs` now docks for real | `bounds_scale_at_150_percent` | 5 | 30 | exercise ⚡ | unit |
| 06-stay-on-top | `trait WinApi`, `FakeWin` records calls, `RealWin` calls `SetWindowPos(HWND_TOPMOST)`; raise on open and when the taskbar comes to the front; HWND via `raw-window-handle` | `topmost_set_after_open`, `topmost_set_again_after_taskbar_click` | 7 | 45 | exercise | fake |
| 07-light-or-dark | `window.appearance()`, `WindowAppearance`, `\|` in a match arm | `dark_taskbar_uses_white_text` | 5 | 30 | exercise | unit |
| 08-hide-in-fullscreen | a window covering its monitor hides the bar on that monitor only | `fullscreen_game_hides_bar`, `fullscreen_on_other_monitor_keeps_bar` | 6 | 40 | exercise | unit |
| 09-every-taskbar | `Shell_SecondaryTrayWnd` (*starter*), "Show on all taskbars" filter; `display_id` trap from R3 | `two_taskbars_two_bars`, `primary_only_by_default` | 6 | 40 | exercise | unit |
| 10-taskbar-moved | compare with the last taskbar, move only on a change (`move_to`, never resize through Win32) | `moved_taskbar_redocks_bar` | 4 | 25 | exercise ⚡ | fake |
| 11-boss-docked-bar | one `Docker::tick` a second: dock and raise on a change, show or hide for auto-hide and fullscreen | `window_options_right`, `docks_left_of_tray`, `stays_topmost`, `hides_for_fullscreen`, `redocks_after_move`, `hidden_taskbar_hides_bar`, `scales_with_dpi` | 50 | 400 | boss | fake, unit |

### World 06: Interaction (`06-interaction`), project "Clicks, hovers and menus", 12 quests

| slug | new | objectives | min | XP | kind | runs |
|---|---|---|---|---|---|---|
| 01-a-click | `view.rs` sends every pill through `interactive_pill`; `.id(..)` makes a `Stateful<Div>`, `.on_click(cx.listener(..))` | `clicking_cpu_pill_is_seen` | 4 | 25 | exercise ⚡ | visual |
| 02-left-click-action | `enum LeftClick { TaskManager, Settings, Nothing }`; a `Launcher` trait runs `taskmgr`, `FakeLauncher` records it | `left_click_opens_task_manager` | 5 | 30 | exercise | visual, fake |
| 03-hover | `.hover(\|style\| ..)` look, `.on_hover(..)` state, clear only your own kind | `hover_sets_hovered_stat` | 6 | 40 | exercise | visual |
| 04-hover-card | `HoverCard` view, `card_for`; `.tooltip(move \|_, cx\| cx.new(..).into())`, 500 ms delay | `hover_shows_card` | 7 | 45 | exercise | visual |
| 05-graph-points | pure: `History` to points inside a box (newest on the right, max at the top, flat in the middle) | `points_span_full_width`, `highest_value_at_top` | 6 | 40 | exercise | unit |
| 06-draw-the-graph | `canvas(..)` + `PathBuilder::stroke`, `line_to`, `window.paint_path` | `card_has_graph` | 8 | 60 | exercise | visual |
| 07-top-processes | `sort_by_key` + `Reverse`, `truncate(5)`; processes by memory (CPU needs two samples) | `top_five_by_memory` | 4 | 25 | exercise ⚡ | unit |
| 08-actions | `actions!(kinetik, [OpenSettings, OpenTaskManager, Exit])`, `.on_action(cx.listener(..))`, focus (`track_focus`, `window.focus`) | `open_settings_action_is_handled` | 7 | 45 | exercise | gpui |
| 09-right-click-menu | `on_mouse_down(MouseButton::Right, ..)` + `deferred(anchored().position(at)..)`, `on_mouse_down_out` closes | `right_click_opens_menu`, `click_elsewhere_closes_it` | 8 | 60 | exercise | visual |
| 10-menu-items | a menu row closes the menu and `window.dispatch_action(action.boxed_clone(), cx)` | `settings_row_dispatches_open_settings` | 5 | 30 | exercise ⚡ | visual |
| 11-tray-icon | `tray-icon`: icon + menu in `main.rs`; tray events map to actions (release, not press; menu ids) | `tray_click_maps_to_open_settings`, `tray_menu_maps_by_id` | 8 | 60 | exercise | unit |
| 12-boss-interactive-bar | `interactive_pill` and `bar_layer` from scratch: click, hover, card with processes, actions, menu; tray in `main.rs` | `left_click_runs_action`, `hover_card_appears`, `card_shows_summary`, `card_lists_top_processes`, `menu_opens_on_right_click`, `menu_exit_quits`, `tray_open_settings` | 55 | 400 | boss | visual, gpui, unit |

### World 07: Settings (`07-settings`), project "Settings window", 16 quests

GPUI 0.2.2 ships no text box, toggle or slider widgets; the learner builds the few Kinetik needs.

| slug | new | objectives | min | XP | kind | runs |
|---|---|---|---|---|---|---|
| 01-a-second-window | `open_settings_window`: keep the `WindowHandle`, `handle.update(..)` fails once closed, `activate_window`; `if let .. &&` | `opening_twice_keeps_one_window` | 4 | 25 | exercise ⚡ | gpui |
| 02-settings-struct | `Settings` with `Serialize, Deserialize` and `#[serde(default)]`; Kinetik's defaults | `defaults_match_kinetik` | 4 | 25 | exercise ⚡ | unit |
| 03-a-global | `impl Global for Settings` (*starter*), `cx.set_global`, `cx.global::<Settings>()` | `bar_reads_global_interval` | 6 | 40 | exercise | gpui |
| 04-live-preview | `cx.observe_global::<Settings>(..)`, keep the `Subscription`, `Settings::update_global` | `changing_font_size_redraws_bar` | 7 | 45 | exercise | gpui |
| 05-sidebar-pages | `enum Page`, a sidebar, page is window state, not a setting | `clicking_arrange_shows_arrange_page` | 5 | 30 | exercise ⚡ | visual |
| 06-a-toggle | `Toggle` component (`RenderOnce`, a `ToggleHandler` closure, `justify_end`) | `toggle_turns_battery_on` | 6 | 40 | exercise | visual |
| 07-slider-maths | pure: share across the track, into the range, snapped to step | `drag_to_middle_is_half`, `value_snaps_to_step` | 6 | 40 | exercise | unit |
| 08-a-slider | `on_mouse_down` + `on_mouse_move` with `pressed_button`; track bounds from a `canvas` | `dragging_slider_changes_gap` | 7 | 45 | exercise | visual |
| 09-colour-swatches | 15 swatches, `update_global` inserts the stat's colour; the bar paints it | `picking_swatch_sets_cpu_colour` | 4 | 25 | exercise ⚡ | visual |
| 10-typing-an-address | `track_focus`, `on_key_down`, `keystroke.key` / `key_char`; a minimal text box | `typing_sets_ping_address`, `backspace_removes_last_char` | 8 | 60 | exercise | visual |
| 11-a-long-list | `uniform_list` + `cx.processor` for the Arrange list | `only_visible_rows_are_built` | 7 | 45 | exercise | visual |
| 12-drag-to-reorder | `move_slot` (`remove` + `insert`); `on_drag` / `on_drop` (*starter*) | `drop_cpu_after_gpu_reorders`, `dragging_a_row_moves_it` | 8 | 60 | exercise | unit, visual |
| 13-keys-to-reorder | `KeyBinding::new("alt-up", MoveUp, Some("ArrangeList"))`, `cx.bind_keys`, `.key_context` | `alt_up_moves_selected_row` | 6 | 40 | exercise | gpui |
| 14-save-to-disk | `create_dir_all`, `serde_json::to_string_pretty`, write a temp file then rename | `save_then_load_round_trips` | 6 | 40 | exercise | unit |
| 15-fix-bad-values | clamp out-of-range values on load, keep an unreadable file as `.bak` (Kinetik v3.1.2) | `interval_99_is_clamped`, `unreadable_file_is_backed_up` | 5 | 30 | exercise ⚡ | unit |
| 16-boss-settings | `apply_settings` (the bar follows), `persist_settings` (saved on change), `Profiles` (Gaming / Work) | `one_settings_window`, `toggle_hides_stat_on_bar`, `slider_changes_bar_live`, `arrange_drag_reorders_bar`, `alt_up_reorders`, `settings_survive_restart`, `switch_to_gaming_profile`, `bad_file_does_not_crash` | 75 | 500 | boss | gpui, visual, unit |

### World 08: Desktop widget (`08-widget`), project "Desktop widget", 12 quests

| slug | new | objectives | min | XP | kind | runs |
|---|---|---|---|---|---|---|
| 01-widget-window | a `Normal`, title-less, transparent window; `slot.take()` + `remove_window` toggles it off | `widget_toggle_opens_window` | 4 | 25 | exercise ⚡ | gpui |
| 02-drag-to-move | `window.start_window_move()` on the header's press, through a swappable fn (the test platform cannot move windows); locked does not move | `header_press_starts_move` | 5 | 30 | exercise | visual |
| 03-snap-to-edges | pure: within 12 px of the work area's edge, snap; one side per axis; keep the size | `near_left_edge_snaps_to_zero`, `middle_does_not_snap` | 6 | 40 | exercise | unit |
| 04-remember-position | saved spot if 40 x 40 of it is on screen, else the top-right corner; `is_some_and` | `position_restored_after_restart` | 4 | 25 | exercise ⚡ | unit |
| 05-groups | `group_of` with `\|` arms; headings in `GROUPS` order | `cpu_stats_under_processor` | 6 | 40 | exercise | visual |
| 06-usage-bars | `.w(relative(share))` | `bar_at_50_is_half_the_track` | 6 | 40 | exercise | visual |
| 07-graph-length | readings for 30 s to 5 min at the interval; widen before multiplying, never divide by 0 | `five_minutes_keeps_300_points` | 4 | 25 | exercise ⚡ | unit |
| 08-core-bars | one bar per thread, `.flex_wrap()`, filled from the bottom | `sixteen_threads_sixteen_bars` | 6 | 40 | exercise | visual |
| 09-styles | `WidgetStyle` to a `(bars, graphs)` pair | `minimal_draws_no_graphs` | 5 | 30 | exercise | visual |
| 10-placement | `WinApi::set_layer`: desktop (`HWND_BOTTOM`), normal, always on top | `desktop_placement_sends_to_bottom` | 6 | 40 | exercise | fake |
| 11-click-through | `WS_EX_LAYERED \| WS_EX_TRANSPARENT`: set with `\|`, clear with `& !`, other bits kept | `lock_sets_click_through_bits`, `unlock_clears_them` | 5 | 30 | exercise ⚡ | unit |
| 12-boss-desktop-widget | `WidgetView::render` from scratch (header, groups, bars, graphs by style, core bars) and `apply_widget_window` | `opens_from_menu`, `groups_drawn`, `bars_match_values`, `style_switches`, `snaps_and_saves`, `placement_applied`, `lock_is_click_through` | 60 | 450 | boss | visual, fake, unit |

### World 09: Game overlay (`09-overlay`), project "Game overlay and FPS", 15 quests

Frame maths is graded on `fixtures/capture.csv`: 1200 made-up frames shaped like a cut-down PresentMon capture
(every second frame generated, five stutters), generated so its numbers are exact. Live ETW is graded only on
"starts, or says it needs administrator".

| slug | new | objectives | min | XP | kind | runs |
|---|---|---|---|---|---|---|
| 01-how-fps-is-counted | presents, ETW, PresentMon, no hooking, admin, frame generation, lows | quiz: 3 | 4 | 15 | read | - |
| 02-read-a-capture | columns found by name in the header; `filter_map` + `?` per row into `Frame` | `capture_has_1200_frames` | 6 | 40 | exercise | unit |
| 03-fps | frames over the seconds they took (not an average of rates) | `average_of_16_6_ms_is_60_fps` | 4 | 25 | exercise ⚡ | unit |
| 04-lows | 1 % and 0.1 % lows: sort with `total_cmp`, average the slowest share, at least one | `one_percent_low_of_capture`, `point_one_percent_low_of_capture` | 7 | 45 | exercise | unit |
| 05-ten-second-window | a `VecDeque` window by time: push, then pop while older than 10 s | `old_frames_are_dropped` | 6 | 40 | exercise | unit |
| 06-stutter | the median (even counts: `is_multiple_of(2)`), spikes over twice it | `spikes_found_in_capture` | 6 | 40 | exercise | unit |
| 07-base-fps | only frames that are not `generated`, over the same time | `frame_gen_doubles_displayed_fps` | 4 | 25 | exercise ⚡ | unit |
| 08-live-frames | `ferrisetw` DXGI present trace (*starter*); a guarded `match` turns OS error 5 into `NeedsAdmin` | `access_denied_means_needs_admin`, `etw_starts_or_says_needs_admin` | 8 | 60 | exercise | unit, live |
| 09-whats-a-game | covers its monitor and is not `explorer.exe`; `eq_ignore_ascii_case` | `borderless_game_counts`, `windowed_app_does_not` | 5 | 30 | exercise | unit |
| 10-overlay-window | top-centre `PopUp` strip, transparent, unfocused; `OverlayView` (*starter*) fed by a replay of the capture | `overlay_sits_top_centre` | 6 | 40 | exercise | unit |
| 11-global-hotkey | `global-hotkey`: Ctrl+Shift+F10 works while a game has focus; act on Pressed only | `hotkey_is_ctrl_shift_f10`, `hotkey_toggles_overlay` | 6 | 40 | exercise | unit |
| 12-auto-show | `AutoShow::step`: exit first, then show for a game, hide on Alt+Tab; match guards | `game_exit_closes_overlay`, `alt_tab_hides_overlay` | 7 | 45 | exercise | unit |
| 13-session-summary | one CSV line in the header's order, empty cell for a missing temperature | `summary_line_matches_header` | 5 | 30 | exercise ⚡ | unit |
| 14-benchmark | Ctrl+Shift+F11 toggles; one row per tick while recording | `recording_writes_one_row_per_tick` | 6 | 40 | exercise | unit |
| 15-boss-game-overlay | `FrameStats::compute` and `OverlayController::tick` (peak GPU temperature, auto-show, summaries); `main.rs` runs the live trace, hotkeys and controller | `fps_from_capture`, `lows_from_capture`, `graph_marks_spikes`, `hotkey_toggles`, `shows_for_game`, `closes_with_game`, `summary_saved`, `benchmark_rows`, `live_or_needs_admin` | 75 | 500 | boss | unit, gpui, live |

### World 10: Dashboard (`10-dashboard`), project "Dashboard", 12 quests

| slug | new | objectives | min | XP | kind | runs |
|---|---|---|---|---|---|---|
| 01-dashboard-window | a window of `Card` components in a `.grid().grid_cols(n)` | `dashboard_has_ten_cards` | 4 | 25 | exercise ⚡ | visual |
| 02-a-card | now, average and peak through `Reading::text` and `History::summary` | `card_shows_peak` | 5 | 30 | exercise | gpui |
| 03-fit-the-window | columns from `window.viewport_size()`, clamped 1..=6; `simulate_resize` | `wide_window_has_four_columns` | 6 | 40 | exercise | unit, visual |
| 04-scale-the-text | a tenth of the card width, clamped 14..=32 | `small_cards_shrink_text` | 4 | 25 | exercise ⚡ | unit |
| 05-a-gauge | a point on a half circle: angle `π × (1 − share)`, `cos` across, `− sin` up | `fifty_percent_ends_at_top` | 7 | 45 | exercise | unit |
| 06-themes | `Theme` struct; Modern, Terminal, Amber terminal, Yellow terminal | `amber_theme_text_is_amber` | 5 | 30 | exercise | unit |
| 07-layout-editor | hidden cards from `Settings::dashboard.hidden` | `hidden_disk_card_is_gone` | 5 | 30 | exercise ⚡ | visual |
| 08-snapshot-json | `#[derive(Serialize)]` `WebSnapshot` of every stat | `snapshot_json_has_cpu_usage` | 5 | 30 | exercise | gpui |
| 09-web-server | axum router and handler; its own thread and tokio runtime, `Arc<Mutex<WebSnapshot>>` shared; tested with `tower::ServiceExt::oneshot` | `get_stats_returns_json` | 8 | 60 | exercise | unit |
| 10-secret-key | `?key=` checked with a constant-time compare, 401 otherwise; `new_key` from `getrandom` (*starter*) | `request_without_key_is_401`, `request_with_key_is_200` | 7 | 45 | exercise | unit |
| 11-local-only | `127.0.0.1` unless "share on network", then `0.0.0.0` | `default_binds_localhost` | 4 | 25 | exercise ⚡ | unit |
| 12-boss-dashboard | `DashboardView::render` from scratch and `serve_dashboard`; `main.rs` makes and saves the key, prints the link, feeds the snapshot with `observe_self` | `window_cards_fit`, `cards_show_now_avg_peak`, `layout_editor_applies`, `theme_switches`, `web_json_matches_window`, `key_required`, `server_answers_on_localhost` | 60 | 450 | boss | visual, gpui, unit |

### World 11: Screen time and alerts (`11-screen-time`), project "Screen time", 11 quests

Every Windows call sits behind a `Desk` trait (foreground program, idle time, full screen, locked, local time);
tests use `FakeDesk`. Day files are graded in a temp folder; the real one is `%LocalAppData%\KinetikStats\ScreenTime`.

| slug | new | objectives | min | XP | kind | runs |
|---|---|---|---|---|---|---|
| 01-whats-in-front | `GetForegroundWindow` to `QueryFullProcessImageNameW` in `RealDesk` (*starter*); `exe_name`: last path piece, lower-cased | `exe_name_from_full_path` | 4 | 25 | exercise ⚡ | unit, fake |
| 02-five-second-samples | `Day::add`: 5 s to the program and to its hour (`entry().or_insert`, chrono `hour()`) | `three_samples_add_15_seconds` | 5 | 30 | exercise | unit |
| 03-idle | `away`: 5 min idle stops counting unless the window in front covers its monitor (`GetLastInputInfo` in the starter) | `idle_six_minutes_not_counted`, `fullscreen_video_still_counts` | 7 | 45 | exercise | unit |
| 04-locked | `app_to_count`: locked first, with `\|\|` (`OpenInputDesktop` fails on the lock screen) | `locked_pc_not_counted` | 4 | 25 | exercise ⚡ | unit |
| 05-day-files | `day_path` (`2026-10-22.json`) and `save_day`; `Day` holds names and seconds only, never titles | `day_file_named_by_date`, `no_window_titles_saved` | 6 | 40 | exercise | unit |
| 06-keep-400-days | `prune`: parse the stem as a date, delete `json` files over 400 days old, leave anything else | `day_401_is_deleted` | 4 | 25 | exercise ⚡ | unit |
| 07-day-summary | `summarize`: most used program and busiest hour with `max_by_key` | `busiest_hour_is_21`, `most_used_is_minecraft` | 7 | 45 | exercise | unit |
| 08-hour-chart | `bar_heights` relative to the busiest hour; the page opens from the bar menu's "Screen time" row | `busiest_bar_is_tallest` | 6 | 40 | exercise | unit, visual |
| 09-hot-for-30-seconds | `HotTimer::check`: since when over the limit, one alert after 30 s, reset when back under; executor clock | `short_spike_no_alert`, `thirty_seconds_alerts_once` | 7 | 45 | exercise | gpui |
| 10-toast | `toast_text` per kind; `show_toast` with `tauri-winrt-notification` and PowerShell's app id (*starter*) | `hot_gpu_toast_text` | 5 | 30 | exercise | unit |
| 11-boss-screen-time | `ScreenTime::sample` and `ScreenTimeView::render` from scratch; `main.rs` runs the sampler, the page and toasts | `counts_foreground_app`, `idle_and_lock_pause`, `files_per_day_pruned`, `summary_right`, `chart_drawn`, `alerts_debounced` | 60 | 450 | boss | unit, visual, gpui |

### World 12: Ship it (`12-ship`), project "Kinetik Stats 1.0", 11 quests

| slug | new | objectives | min | XP | kind | runs |
|---|---|---|---|---|---|---|
| 01-release-profile | `[profile.release]`: `lto`, `strip`, `opt-level`; why the exe size and memory matter | quiz: 2 | 3 | 15 | read ⚡ | - |
| 02-single-instance | `CreateMutexW("KinetikStats")`: a second launch tells the first to open Settings | `second_launch_is_detected` | 6 | 40 | exercise | fake |
| 03-command-line | `--settings Arrange`, `--dashboard` | `settings_arrange_opens_arrange_page`, `quoted_screen_time_page_parses` | 5 | 30 | exercise | unit |
| 04-app-theme | `Theme` global: Graphite, OLED black, Nord, Mocha, ... with an accent | `nord_with_mint_accent` | 5 | 30 | exercise ⚡ | gpui |
| 05-run-at-startup | `winreg`: HKCU `Run` value pointing at the exe (no admin) | `startup_value_points_at_exe` | 6 | 40 | exercise | unit |
| 06-check-for-updates | parse a GitHub releases JSON fixture, compare with `semver` | `v4_1_0_is_newer`, `test_versions_skipped_unless_on` | 6 | 40 | exercise | unit |
| 07-checksum | `sha2::Sha256` of the zip against the release notes | `tampered_zip_fails` | 4 | 25 | exercise ⚡ | unit |
| 08-swap-and-roll-back | stage, check, swap the exe; roll back if any step fails | `failed_swap_rolls_back` | 8 | 60 | exercise | unit |
| 09-memory-budget | cap every history and cache; a sum under the budget | `history_points_under_budget` | 5 | 30 | exercise | unit |
| 10-portable-zip | `zip`: `KinetikStats.exe` + README, no installer | `zip_has_exe_and_readme` | 5 | 30 | exercise ⚡ | unit |
| 11-boss-kinetik-stats | the whole app from one `main`: bar, widget, overlay, dashboard, settings, screen time, updater, release zip | `single_instance`, `bar_docked`, `widget_opens`, `overlay_hotkey`, `settings_persist`, `update_check_and_verify`, `zip_built`, `memory_under_budget` (+ look: "run it for a day") | 90 | 500 | boss | all |

## 4. Totals

| world | quests | XP | minutes |
|---|---|---|---|
| 00 Setup | 6 | 140 | 25 |
| 01 Readings | 13 | 745 | 107 |
| 02 Views | 12 | 725 | 104 |
| 03 Live | 13 | 825 | 115 |
| 04 Sensors | 14 | 940 | 135 |
| 05 Taskbar | 11 | 750 | 105 |
| 06 Interaction | 12 | 860 | 123 |
| 07 Settings | 16 | 1,090 | 164 |
| 08 Widget | 12 | 815 | 117 |
| 09 Overlay | 15 | 1,015 | 155 |
| 10 Dashboard | 12 | 835 | 120 |
| 11 Screen time | 11 | 800 | 115 |
| 12 Ship | 11 | 840 | 143 |
| **total** | **158** | **10,380** | **~25.5 h** |

## 5. Risks and spikes

Spikes R1-R3 and R5 ran on 2026-10-22 on FOX (Windows 11, rustc 1.98.1 MSVC, 3 monitors,
auto-hide taskbar). Code: `docs/tracks/gpui-spike/` (`visual.rs` is R2, `probe.rs` is R3 + R5).

| id | risk | result |
|---|---|---|
| R1 | gpui 0.2.2 from crates.io may not build on Windows | **Pass.** Builds and opens a window with no extra setup. Numbers in section 1 (Runner). Only noise: a future-incompat warning from `proc-macro-error2`. |
| R2 | `#[gpui::test]` + `VisualTestContext` headless on Windows | **Pass.** `add_window_view`, `debug_bounds` (size and position), `simulate_click` hitting an `on_click`, `simulate_resize`, `executor().advance_clock` all work; 3 tests in 0.01 s. The `visual` grading row stands. |
| R3 | PopUp + Transparent + no titlebar: see-through, on top, placeable | **Pass, with two rules.** (a) Transparent is exact: over white, black and red backdrops the empty part of the bar matches the backdrop pixel for pixel; the pill draws its colour exactly. (b) `SetWindowPos(HWND_TOPMOST)` on the HWND from `HasWindowHandle` sticks and the bar sits above `Shell_TrayWnd` in z-order. (c) PopUp gets `WS_EX_TOOLWINDOW`: no taskbar button. **Rule 1:** GPUI checks that the requested bounds' centre lies on the target display; if not (an auto-hidden taskbar's off-screen rect, another monitor without `display_id`) it silently uses its default 1552x873 centred window. World 05 teaches `display_id` and handles auto-hide. **Rule 2:** bounds are the client area; Windows adds an invisible 8 px frame. Resizing the HWND afterwards with `SetWindowPos` shrinks GPUI's viewport (200x48 became 184x39), so size through GPUI, never through Win32. Not tested: docking on a visible taskbar (this PC auto-hides it), and topmost surviving a click on the taskbar. World 05 handles auto-hide (the bar hides with the taskbar and returns when it slides up) and re-raises the bar when the taskbar comes to the front; both still need a check on a PC with a fixed taskbar. |
| R4 | 0.2.2 is a year old; Zed's git API has moved | **Decided: stay on crates.io `=0.2.2`.** R1-R3 found nothing that needs a newer gpui. Moving later is a "what changed" pass like `neoforge-latest`. |
| R5 | `tray-icon` and `global-hotkey` need a Win32 message loop | **Pass.** Both created inside `Application::run`; GPUI's main-thread loop pumps their hidden windows. A real Ctrl+Alt+Shift+F9 via `SendInput` gave 2 events (Pressed, Released); a click message to the tray window gave 1 `TrayIconEvent::Click`. Read both receivers from a `cx.spawn` loop every 100 ms. |
| R6 | ETW needs admin; Oxide runs tests unelevated | not run; designed around: fixtures grade, live is `skip: needs administrator` |
| R7 | disk | 2.8 GB for gpui + Win32 crates in debug. Under the owner's 30 GB rule. |

## 6. Later (side world `13-extras`, not planned in detail)

Now playing (WinRT `GlobalSystemMediaTransportControlsSessionManager`), Bluetooth device
batteries (WinRT), Timeline (snapshot diff every 3 hours), Crashes (Event Log query with
plain-words explanations), companion shared memory, per-stat animated icons
(`with_animation`), the admin startup task (Task Scheduler) and the protected Program Files copy.
The right-click menu and hover card as their own pop-up windows: inside the docked bar's
48 px window they are clipped (World 06 runs the bar in a normal window for that reason).

## 7. Open questions for the owner

1. Track id `gpui` with the title "Kinetik in Rust", or worlds inside the Rust track after world 10?
2. One story only (the Kinetik names are the plain version), or also a "plain" flavour with
   textbook names? This plan assumes one story: the project is the hook.
3. World 07 has 16 quests. Fine, or split Arrange into its own world?

## 8. Order of work

1. ~~Spikes R1-R3, R5~~ done 2026-10-22, all pass (section 5).
2. ~~`content/tracks/gpui/track.toml` + `docs/tracks/gpui-terms.md`~~ done; ledger grows with each world.
3. ~~Worlds 00-11~~ written 2026-10-22, content-check green (147 quests): a window in 15 minutes, a real-looking
   bar by the end of world 02. Next worlds: one content session each, `cargo run -p content-check -- gpui/<world>` green, commit per world.
4. Owner plays world 00-02 in the app before worlds 03+ are written.
