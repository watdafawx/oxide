# GPUI world 07 (Settings): rewrite map

Rewritten in the golden shape against `docs/audit-2026-10-24/gpui-06-11.md`. Code blocks show a
similar example, not the solution; TODOs say the goal in words; quiz answers vary. Term ledger:
`docs/tracks/gpui-terms.md` (07 section replaced). GPUI does not build on the Linux sandbox; the
one code change (07-a-toggle) is **not compiled**.

## 16 quests became 17

| old | new |
|---|---|
| 01-a-second-window | 01-a-second-window (the `handle.update` closure's three inputs named; `&mut App` and `..Default::default()` glossed; the "Not focus-free" card cut) |
| 02-settings-struct | 02-settings-struct (all twelve defaults stated; why `BTreeMap`: stable key order in the JSON; a `#[serde(default)]` thinking point) |
| 03-a-global | 03-a-global |
| 04-live-preview | 04-live-preview ("clone, then apply" restated accurately: the clone keeps nothing borrowed from `cx`) |
| 05-sidebar-pages | 05-sidebar-pages (`cx.listener` on another view) |
| (new, read) | 06-handlers-without-a-view: `App` vs `Context<T>` vs `Window`, why a component's click is a plain closure with `&mut App`, type alias, `Box<dyn Fn(..) + 'static>` (the audit's rewritten card) |
| 06-a-toggle | 07-a-toggle: the 12-call style chain is now in the starter (and glossed in one card); the task is the click, `.on_click(move \|_, window, cx\| on_toggle(!on, window, cx))`. 40 → 30 XP, 6 → 4 min |
| 07-slider-maths | 08-slider-maths |
| 08-a-slider | 09-a-slider (`Rc<Cell>` refresher pointing at rust 06; the example is the right-click listener from world 06) |
| 09-colour-swatches | 10-colour-swatches |
| 10-typing-an-address | 11-typing-an-address (60 → 50 XP) |
| 11-a-long-list | 12-a-long-list (`cx.processor` gets `this` = `SettingsWindow`; `collect()` typed by the return) |
| 12-drag-to-reorder | 13-drag-to-reorder (keeps the fixed "`move_up` from Readings swapped neighbours"; links rust 03 remove/insert) |
| 13-keys-to-reorder | 14-keys-to-reorder |
| 14-save-to-disk | 15-save-to-disk |
| 15-fix-bad-values | 16-fix-bad-values |
| 16-boss-settings | 17-boss-settings (`App` closure tied to quest 06) |

Code change: 07-a-toggle/starter now contains the track's whole look (copied from the solution);
only `.on_click(..)` is left as a TODO, with `let _ = &on_toggle;` to keep it used. TODOs reworded
in 01, 03, 12, 16.

## Ids and references

- Every quest from old 06 on moved up by one; the boss is `17-boss-settings`.
- `08-widget/01-widget-window`'s `requires` updated to `07-settings/17-boss-settings`.
- `01-a-second-window` requires `06-interaction/16-boss-interactive-bar`.
