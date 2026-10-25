# GPUI world 06 (Interaction): rewrite map

Rewritten in the golden shape against `docs/audit-2026-10-24/gpui-06-11.md`. Each lesson's code
block now shows a similar example (another element, field, button or action), not the solution
line; the TODOs state the goal in words; only the last hint has the exact code. Quiz answers
are no longer all index 0. Term ledger rows: `docs/tracks/gpui-terms.md` (06 section replaced).

GPUI quests do not build on the Linux sandbox, so the code changes below were made by hand and
are **not compiled**. Each one is small and listed here.

## 12 quests became 16

| old | new |
|---|---|
| (new, read) | 01-what-a-listener-is: the four inputs of `cx.listener(\|this, event, window, cx\|)`, why a wrapper, `move` + `Copy`, elements rebuilt every frame (the audit's rewritten card) |
| 01-a-click | 02-a-click (`SharedString` glossed; example on another element) |
| 02-left-click-action | 03-left-click-action (`Rc<RefCell>` refresher card pointing at rust 06) |
| 03-hover | 04-hover (`*hovered` deref card; the naive version shown, then the race) |
| 04-hover-card | 05-hover-card (tooltip is a plain closure, not a listener; why clone inside an `Fn`) |
| 05-graph-points | 06-graph-points |
| 06-draw-the-graph | 07-draw-the-graph: **the task is now the canvas paint loop** (`PathBuilder`, `move_to`/`line_to`, `build`, `paint_path`); the starter adds the child and leaves `todo!()` in the paint closure, so the test panics until it is written. 60 → 50 XP |
| 07-top-processes | 08-top-processes (`ProcessUse` pair explained; example sorts by name) |
| (new, bridge) | 09-processes-when-it-opens: a `fn()` field and calling it `(top_processes)()`; struct update `..card.clone()` as "for the curious". Before, this wiring appeared silently in the next starter. Starter = old 07 solution; new test `card_lists_top_processes` (from the boss) |
| (new, read, bridge) | 10-who-hears-an-action: focus, `FocusHandle`, `track_focus`, `focus_in`, the `match &this.ui.focus` the boss needs, bubbling |
| 08-actions | 11-actions: the `OpenTaskManager` handler is now in the starter as the model; the learner writes `OpenSettings` and `Exit`; the type annotation as router is explained |
| 09-right-click-menu (two ideas) | 12-a-menu-on-top (`when_some`, `anchored`, `deferred`; new test `menu_draws_at_menu_at` sets `menu_at` directly) and 13-right-click (`on_mouse_down(Right)`, `MouseDownEvent`; the old two tests). Folder `09-right-click-menu` renamed to `13-right-click` |
| 10-menu-items | 14-menu-items (`Box<dyn Action>` and `boxed_clone` explained) |
| 11-tray-icon | 15-tray-icon (60 → 50 XP; the two `MouseButton`s named) |
| 12-boss-interactive-bar | 16-boss-interactive-bar (hints point at the quests that taught each piece; tooltip hint uses the taught `let mut card` form) |

Code changes (all hand-made, not compiled):
- 07-draw-the-graph/starter: `.child(graph(..))` added; paint body replaced by a TODO, `let _ = (points, window);` and `todo!()`.
- 09-processes-when-it-opens: copied from 08-top-processes/solution (crate renamed `kinetik_06_processes_when_it_opens`, also in `main.rs`); tooltip TODO in the starter, `let mut card = card.clone(); card.top = (top_processes)();` in the solution; test replaced.
- 11-actions/starter: `OpenTaskManager` handler moved from TODO into the starter.
- 12-a-menu-on-top: copied from old 09 starter (crate `kinetik_06_a_menu_on_top`); no pill right-click, `MouseButton`/`MouseDownEvent` imports dropped, new test.
- 12 and 13: `menu_row`'s `on_click` is no longer pre-filled (old 09 had it filled although 10 asks for it).
- 13-right-click/starter = 12 solution + imports + pill TODO.
- TODO comments reworded as goals in 02, 05, 08, 14.

## Ids and references

- `01-a-click`'s `requires` moved to the new 01 and now names `05-taskbar/12-boss-docked-bar` (the 05 boss was renumbered by the 05 rewrite).
- `07-settings/01-a-second-window` now requires `06-interaction/16-boss-interactive-bar`.
- No other `requires` or links named a world 06 id.
