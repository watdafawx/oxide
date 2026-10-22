# GPUI track: term ledger

Every GPUI, crate and project term the track introduces, in order, with the quest that does.
A lesson may use only rows at or above its own quest, plus the Rust track's ledger
(`docs/tracks/rust-terms.md`) up to `rust/05-traits/12-boss-shape-renderer`, which this track
requires. Helpers a starter contains are marked *starter*: shown, not taught.

| term | quest | how it is introduced |
|---|---|---|
| GPUI, describe-don't-paint, render every change | 00-setup/01-what-gpui-is | read |
| msvc vs gnu toolchain, `rustup show`, DirectX / DirectWrite | 00-setup/02-msvc-toolchain | read |
| dependency, `[dependencies]`, `=` version pin, `features = ["test-support"]` | 00-setup/03-first-build | add gpui to Cargo.toml |
| `Application::new().run(\|cx\| ..)`, context `cx`, `cx.open_window`, `WindowOptions` (*starter*), `on_window_closed` + `quit` + `.detach()` | 00-setup/04-hello-window | explained in the lesson, `main.rs` is *starter* |
| `px(..)`, `size(w, h)`, `Size<Pixels>` | 00-setup/04-hello-window | `bar_size` |
| view, `impl Render`, `render(&mut self, window, cx)`, `div()`, `.child(..)`, `.debug_selector(..)` | 00-setup/05-a-view | the first pill |
| event loop, render when needed, quit on close | 00-setup/06-it-runs | read |
| `StatKind` (9 kinds), `label()` | 01-readings/01-what-a-stat-is | match arm per kind |
| `Reading { kind, value: Option<f64> }`, `--` for no value | 01-readings/02-a-reading | `None` arm |
| `Unit`, `StatKind::unit()` | 01-readings/03-units | match arm per kind |
| `{value:.0}` rounding in `format!` | 01-readings/04-format-percent | `format_percent` |
| repeated division by 1024, `while` with `&&`, `{value:.1}` (*starter*) | 01-readings/05-format-bytes | `format_bytes` |
| reuse a function inside another, `as u64` | 01-readings/06-format-speed | `format_speed` |
| `Limits`, `limit_for` (*starter*), match on a pair `(Some(v), Some(limit))`, `>=` at the limit | 01-readings/07-warning-limits | `is_warning` |
| `WarnWhen { Above, Below }`, `Limit { at, when }` | 01-readings/08-low-is-bad | battery warns low |
| `VecDeque`, `push_back`, `pop_front`, cap | 01-readings/09-history | `History::push` |
| `f64::MAX` / `f64::MIN` start values, `.min()` / `.max()`, average | 01-readings/10-min-avg-max | `History::summary` |
| `Slot { Stat, Divider }`, `DividerStyle`, `vec![..]` | 01-readings/11-bar-order | `default_order` |
| slice `&mut [T]`, `.swap(a, b)`, early `return false` | 01-readings/12-move-a-slot | `move_up` |
| `Snapshot` = `HashMap<StatKind, f64>`, `.get(..).copied()`, `Pill`, `BarItem`, `Column`, `Option::take`, `let .. else` | 01-readings/13-boss-stat-model | boss lesson and hints |
| `.flex()`, `.flex_row()`, `.gap_2()`, boxes stack by default | 02-views/01-a-row | pills side by side |
| `.h(..)`, `.items_center()` (*starter*) | 02-views/02-pixels | 32 px bar |
| `rgb(0x..)`, hex colour digits, `.bg(..)`, `.text_color(..)`, `Rgba` | 02-views/03-colours | `value_colour` |
| `.children(iter)`, `debug_selector` with `format!` and `move` (*starter*) | 02-views/04-children-from-a-list | one element per pill |
| component, `#[derive(IntoElement)]`, `RenderOnce`, `render(self, ..)` | 02-views/05-a-pill-component | `PillView` |
| `.when(condition, \|el\| ..)` | 02-views/06-labels-or-not | `LabelMode` |
| `divider_width` match, `.flex_none()`, `.into_any_element()` / `AnyElement` (*starter*) | 02-views/07-dividers | four divider styles |
| `.flex_col()`, `.children(Option)` | 02-views/08-two-lines | `column` |
| alpha `Rgba.a`, `.rounded(..)` (*starter*), `.px_2()` (*starter*) | 02-views/09-the-pill-background | `pill_background` |
| `.min_w(..)`, `.font_family(..)` (*starter*) | 02-views/10-numbers-that-sit-still | 48 px value box |
| `.text_size(..)`, `f32::clamp` | 02-views/11-text-size | `bar_text_size` |
| (no new terms) | 02-views/12-boss-static-bar | assembles world 02 |
| view state in fields, `view.rs` holds the drawing (*starter*) | 03-live/01-state-in-a-view | `StatsBar::new(snapshot)` |
| entity, `Entity<T>`, `cx.new(\|_\| ..)`, `read_with` | 03-live/02-entity | `new_bar` |
| `entity.update(cx, \|bar, cx\| ..)`, `Context<T>`, `cx.notify()` | 03-live/03-update-and-notify | `set_reading` |
| `trait Sensor`, `Box<dyn Sensor>`, `ScriptedSensor` | 03-live/04-a-fake-sensor | test stand-in |
| `cx.spawn(async move \|this, cx\| ..)`, `Task`, dropped task is cancelled, `_poll` field | 03-live/05-spawn-and-keep | keep the task |
| `cx.background_executor().timer(..).await`, test clock `advance_clock` | 03-live/06-a-timer | wait one interval |
| `loop` with `.await`, `WeakEntity`, `this.update(..).is_err()` then `break` | 03-live/07-every-second | the polling loop |
| `Duration::from_millis`, `u64::clamp` | 03-live/08-update-interval | `clamp_interval` |
| compare before notify, `PartialEq` on items | 03-live/09-redraw-only-on-change | skip identical redraws |
| `HashMap::entry(..).or_insert_with(..)` | 03-live/10-history-per-stat | `histories` |
| `sysinfo` (`system` feature), `System::new()`, `refresh_cpu_usage`, `global_cpu_usage`, `MINIMUM_CPU_UPDATE_INTERVAL` | 03-live/11-real-cpu | `CpuSensor` |
| `refresh_memory`, `used_memory` | 03-live/12-real-ram | `RamSensor` |
| (no new terms) | 03-live/13-boss-live-bar | assembles world 03 |
| admin-only stats, drivers, never 0 for unknown | 04-sensors/01-what-needs-admin | read |
| `Networks`, `refresh`, `received()`, `Instant`, `Duration::as_secs_f64`, `Option::replace` (*starter*) | 04-sensors/02-network-speed | `speed` |
| `iter().any(..)`, `str::contains` | 04-sensors/03-skip-loopback | `counts_toward_total` |
| `StatKind::DiskFree` (*starter*), `Disks`, `mount_point`, `Path::new`, `find(..)?` | 04-sensors/04-free-disk | `DiskSensor` |
| `nvml-wrapper`, `Nvml::init`, `Result::ok()` then `?` in a chain | 04-sensors/05-gpu-via-nvml | `NvidiaGpu::usage` |
| `position`, `enumerate`, `max_by_key` | 04-sensors/06-pick-the-gpu | `pick_gpu` |
| tenths of a kelvin, range `contains`, `wmi` query (*starter*) | 04-sensors/07-cpu-temp-fallback | `acpi_to_celsius` |
| `unsafe { .. }`, `// SAFETY:`, `windows` crate, `SYSTEM_POWER_STATUS`, bit flags `& 128` | 04-sensors/08-battery | `read_power_status`, `decode_battery` |
| `TcpStream::connect_timeout`, `SocketAddr`, `Instant::elapsed` | 04-sensors/09-ping | `ping_once` |
| `/` and `%` for units of time | 04-sensors/10-uptime | `format_uptime` |
| `HashSet`, `continue` | 04-sensors/11-lazy-sensors | read only wanted stats |
| `Send`, supertrait `trait Sensor: Send`, `Arc<Mutex<..>>` and `background_executor().spawn` (*starter*) | 04-sensors/12-off-the-ui-thread | the compile error |
| `enum Missing`, default trait method, WMI `HRESULT` | 04-sensors/13-why-its-missing | `missing_from_wmi` |
| (no new terms) | 04-sensors/14-boss-sensor-hub | assembles world 04 |
| `titlebar: None`, `WindowKind::PopUp`, `is_movable`, `is_resizable`, `focus: false` | 05-taskbar/01-window-options | `bar_window_options` |
| `WindowBackgroundAppearance::Transparent` | 05-taskbar/02-see-through | see-through window |
| window class, `FindWindowW`, `FindWindowExW`, `w!(..)`, `HWND` | 05-taskbar/03-find-the-taskbar | `find_taskbar` |
| `Rect`, `intersect`, auto-hide | 05-taskbar/04-dock-maths | `dock_bar` |
| physical vs logical pixels, scale, `Bounds::new(point, size)` | 05-taskbar/05-dpi | `to_logical` |
| `trait WinApi`, fake that records calls, topmost, `SetWindowPos`, `raw-window-handle` (*starter*) | 05-taskbar/06-stay-on-top | `Docker::on_open` |
| `window.appearance()`, `WindowAppearance`, `\|` in a match arm | 05-taskbar/07-light-or-dark | `text_colour_for` |
| covers its monitor, per-monitor hiding | 05-taskbar/08-hide-in-fullscreen | `bar_visible` |
| `Shell_SecondaryTrayWnd` (*starter*), `filter(..).copied()`, `display_id` | 05-taskbar/09-every-taskbar | `taskbars_to_use` |
| compare with last, `move_to` without resizing | 05-taskbar/10-taskbar-moved | `Docker::redock` |
| (no new terms) | 05-taskbar/11-boss-docked-bar | `Docker::tick` |
| `.id(..)`, `Stateful<Div>`, `SharedString`, `.on_click(cx.listener(..))`, `move` closures in listeners | 06-interaction/01-a-click | `interactive_pill` |
| `Launcher` trait, `FakeLauncher`, `std::process::Command` (*starter*) | 06-interaction/02-left-click-action | `on_pill_click` |
| `.hover(\|style\| ..)`, `.on_hover(..)`, `rgba(..)` | 06-interaction/03-hover | `ui.hovered` |
| `.tooltip(..)`, `AnyView`, `cx.new(..).into()`, tooltip delay | 06-interaction/04-hover-card | `HoverCard` |
| `Bounds` edges, `Pixels` arithmetic, `point(..)` | 06-interaction/05-graph-points | `graph_points` |
| `canvas(prepaint, paint)`, `PathBuilder::stroke`, `move_to` / `line_to`, `window.paint_path` | 06-interaction/06-draw-the-graph | `graph` |
| `sort_by_key`, `std::cmp::Reverse`, `truncate` | 06-interaction/07-top-processes | `top_five` |
| `actions!`, `.on_action(..)`, `track_focus`, `window.focus`, `cx.quit()` | 06-interaction/08-actions | `bar_layer` |
| `on_mouse_down(MouseButton::Right, ..)`, `MouseDownEvent`, `deferred`, `anchored().position(..)`, `on_mouse_down_out`, `when_some` | 06-interaction/09-right-click-menu | the menu |
| `window.dispatch_action(..)`, `Action::boxed_clone` | 06-interaction/10-menu-items | `menu_row` |
| `tray-icon`, `TrayIconEvent::Click { .. }` with `..`, `MouseButtonState`, menu ids | 06-interaction/11-tray-icon | `tray_click_action`, `tray_menu_action` |
| (no new terms) | 06-interaction/12-boss-interactive-bar | assembles world 06 |
| `WindowHandle<T>`, `handle.update(..)` on a closed window, `activate_window`, `if let .. &&` | 07-settings/01-a-second-window | `open_settings_window` |
| `#[derive(Serialize, Deserialize)]`, `#[serde(default)]`, `BTreeMap`, `impl Default` | 07-settings/02-settings-struct | `Settings` |
| global, `impl Global`, `cx.set_global`, `cx.global::<T>()` | 07-settings/03-a-global | `StatsBar::from_settings` |
| `cx.observe_global::<T>(..)`, `Subscription`, `T::update_global(cx, ..)` (`UpdateGlobal`) | 07-settings/04-live-preview | `watch_settings` |
| window state vs saved setting, `enum Page` | 07-settings/05-sidebar-pages | `sidebar_row` |
| `type` alias for a boxed closure, `.justify_end()`, `.rounded_full()` | 07-settings/06-a-toggle | `Toggle` |
| share, range, snap with `round()` | 07-settings/07-slider-maths | `slider_value` |
| `MouseMoveEvent::pressed_button`, `canvas` prepaint to record bounds, `Rc<Cell<..>>` | 07-settings/08-a-slider | `gap_slider` |
| `HashMap/BTreeMap::insert` through `update_global` | 07-settings/09-colour-swatches | `swatch` |
| `track_focus`, `on_key_down`, `KeyDownEvent`, `keystroke.key`, `key_char` | 07-settings/10-typing-an-address | `address_box` |
| virtual list, `uniform_list`, `cx.processor`, `Range<usize>` | 07-settings/11-a-long-list | `arrange_list` |
| `Vec::remove` + `insert`, `on_drag` / `on_drop` / `drag_over` (*starter*) | 07-settings/12-drag-to-reorder | `move_slot` |
| `KeyBinding::new(keys, action, context)`, `cx.bind_keys`, `.key_context(..)` | 07-settings/13-keys-to-reorder | `bind_settings_keys` |
| `fs::create_dir_all`, `serde_json::to_string_pretty`, temp file + `fs::rename`, `?` with `io::Result` | 07-settings/14-save-to-disk | `save` |
| `serde_json::from_str`, `clamp_settings` (*starter*), `.bak` backup | 07-settings/15-fix-bad-values | `load` |
| `App::observe_global` closure with only `cx`, `cx.set_global` to swap profiles | 07-settings/16-boss-settings | `apply_settings`, `persist_settings`, `Profiles` |
| `WindowKind::Normal` without a title bar, `Option::take`, `window.remove_window()` | 08-widget/01-widget-window | `toggle_widget_window` |
| `window.start_window_move()`, a `fn` field to swap behaviour in tests | 08-widget/02-drag-to-move | `widget_header` |
| work area, distance with `abs()`, one side per axis | 08-widget/03-snap-to-edges | `snap_to_edges` |
| `Option::is_some_and`, falling back to a default place | 08-widget/04-remember-position | `place_widget` |
| `\|` joining kinds in one match arm, fixed heading order | 08-widget/05-groups | `group_of` |
| `relative(share)` lengths | 08-widget/06-usage-bars | `usage_bar` |
| widen with `as u64` before multiplying, `.max(1)` before dividing | 08-widget/07-graph-length | `history_points` |
| `.flex_wrap()`, `.items_end()`, `thread_local!` reader (*starter*) | 08-widget/08-core-bars | `core_bars` |
| a tuple of `bool`s from a `match` | 08-widget/09-styles | `parts` |
| window stack, `HWND_BOTTOM` / `HWND_NOTOPMOST` / `HWND_TOPMOST`, `WinApi::set_layer` | 08-widget/10-placement | `apply_placement` |
| extended window style bits, `\|` to set, `& !` to clear, `GetWindowLongPtrW` (*starter*) | 08-widget/11-click-through | `click_through_style` |
| `cx.defer(..)` to open a window after an update (*starter*), `observe` + `observe_global` in one view | 08-widget/12-boss-desktop-widget | `WidgetView::render`, `apply_widget_window` |
| present, ETW, PresentMon, frame generation | 09-overlay/01-how-fps-is-counted | read |
| CSV header lookup by name, `filter_map` with `?`, a fixture file with `include_str!` | 09-overlay/02-read-a-capture | `parse_capture` |
| frames over seconds | 09-overlay/03-fps | `average_fps` |
| `f64::total_cmp`, slices `[..n]`, `.max(1)` | 09-overlay/04-lows | `low_fps` |
| a window by time, `while` + `pop_front` | 09-overlay/05-ten-second-window | `FrameWindow::push` |
| median, `is_multiple_of` | 09-overlay/06-stutter | `spikes` |
| `filter(..).count()` | 09-overlay/07-base-fps | `base_fps` |
| `ferrisetw` (*starter*), nested enum patterns, match guards, `raw_os_error` | 09-overlay/08-live-frames | `live_error` |
| `eq_ignore_ascii_case` | 09-overlay/09-whats-a-game | `is_game` |
| `Pixels` arithmetic for centring | 09-overlay/10-overlay-window | `overlay_bounds` |
| `global-hotkey`: `HotKey::new`, `Modifiers`, `Code`, `HotKeyState` | 09-overlay/11-global-hotkey | `overlay_hotkey`, `hotkey_action` |
| state machine, `*self = Default`, `if let .. &&` | 09-overlay/12-auto-show | `AutoShow::step` |
| `Option::map` + `unwrap_or_default` for an empty cell | 09-overlay/13-session-summary | `SessionSummary::csv_line` |
| closures as small helpers inside a method | 09-overlay/14-benchmark | `Benchmark::tick` |
| (no new terms) | 09-overlay/15-boss-game-overlay | `FrameStats::compute`, `OverlayController::tick` |
| `.grid()`, `.grid_cols(n)` | 10-dashboard/01-dashboard-window | `DashboardView` |
| `Option::and_then` then `map` through a history's summary | 10-dashboard/02-a-card | `card_numbers` |
| `window.viewport_size()`, `f32::from(Pixels)`, `simulate_resize` | 10-dashboard/03-fit-the-window | `columns_for` |
| clamping a computed size | 10-dashboard/04-scale-the-text | `card_text_size` |
| radians, `cos` / `sin`, `std::f32::consts::PI` | 10-dashboard/05-a-gauge | `gauge_point` |
| a struct of colours chosen by `match` | 10-dashboard/06-themes | `theme` |
| `filter` + `contains` against a settings list | 10-dashboard/07-layout-editor | `visible_cards` |
| `#[derive(Serialize)]` for an API, enum variants as JSON strings | 10-dashboard/08-snapshot-json | `web_snapshot` |
| axum `Router`, `State`, `Json`, a second runtime next to GPUI, `tower::ServiceExt::oneshot` | 10-dashboard/09-web-server | `stats` |
| `Query`, `StatusCode::UNAUTHORIZED`, constant-time compare, `getrandom` (*starter*) | 10-dashboard/10-secret-key | the key check |
| `Ipv4Addr::LOCALHOST` / `UNSPECIFIED`, `SocketAddr::from((ip, port))` | 10-dashboard/11-local-only | `bind_addr` |
| `std::thread::spawn` + `tokio::runtime::Runtime`, `axum::serve`, `observe_self` (*starter*) | 10-dashboard/12-boss-dashboard | `DashboardView::render`, `serve_dashboard` |
