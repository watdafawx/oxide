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
