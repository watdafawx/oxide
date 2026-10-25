# GPUI track: term ledger

Every GPUI, crate and project term the track introduces, in order, with the quest that does.
A lesson may use only rows at or above its own quest, plus the Rust track's ledger
(`docs/tracks/rust-terms.md`) up to `rust/05-traits/21-boss-shape-renderer`, which this track
requires. Helpers a starter contains are marked *starter*: shown, not taught.

| term | quest | how it is introduced |
|---|---|---|
| GPUI, describe-don't-paint, describe again on change; the Rust prerequisite (Traits now, Smart pointers / Concurrency / Async later) | 00-setup/01-what-gpui-is | read |
| msvc vs gnu toolchain, `rustup show`, DirectX / DirectWrite | 00-setup/02-msvc-toolchain | read |
| dependency, `[dependencies]`, `=` version pin, feature, `features = ["test-support"]` | 00-setup/03-first-build | add gpui to Cargo.toml (lesson shows a `sysinfo` line) |
| `Application::new().run(\|cx\| ..)`, context `cx` (`App`), `cx.open_window`, `WindowOptions` + `..Default::default()`, `Bounds::centered` (*starter*), `on_window_closed` + `quit`, subscription, dropping cancels, `.detach()` | 00-setup/04-hello-window | explained in the lesson, `main.rs` is *starter* |
| `px(..)` takes `f32`, `200.` float literal, `size(w, h)`, `Size<Pixels>` | 00-setup/04-hello-window | `bar_size` |
| view (a struct GPUI keeps), `cx.new(\|_\| ..)` puts it in the window, `impl Render`, `render(&mut self, window: &mut Window, cx: &mut Context<Self>)`, `Context<Self>` vs `App`, element (one frame's description, thrown away), `impl IntoElement`; no virtual-DOM diff (for the curious) | 00-setup/05-what-a-view-is | read |
| builder chain (each call takes the box and hands it back), `div()`, `.child(..)` with text or a box, nesting, `.debug_selector(\|\| "..".into())` (closure, runs only with test-support, `.into()` to `String`) | 00-setup/06-the-first-pill | the first pill |
| event loop, render on open / resize / when asked, quit on close | 00-setup/07-it-runs | read |
| `StatKind` (9 kinds), `label()`; `\|` in a match arm (for the curious) | 01-readings/01-what-a-stat-is | match arm per kind |
| `Reading { kind, value: Option<f64> }`, `--` for no value, `.to_string()` on a literal | 01-readings/02-a-reading | `None` arm |
| `Unit`, `StatKind::unit()` | 01-readings/03-units | match arm per kind |
| `{value:.N}` precision, `:.0` rounding; ties round to even (for the curious) | 01-readings/04-format-percent | `format_percent` |
| repeated division by 1024 in a `while` with `&&`, `mut` counters, `{value:.1}` (*starter*); KB vs KiB (for the curious) | 01-readings/05-format-bytes | `format_bytes` |
| reuse a function inside `format!`, `as u64`; float-to-int `as` saturates, NaN is 0 (for the curious) | 01-readings/06-format-speed | `format_speed` |
| `Limits`, `limit_for` (*starter*), match on a tuple of two `Option`s, `>=` at the limit | 01-readings/07-warning-limits | `is_warning` |
| `WarnWhen { Above, Below }`, `Limit { at, when }`, a `match` as an arm's value | 01-readings/08-low-is-bad | battery warns low |
| `VecDeque`, `push_back`, `pop_front` returns `Option`, cap with `>`; ring buffer (for the curious) | 01-readings/09-history | `History::push` |
| `f64::MAX` / `f64::MIN` start values, `.min()` / `.max()`, average with `as f64`, empty gives NaN so check first; `f64` not `Ord`, `fold` (for the curious) | 01-readings/10-min-avg-max | `History::summary` |
| `Slot { Stat, Divider }`, `DividerStyle`, entries in a `vec![..]` | 01-readings/11-bar-order | dividers in `default_order` |
| slice `&mut [T]`, `.swap(a, b)`, guard before `index - 1` (usize overflow), early `return false` | 01-readings/12-move-a-slot | `move_up` |
| `Pill` (`label: &'static str`), `BarItem`, `Column`, `error[E0382]` on a moved `Option`, `Option::take`, `pill.clone()`, `*style` | 01-readings/13-two-lines | `two_line_columns` |
| `let .. else` (else must `return` or `break`), a `Reading` per number for its unit | 01-readings/14-no-data-yet | `hover_summary` |
| type alias `type Snapshot = HashMap<StatKind, f64>`, `.get(&k).copied()`, `*kind` on a borrowed `&StatKind` | 01-readings/15-boss-stat-model | boss lesson cards |
| `.flex()`, `.flex_row()`, `.flex_col()` (named), `.gap_N()` as a spacing-scale step (`gap_1` 4 px, `gap_2` 8 px), boxes stack by default; Taffy flexbox / Tailwind names (for the curious) | 02-views/01-a-row | pills side by side |
| `.h(..)`, `.w(..)`, `px(32.)` is an `f32`, `.items_center()` (*starter*) | 02-views/02-pixels | 32 px bar |
| spacing scale, rem (16 px base), padding `.p_N()` / `.px_N()` / `.py_N()`, `px(..)` (pixels) vs `.px_2()` (padding), `rems(..)` and `m`/`w_`/`h_` (for the curious) | 02-views/03-the-spacing-scale | read |
| `rgb(0x..)`, hex colour digits, `.bg(..)`, `.text_color(..)`, `Rgba`, text style runs down to children | 02-views/04-colours | `value_colour` |
| `.children(iter)`, a function name as the `.map(..)` argument | 02-views/05-children-from-a-list | one element per pill |
| names built with `format!("pill-{kind:?}")` (closure runs only with test support, so free in release), `let kind = pill.kind;` because format braces take a plain name, `move` (habit for closures GPUI keeps) | 02-views/06-name-the-parts | label and value boxes |
| component, `#[derive(IntoElement)]`, `RenderOnce`, `render(self, ..)`, fields move out of `self`, view vs component (when to choose), `cx: &mut App` vs `cx: &mut Context<Self>` | 02-views/07-a-pill-component | `PillView` |
| `.when(condition, \|el\| ..)` | 02-views/08-labels-or-not | `LabelMode` |
| `divider_width` match, `.flex_none()`, `AnyElement` / `.into_any_element()` (type forgotten, like `Box<dyn Trait>`), `.enumerate()` for unique `divider-<i>` names | 02-views/09-dividers | four divider styles |
| `.children(Option)`, the learner writes `.into_any_element()` | 02-views/10-two-lines | `column` |
| alpha `Rgba.a`, writing a field of a `mut` colour, `.rounded(..)` (*starter*), `.px_2()` (*starter*) | 02-views/11-the-pill-background | `pill_background` |
| `.min_w(..)`, `.min_h(..)`, `.font_family(..)` (*starter*) | 02-views/12-numbers-that-sit-still | 48 px value box |
| `.text_size(..)` (runs down to children), `f32::clamp` | 02-views/13-text-size | `bar_text_size` |
| pill back to a function on purpose (the Interaction world needs the bar's `Context<StatsBar>` in it) | 02-views/14-boss-static-bar | assembles world 02 |
| state = a view's fields, `render` only reads them, `view.rs` holds the drawing (*starter*) | 03-live/01-state-in-a-view | `StatsBar::new(snapshot)` |
| entity, `Entity<T>`, `cx.new(\|_\| ..)` (what `main.rs` has done since 00/05), a view is an entity whose type implements `Render`, `read_with`; the app's entity store (for the curious) | 03-live/02-entity | `new_bar` |
| `entity.update(cx, \|bar, cx\| ..)`, `Context<T>`, `cx.notify()` | 03-live/03-update-and-notify | `set_reading` |
| the context family: `App`, `Context<T>`, `Window`, `AsyncApp`, `TestAppContext`, `VisualTestContext` | 03-live/04-which-cx | read |
| `trait Sensor`, `Box<dyn Sensor>`, `ScriptedSensor`, `get(i).copied()` | 03-live/05-a-fake-sensor | test stand-in |
| `cx.spawn(..)`, **async closure** `async move \|this, cx\| { .. }`, `this: WeakEntity<T>` (named), `AsyncApp`, `this.update(..)` returns `Result`, `.ok()`, `Task`, `.detach()`; async closures vs closure returning an async block (for the curious) | 03-live/06-run-it-later | first tick on its own |
| dropped `Task` is cancelled (same rule as the `Subscription` from 00/04), a detached loop outlives its bar, `_poll` field, leading `_` on a field | 03-live/07-keep-the-task | keep the task |
| executors, `cx.background_executor().timer(..).await`, never `thread::sleep`, the spawned block cannot borrow `self`, test clock `advance_clock` | 03-live/08-a-timer | wait one interval |
| `loop` with `.await`, why the handle is weak (no bar-task cycle), `this.update(..).is_err()` then `break` | 03-live/09-every-second | the polling loop |
| `Duration::from_millis`, `u64::clamp`, builder method `fn with_interval(mut self, ..) -> Self`, `mut self` | 03-live/10-update-interval | `clamp_interval` |
| compare before notify, `PartialEq` on items | 03-live/11-redraw-only-on-change | skip identical redraws |
| `HashMap::entry(..).or_insert_with(..)` | 03-live/12-history-per-stat | `histories` |
| `sysinfo` (`system` feature), `System::new()`, `refresh_cpu_usage`, `global_cpu_usage`, `cpus()` / `cpu_usage()`, `MINIMUM_CPU_UPDATE_INTERVAL` | 03-live/13-real-cpu | `CpuSensor` |
| `refresh_memory`, `used_memory`, `total_memory` | 03-live/14-real-ram | `RamSensor` |
| (no new terms) | 03-live/16-boss-live-bar | assembles world 03 |
| performance counters, driver, DLL (named), WMI (a query service), admin-only stats, never 0 for unknown | 04-sensors/01-what-needs-admin | read |
| `Networks`, `refresh`, `received()`, `Instant`, `Duration`, `Duration::as_secs_f64` vs `as_secs`, `Option::replace` (*starter*), divide-by-zero guard | 04-sensors/02-network-speed | `speed` |
| `iter().any(..)`, `!` for "none of them", `str::contains` | 04-sensors/03-skip-loopback | `counts_toward_total` |
| `StatKind::DiskFree` (*starter*), `Disks`, `mount_point`, `Path::new`, `find(..)?` | 04-sensors/04-free-disk | `DiskSensor` |
| DLL loaded at run time, `nvml-wrapper`, `Nvml::init`, `Result::ok()` then `?` in a chain | 04-sensors/05-gpu-via-nvml | `NvidiaGpu::usage` |
| `position`, `enumerate` as `(index, item)`, `max_by_key`, `map` to the index | 04-sensors/06-pick-the-gpu | `pick_gpu` |
| WMI query and `ThermalZone` (*starter*), ACPI, tenths of a kelvin, array and range `contains` | 04-sensors/07-cpu-temp-fallback | `acpi_to_celsius` |
| `unsafe { .. }` (why: C code is outside the compiler's view), `// SAFETY:`, `windows` crate, `SYSTEM_POWER_STATUS::default()`, `&mut` out-struct, `.ok()?` | 04-sensors/08-battery | `read_power_status` |
| bits, bit mask / flags, "the bit worth 8", `flag & 8 != 0`, 255 as "unknown" | 04-sensors/09-battery-flags | `decode_battery` |
| `TcpStream::connect_timeout`, `SocketAddr`, `Instant::elapsed` | 04-sensors/10-ping | `ping_once` |
| integer `/` and `%` for units of time | 04-sensors/11-uptime | `format_uptime` |
| `HashSet` (`contains`), `continue` | 04-sensors/12-lazy-sensors | read only wanted stats |
| UI thread, waiting vs blocking, foreground vs background executor, `background_executor().spawn(async move { .. })` returns a `Task<T>` to `.await`, the hop (`read_with` → background → `this.update`), let-else, `Arc<Mutex<..>>` refresher (rust 07), `Send` refresher, poisoned lock and `into_inner` | 04-sensors/13-keep-the-ui-thread-free | read |
| `Send` on `dyn Trait`, supertrait `trait Sensor: Send`, why `cx.spawn` needs no Send | 04-sensors/14-off-the-ui-thread | the compile error |
| writing the hop: `background_executor().spawn(async move { .. }).await`, `this.update(..).is_err()` then `break` | 04-sensors/15-write-the-hop | `StatsBar::start` |
| `enum Missing`, default trait method, HRESULT (top bit = failed, hex codes), `0x8004_1003_u32 as i32` | 04-sensors/16-why-its-missing | `missing_from_wmi` |
| nested `match` on two `Option`s, `HashMap::insert`/`remove` return the old value, `*why` to copy out | 04-sensors/17-boss-sensor-hub | `read_sensors`, `apply`, `hover_text` |
| `WindowOptions` + `..Default::default()`, `titlebar: None`, `WindowKind::PopUp`, `WS_EX_TOOLWINDOW` (named), `is_movable`, `is_resizable`, focus, `focus: false` | 05-taskbar/01-window-options | `bar_window_options` |
| `WindowBackgroundAppearance::Opaque` / `Transparent`, compositing (named) | 05-taskbar/02-see-through | see-through window |
| Win32 window, child window, window class, HWND (a handle), UTF-16 and `w!(..)`, `FindWindowW`, `FindWindowExW`, app bars / `SHAppBarMessage` (for the curious) | 05-taskbar/03-find-the-taskbar | `find_taskbar` |
| screen coordinates (y grows down), `Rect`, `intersect`, auto-hide | 05-taskbar/04-dock-maths | `dock_bar` |
| display scaling, DPI (96 = 100 %), physical vs logical pixels, `as f32` before dividing, `Bounds::new(point, size)` | 05-taskbar/05-dpi | `to_logical` |
| seam, `trait WinApi`, `RealWin`, `FakeWin` that records `Call`s (a spy), `Docker<A: WinApi>` generic vs `Box<dyn ..>` | 05-taskbar/06-a-seam-for-tests | read |
| z-order, topmost, `SetWindowPos(HWND_TOPMOST, ..)` (*starter*), `raw-window-handle` and `hwnd_of` (*starter*) | 05-taskbar/07-stay-on-top | `Docker::on_open`, `on_foreground_change` |
| `render`'s `window` parameter, `window.appearance()`, `WindowAppearance`, `\|` in a match arm, exhaustive `match` | 05-taskbar/08-light-or-dark | `text_colour_for` |
| no fullscreen flag: a window that covers its monitor, maximised vs fullscreen, per-monitor hiding | 05-taskbar/09-hide-in-fullscreen | `covers_monitor`, `bar_visible` |
| `Shell_SecondaryTrayWnd` (*starter*), `filter(..).copied()` on a `Copy` type, `display_id` | 05-taskbar/10-every-taskbar | `taskbars_to_use` |
| compare with last (`PartialEq` on `Taskbar`), `move_to` without resizing | 05-taskbar/11-taskbar-moved | `Docker::redock` |
| keep an `Option` instead of `?` when later steps still run, `taskkill` | 05-taskbar/12-boss-docked-bar | `Docker::tick` |
| listener, `cx.listener(\|this, event, window, cx\| ..)`: the four inputs in order, `_` for unused ones, why a wrapper (the draw-time borrow has ended), `move` takes a copy of a `Copy` value, elements rebuilt every frame (under the hood) | 06-interaction/01-what-a-listener-is | read |
| `.id(..)`, `Stateful<Div>`, `SharedString::from`, `.on_click(cx.listener(..))` | 06-interaction/02-a-click | `interactive_pill` |
| `Launcher` trait, `FakeLauncher`, `std::process::Command` (*starter*), `Rc<RefCell<Vec<String>>>` refresher (rust/06) | 06-interaction/03-left-click-action | `on_pill_click` |
| `.hover(\|style\| ..)`, `rgba(..)`, `.on_hover(..)` with `hovered: &bool`, `*hovered`, the hover race | 06-interaction/04-hover | `ui.hovered` |
| `.tooltip(..)` as a plain closure (no `this`), `AnyView`, `cx.new(..).into()`, clone inside an `Fn` closure, tooltip delay | 06-interaction/05-hover-card | `HoverCard` |
| `Bounds` edges, share between min and max, `Pixels` arithmetic, `point(..)` | 06-interaction/06-graph-points | `graph_points` |
| `canvas(prepaint, paint)`, `PathBuilder::stroke`, `move_to` / `line_to`, `build()` as a `Result`, `window.paint_path` | 06-interaction/07-draw-the-graph | the paint closure of `graph` |
| `ProcessUse` pair `.0` / `.1`, `sort_by_key`, `std::cmp::Reverse`, `truncate` | 06-interaction/08-top-processes | `top_five` |
| `fn() -> T` field (function pointer), calling a field `(field)()`, copying a `Copy` value out before a `move` closure; struct update `..x` (for the curious) | 06-interaction/09-processes-when-it-opens | tooltip fills `card.top` |
| focus, `FocusHandle`, `cx.focus_handle()`, `.track_focus(&handle)`, `window.focus`, `focus_in` (*starter*), `match &option` giving the element back in both arms, actions travel up from focus | 06-interaction/10-who-hears-an-action | read |
| `actions!`, `.on_action(cx.listener(..))`, the event's type `_: &Action` picks the handler, `cx.quit()` | 06-interaction/11-actions | `bar_layer` |
| `.when_some(option, \|el, value\| ..)`, `anchored().position(..)`, `deferred(..)`, paint order | 06-interaction/12-a-menu-on-top | the menu child |
| `on_mouse_down(MouseButton::Right, ..)`, `MouseDownEvent::position`, `on_mouse_down_out` (*starter*) | 06-interaction/13-right-click | the pill's right click |
| `window.dispatch_action(..)`, `Box<dyn Action>`, `Box::new(action)`, `Action::boxed_clone` | 06-interaction/14-menu-items | `menu_row` |
| `tray-icon`, `TrayIconEvent::Click { .. }` with `..`, `MouseButtonState`, `tray_icon::MouseButton` in full, menu ids | 06-interaction/15-tray-icon | `tray_click_action`, `tray_menu_action` |
| (no new terms) | 06-interaction/16-boss-interactive-bar | assembles world 06 |
| `WindowHandle<T>`, `handle.update(cx, \|view, window, cx\| ..)` on a closed window, `activate_window`, `if let .. &&`, `&mut App` for opening windows, `..Default::default()` | 07-settings/01-a-second-window | `open_settings_window` |
| `#[derive(Serialize, Deserialize)]`, `#[serde(default)]`, `BTreeMap` for a stable JSON key order, `impl Default` | 07-settings/02-settings-struct | `Settings` |
| global, `impl Global`, `cx.set_global`, `cx.global::<T>()` | 07-settings/03-a-global | `StatsBar::from_settings` |
| `cx.observe_global::<T>(..)`, `Subscription`, clone a global before changing through `cx`, `T::update_global(cx, ..)` (`UpdateGlobal`) | 07-settings/04-live-preview | `watch_settings` |
| window state vs saved setting, `enum Page`, `cx.listener` on another view (`this` = `SettingsWindow`) | 07-settings/05-sidebar-pages | `sidebar_row` |
| `App` vs `Context<T>` vs `Window`, a component has no `this`, plain click closure with `&mut App`, type alias, `Box<dyn Fn(..) + 'static>` | 07-settings/06-handlers-without-a-view | read |
| `RenderOnce` click as a plain `move` closure; `.p(..)`, `.size(..)`, `.rounded_full()`, `.justify_end()` (*starter*, glossed) | 07-settings/07-a-toggle | `Toggle` |
| share, range, snap with `round()` | 07-settings/08-slider-maths | `slider_value` |
| `MouseMoveEvent::pressed_button`, `canvas` prepaint to record bounds, `Rc<Cell<..>>` refresher (rust/06) | 07-settings/09-a-slider | `gap_slider` |
| `HashMap/BTreeMap::insert` through `update_global`, `\|_, _, _, cx\|` | 07-settings/10-colour-swatches | `swatch` |
| `on_key_down`, `KeyDownEvent`, `keystroke.key`, `key_char: Option<String>`, focus for keys | 07-settings/11-typing-an-address | `address_box` |
| virtual list, `uniform_list(id, count, ..)`, `cx.processor`, `Range<usize>`, `collect()` typed by the return | 07-settings/12-a-long-list | `arrange_list` |
| `Vec::remove` + `insert`, `on_drag` / `on_drop` / `drag_over` (*starter*) | 07-settings/13-drag-to-reorder | `move_slot` |
| `KeyBinding::new(keys, action, context)`, `"alt-up"` key text, `cx.bind_keys`, `.key_context(..)`, `None` context | 07-settings/14-keys-to-reorder | `bind_settings_keys` |
| `fs::create_dir_all`, `path.parent()`, `serde_json::to_string_pretty`, temp file + `fs::rename`, `?` with `io::Result` | 07-settings/15-save-to-disk | `save` |
| `serde_json::from_str::<T>`, `match` on `Ok` / `Err`, `let _ =` for an ignored result, `clamp_settings` (*starter*), `.bak` backup | 07-settings/16-fix-bad-values | `load` |
| `App::observe_global` closure with only `cx`, `cx.set_global` to swap profiles | 07-settings/17-boss-settings | `apply_settings`, `persist_settings`, `Profiles` |
| `WindowKind::Normal` without a title bar, `Option::take`, `handle.update(cx, \|view, window, cx\| ..)` inputs named, `window.remove_window()` | 08-widget/01-widget-window | `toggle_widget_window`; the Settings window's `activate_window` as the sibling example |
| `window.start_window_move()`, listener shape refresher (`this`, event, `window`, `cx`), function pointer field `fn(&mut Window)`, calling a field `(this.start_move)(window)` | 08-widget/02-drag-to-move | `widget_header` |
| work area, distance with `abs()`, `else if` for one side per direction, `let mut r = window` copy (`Rect` is `Copy`) | 08-widget/03-snap-to-edges | `snap_to_edges` |
| `Rect::intersect` reused, `Option::is_some_and` | 08-widget/04-remember-position | `place_widget` |
| `\|` joining kinds in one match arm, no `_` arm on purpose | 08-widget/05-groups | `group_of` |
| `relative(share)` lengths, `.w_full()` / `.h_full()` = 100 % | 08-widget/06-usage-bars | `usage_bar` |
| widen with `as u64` (same types for `*`), `as usize`, `.max(1)` before dividing | 08-widget/07-graph-length | `history_points` |
| `.flex_wrap()`, `.items_end()`, `.children` from `iter().enumerate()`; `thread_local!` (for the curious, *starter*) | 08-widget/08-core-bars | `core_bars` |
| tuple `(bool, bool)` from a `match`, `let (a, b) = ..` | 08-widget/09-styles | `parts` |
| window stack (z-order), `HWND_BOTTOM` / `HWND_NOTOPMOST` / `HWND_TOPMOST`, `WinApi::set_layer`, `match` as a value in a `let` | 08-widget/10-placement | `apply_placement` |
| extended window style bits, `\|` to set, `!` flips every bit, `& !` to clear, `GetWindowLongPtrW` (*starter*) | 08-widget/11-click-through | `click_through_style` |
| `filter_map` with a `match` inside, `*kind` on a `&StatKind` from matching a `&Slot` | 08-widget/12-stats-in-bar-order | `bar_stats` |
| `Vec<(Group, Vec<StatKind>)>`, walking `GROUPS`, `continue` to skip, `**kind` vs `*kind` after `.copied()` | 08-widget/13-skip-empty-headings | `grouped` |
| `entity.read(cx)` borrows `cx`, `error[E0502]`, a `{ }` block ends the borrow, block value, owned elements outlive it (read) | 08-widget/14-read-then-let-go | quiz |
| a closure for one row, `let mut el` + `el = el.child(..)` in an `if let` chain, `.justify_between()`, `.size_full()`; `cx.defer(..)` (*starter*), `observe` + `observe_global` in one view | 08-widget/15-boss-desktop-widget | `WidgetView::render`, `apply_widget_window` |
| present, ETW, PresentMon, frame generation | 09-overlay/01-how-fps-is-counted | read |
| CSV header lookup by name, `filter_map` with `?` on `Option`, `parse()` target from the field type, `*` on `&&str`, a fixture file with `include_str!` | 09-overlay/02-read-a-capture | `parse_capture` |
| frames over seconds, `let total: f64 = ..sum()`, `as f64`, float divide by zero is `NaN` | 09-overlay/03-fps | `average_fps` |
| `to_vec()` copy, `sort_by(\|a, b\| b.total_cmp(a))`, `.round() as usize`, `.max(1)`, slice `[..n]`, `sum::<f64>()` | 09-overlay/04-lows | `low_fps` |
| a window by time, `while` + `pop_front`, `front().is_some_and(..)` | 09-overlay/05-ten-second-window | `FrameWindow::push` |
| median, `is_multiple_of`, `enumerate().filter(\|(_, m)\| **m > ..)` (`&&f64`), `sort_by(f64::total_cmp)` (for the curious) | 09-overlay/06-stutter | `spikes` |
| `map(\|f\| f.ms).sum::<f64>()`, `filter(..).count()` | 09-overlay/07-base-fps | `base_fps` |
| `ferrisetw` (*starter*), callback on another thread: `Fn + Send + Sync + 'static` (refresher → Rust 07), nested enum patterns, match guards, `raw_os_error` | 09-overlay/08-live-frames | `live_error` |
| `eq_ignore_ascii_case`, `!iter().any(..)` | 09-overlay/09-whats-a-game | `is_game` |
| `FrameFeed = Arc<Mutex<FrameWindow>>`, `lock()`, poisoned lock + `unwrap_or_else(\|p\| p.into_inner())` (refresher → Rust 07), `Pixels` arithmetic for centring | 09-overlay/10-overlay-window | `overlay_bounds` |
| `global-hotkey`: `HotKey::new`, `Modifiers` joined with `\|`, `Code`, `HotKeyState` | 09-overlay/11-global-hotkey | `overlay_hotkey`, `hotkey_action` |
| state machine, `*self = AutoShow::default()`, `if let .. &&`, guards on arms | 09-overlay/12-auto-show | `AutoShow::step` |
| `Option::map` + `unwrap_or_default` for an empty cell | 09-overlay/13-session-summary | `SessionSummary::csv_line` |
| closures in a `let` as small helpers inside a method | 09-overlay/14-benchmark | `Benchmark::tick` |
| `frames.last().map(..).unwrap_or(..)`, max of two `Option`s with a `match` on the pair (no `Option::or`) | 09-overlay/15-boss-game-overlay | `FrameStats::compute`, `OverlayController::tick` |
| grid, `.grid()`, `.grid_cols(n)` (a `u16`, `as u16`), style calls return the element, spacing scale (`gap_4` = 16 px, one step = a quarter of the 16 px base, `p_4`) | 10-dashboard/01-dashboard-window | `DashboardView` |
| closure capturing `kind`, `Option::and_then` (a step that may itself be `None`) vs `map`, `Copy` summary mapped twice | 10-dashboard/02-a-card | `card_numbers` |
| `window.viewport_size()`, `Pixels` is a length not a number, `f32::from(Pixels)`, `as usize` drops the fraction, `usize::clamp`, `simulate_resize` | 10-dashboard/03-fit-the-window | `columns_for` |
| card width = window minus the `p_4` padding (32 px) over the columns; clamping a computed size | 10-dashboard/04-scale-the-text | `card_text_size` |
| radians (full turn 2π, half π), angles from the right anticlockwise, `cos` / `sin`, `std::f32::consts::PI` / `FRAC_PI_2`, screen y down | 10-dashboard/05-a-gauge | `gauge_point` |
| a struct of colours chosen by `match`, one arm per look | 10-dashboard/06-themes | `theme` |
| `filter` + `collect`, `slice::contains` with the closure's reference | 10-dashboard/07-layout-editor | `visible_cards` |
| JSON (text, names in quotes, `{}` and `[]`, `null`), `#[derive(Serialize)]` for an API, enum variants as JSON strings, closure with a `{ }` block body | 10-dashboard/08-snapshot-json | `web_snapshot` |
| HTTP request (method, path) and response (status, body), 200, web server, route, handler; axum `Router`, `State`, `Json` (refresher, rust 10-apps); `Arc<Mutex<..>>` shared with the app (refresher, rust 07); lock, clone, let go; poisoned lock, `unwrap_or_else(\|p\| p.into_inner())`; `Response` + `.into_response()`; `tower::ServiceExt::oneshot` (*test*) | 10-dashboard/09-web-server | `stats` |
| query string `?key=..`, `Query`, `Option::as_deref`, `unwrap_or("")`, constant-time compare, 401 `StatusCode::UNAUTHORIZED`, early `return`; `getrandom` (*starter*) | 10-dashboard/10-secret-key | the key check |
| IP address, port, loopback `127.0.0.1` = `Ipv4Addr::LOCALHOST`, `0.0.0.0` = `UNSPECIFIED`, `SocketAddr::from((ip, port))`, `if` as a value | 10-dashboard/11-local-only | `bind_addr` |
| executor (GPUI vs tokio), `std::thread::spawn(move \|\| ..)` (refresher, rust 07), `tokio::runtime::Runtime::new()`, `block_on`, `?` inside a closure that returns `io::Result`, `JoinHandle`, `Builder::new_current_thread` (for the curious) | 10-dashboard/12-its-own-runtime | read |
| `tokio::net::TcpListener::bind(..).await?`, `axum::serve(listener, router).await`, dropping a `JoinHandle` does not stop the thread | 10-dashboard/13-serve-on-a-thread | `serve_dashboard` |
| settings before the bar borrow, `Option::map(..).unwrap_or_default()`, `.size_full()`; `observe_self` (*starter*) | 10-dashboard/14-boss-dashboard | `DashboardView::render` |
| `rsplit` on a `char` array, `.next()` with `unwrap_or`, `to_lowercase`; `GetForegroundWindow` to `QueryFullProcessImageNameW` behind a `Desk` trait (*starter*) | 11-screen-time/01-whats-in-front | `exe_name` |
| `BTreeMap::entry().or_insert` gives `&mut`, dereference `*` to change through it, chrono `Timelike::hour`, `as usize` to index | 11-screen-time/02-five-second-samples | `Day::add` |
| `Duration` comparisons, `&&` with `!`, a `const` `Duration`; `GetLastInputInfo`, `wrapping_sub` (*starter*) | 11-screen-time/03-idle | `away` |
| short-circuit `\|\|` (stops at true); `OpenInputDesktop` as a lock check (*starter*) | 11-screen-time/04-locked | `app_to_count` |
| `NaiveDate::format` (`%Y-%m-%d`), `Path::join`, `create_dir_all`, `serde_json::to_string_pretty` with plain `?` (`From` into `io::Error`) | 11-screen-time/05-day-files | `day_path`, `save_day` |
| `read_dir` entries as `io::Result`, `file_stem`, `OsStr` and `to_str`, `NaiveDate::parse_from_str(..).ok()`, `extension().is_some_and`, date minus date, `TimeDelta::num_days`, `let` chains | 11-screen-time/06-keep-400-days | `prune` |
| `max_by_key` on map pairs, one `*` per reference (`**secs`), `\|&hour\|` pattern, a range as an iterator, ties go to the last | 11-screen-time/07-day-summary | `summarize` |
| `Iterator::max`, `NaN` from 0 / 0, array `map`, cast before dividing; `.items_end()`, `.flex_1()` (*starter*) | 11-screen-time/08-hour-chart | `bar_heights` |
| `HashMap::entry` for "since when", `Instant::duration_since`, `HashSet::insert` as a once-check, `&&` stops at false, `background_executor().now()` | 11-screen-time/09-hot-for-30-seconds | `HotTimer::check` |
| `\|` in match patterns, a binding catch-all arm (`other => ..`); `tauri-winrt-notification` `Toast` (*starter*) | 11-screen-time/10-toast | `toast_text` |
| `unwrap_or_else(\|\| ..)`, a closure kept in a `let`, a named function passed to `map` (`.map(clock)`), `{:02}` zero padding | 11-screen-time/11-dashes-for-nothing | read |
| `let _ =` to drop a `Result` on purpose, clone out of `Entity::read`, `.text_xl()`, `.w_full()`, `.flex_none()`, `enumerate` for child names | 11-screen-time/12-boss-screen-time | `ScreenTime::sample`, `ScreenTimeView::render` |
