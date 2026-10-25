# Task manager track: term ledger

Every egui, GPUI, web and project term the track introduces, in order, with the quest that does.
A lesson may use only rows at or above its own quest, plus the Rust track's ledger
(`docs/tracks/rust-terms.md`) up to `rust/05-traits/21-boss-shape-renderer`, which this track
requires. Helpers a starter contains are marked *starter*: shown, not taught.

| term | quest | how it is introduced |
|---|---|---|
| the project: a task manager, one row per process; three routes (egui, GPUI, Tauri) | 00-pick-a-gui/01-why-a-new-one | read |
| `Process { pid, name, cpu, memory_bytes }` (*starter*), `ProcessRow`, **view model** | 00-pick-a-gui/02-the-row-model | `from_process` |
| `{:.1}` one decimal in `format!`, `u64 / 1_048_576` drops the remainder, `_` in number literals | 00-pick-a-gui/02-the-row-model | CPU and memory text |
| **immediate mode**, `ui` function run on demand, a click is an answer, `request_repaint_after`, egui's visible-rows table | 00-pick-a-gui/03-immediate-mode | read |
| `ui.label(..)`, `ui.horizontal(\|ui\| ..)`, `.as_str()` for a label, egui_kittest `Harness::new_ui`, `get_by_label`, `.rect()` | 00-pick-a-gui/04-egui-row | `row_ui` |
| `ui.button(..)`, **response**, `.clicked()`, `sort_by_cpu` (*starter*), `Harness::new_ui_state`, `click()`, `run()` | 00-pick-a-gui/05-egui-sort-button | the CPU button |
| GPUI **view**, `Render`, `div()`, `.flex()`, `cx.notify()`, `.on_click(..)` callback, `uniform_list`, actions | 00-pick-a-gui/06-views-and-notify | read |
| `impl Render`, `render(&mut self, window, cx)`, `.child(..)`, `.clone()` text into a box, `.debug_selector(..)`, `.gap_4()`, `add_window_view`, `debug_bounds` | 00-pick-a-gui/07-gpui-row | `RowView` |
| `.id(..)` (*starter*), `cx.listener(\|view, _, _, cx\| ..)`, `cx.notify()` in a listener and in `set_processes`, `simulate_click`, `read_with`, `update` | 00-pick-a-gui/08-gpui-click | the CPU header |
| web view, WebView2, `#[tauri::command]`, JSON across the bridge, events, Leptos, Dioxus, WebAssembly | 00-pick-a-gui/09-the-web-route | read |
| working set, private memory, idle CPU, renderer (wgpu vs glow) | 00-pick-a-gui/10-the-numbers | read |
| keep rules in plain Rust, GUI only draws | 00-pick-a-gui/11-pick-one | read |
| `SortBy`, `TaskList::visible`, `.to_lowercase()`, `.contains(..)`, `.filter(..)`, `sort_by_key`, `std::cmp::Reverse`, `.enumerate()`, `.children(..)`, `text_edit_singleline` (*starter*), `query_by_label`, `state_mut` | 00-pick-a-gui/12-boss-one-model-two-faces | boss lesson and hints |
