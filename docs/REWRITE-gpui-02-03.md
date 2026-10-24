# GPUI worlds 02 (Views) and 03 (Live): rewrite map

Both worlds were rewritten in the golden shape (`docs/CONTENT.md`, `content/golden/`). Audit:
`docs/audit-2026-10-24/gpui-00-05.md`. Ledger rows: the scratchpad `ledger/gpui-02-03.md`, which
replaces every 02-views and 03-live row of `docs/tracks/gpui-terms.md`. No flavours in gpui.

Throughout: each lesson's code block now shows a similar example (another element, field or
value: `.flex_col().gap_1()`, `.w(px(360.))`, a `stale` colour, a divider *height* match, a
`total_memory` read), not the answer line. "Your turn" and every `// TODO:` say the goal in words;
only the last hint has the exact code. Quiz and point answers are spread across positions.

## 02-views: 12 quests became 14

The element/view model, the `render` signature, the builder chain and why `debug_selector` takes a
closure are now taught in 00-setup (`05-what-a-view-is`, `06-the-first-pill`, rewritten in the same
round), so world 02 builds on them instead of repeating them.

| old | new |
|---|---|
| 01-a-row | 01-a-row (`gap_N` is a scale step, not pixels; Taffy/Tailwind for the curious) |
| 02-pixels | 02-pixels (`.w`, why `32.` has a dot) |
| (new) | **03-the-spacing-scale** (read): scale steps and rem, `p_N`/`px_N`/`py_N` padding, the `px(..)` vs `.px_2()` trap |
| 03-colours | 04-colours (text colour runs down to children) |
| 04-children-from-a-list | 05-children-from-a-list (a function name as `.map`'s argument) |
| (new, split from 05) | **06-name-the-parts** (exercise): the learner adds the named label and value boxes to `pill_element`. Teaches `{kind:?}`, why `let kind = pill.kind` (format braces take a plain name, not a path) and `move` (honest: not required here, since `debug_selector` runs its closure at once; a habit for closures GPUI keeps, like listeners) |
| 05-a-pill-component | 07-a-pill-component (now one idea: move `pill_element`'s body into `PillView::render`; view vs component and when to choose; `&mut App` vs `&mut Context<Self>`). Starter changed: `pill_element` already has the named boxes (from 06) and `StatsBar::render` already builds `PillView`s |
| 06-labels-or-not | 08-labels-or-not ("Breaking the chain" now leans on the builder) |
| 07-dividers | 09-dividers (`AnyElement` card: an iterator gives one type, the type is forgotten like `Box<dyn Trait>`; `.enumerate()` and why dividers are numbered) |
| 08-two-lines | 10-two-lines (why the column ends in `.into_any_element()`) |
| 09-the-pill-background | 11-the-pill-background (`.px_2()` is padding, links to 03) |
| 10-numbers-that-sit-still | 12-numbers-that-sit-still |
| 11-text-size | 13-text-size (text size cascades) |
| 12-boss-static-bar | 14-boss-static-bar (slug kept). New card "Why a function again": the boss keeps `pill_element` as a function on purpose, because world 06 gives each pill click/hover handlers built from the bar's `cx: &mut Context<StatsBar>`, which a component's `render` (`&mut App`) does not get; every later world's starter uses that function. Code block is now the `if self.two_lines { .. } else { .. }` skeleton. Hint 2 explains `*style` |

## 03-live: 13 quests became 15

| old | new |
|---|---|
| 01-state-in-a-view | 01-state-in-a-view (view = struct whose fields are its state, `render` only reads; ties back to 00/05) |
| 02-entity | 02-entity ("you have seen it": `main.rs` has called `cx.new` since 00/05; a view is an entity whose type implements `Render`; the `|_|`; entity store for the curious) |
| 03-update-and-notify | 03-update-and-notify (code block is now the failing test output) |
| (new) | **04-which-cx** (read): `App`, `Context<T>`, `Window`, `AsyncApp`, test contexts, and why there are several |
| 04-a-fake-sensor | 05-a-fake-sensor |
| (new, split from 05) | **06-run-it-later** (exercise): `cx.spawn` with an **async closure** (card at point of use: like the Async world's `async move { }` blocks, with two arguments GPUI passes), `this` named as a `WeakEntity`, `AsyncApp`, `.ok()`, `.detach()` linked to `on_window_closed(..).detach()` from 00/04. Says plainly the bar reads once and quests 08-09 make it repeat |
| 05-spawn-and-keep | 07-keep-the-task (drop cancels, same rule as the subscription; detach would outlive the bar; `_poll` and the leading `_`) |
| 06-a-timer | 08-a-timer (executors; why the block cannot borrow `self`) |
| 07-every-second | 09-every-second (why the handle is weak: no bar/task cycle) |
| 08-update-interval | 10-update-interval (new cards: `with_interval` as a builder method, `mut self`; the boss needs it) |
| 09-redraw-only-on-change | 11-redraw-only-on-change |
| 10-history-per-stat | 12-history-per-stat |
| 11-real-cpu | 13-real-cpu ("this PC" became "your PC") |
| 12-real-ram | 14-real-ram |
| 13-boss-live-bar | 16-boss-live-bar (slug kept; code block is a four-step tick skeleton; hint 4 points at 10's builder) |

The notify/redraw model of 03 is unchanged.

## Id changes and references

- `gpui/02-views/12-boss-static-bar` → `14-boss-static-bar`: updated in `content/tracks/gpui/worlds/03-live/01-state-in-a-view/quest.toml` (only that token; the rust ids on the same line are untouched).
- `gpui/03-live/13-boss-live-bar` → `16-boss-live-bar`: updated in `content/tracks/gpui/worlds/04-sensors/01-what-needs-admin/quest.toml` (only that token).
- All other quests were renumbered; nothing else referenced them. `02-views/01-a-row` has no `requires`, so it still chains to the last quest of 01-readings; `03-live/01` keeps its `requires`.
- `03-live/05-spawn-and-keep` was renamed to `07-keep-the-task`.
- Cargo package names are unchanged for existing quests; the new ones are `kinetik_02_name_the_parts` and `kinetik_03_run_it_later`.
- **Not updated:** `docs/tracks/gpui.md` section 3 still lists the old 02/03 quest tables.

## Not compiled

GPUI does not build on this Linux sandbox, so these code changes were checked by reading only:
`02-views/06-name-the-parts` (new; starter/solution from 05's solution), `02-views/07-a-pill-component`
(starter changed), `03-live/06-run-it-later` (new; from 07's starter). All other code edits are
`// TODO:` comment and `todo!(..)` message wording.
