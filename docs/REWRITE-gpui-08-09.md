# GPUI worlds 08 (widget) and 09 (overlay): rewrite map

Audit: `docs/audit-2026-10-24/gpui-06-11.md`. Ledger rows: scratchpad `ledger/gpui-08-09.md` (replaces the 08/09 rows in `docs/tracks/gpui-terms.md`).

Across both worlds:
- The lesson's code block is no longer the solution line. Each one shows a sibling: a different field, stat or enum (the Settings window's `activate_window` for 08-01, a vertical centre for 09-10, fastest-first sorting for 09-04, and so on). The last hint keeps the exact code.
- `// TODO:` comments and `todo!` texts say the goal in words, with no code in them.
- Two points per exercise, with varied answer positions. The read quests' answers are varied too.
- Syntax the audit flagged now gets a card where it first appears. Calling a field that holds a function, `(field)(..)`, is in 08-02. `*`, `**` and `&&T` are in 08-12, 08-13, 09-02 and 09-06, and `*self =` is in 09-12; these link to `rust/02-ownership/09-follow-the-arrow`. `.w_full()` is in 08-06, `!` on bits in 08-11, `parse()` inference in 09-02, `[..n]` in 09-04 and `Option::map` in 09-13.
- Thread refreshers at the point of use name Rust world 07 (Concurrency) in words, with no link, because that world is being renamed. The ETW callback thread and `Send + Sync + 'static` are in 09-08. `Arc<Mutex>`, `lock()`, poisoning and `into_inner` are in 09-10.
- "On this PC" became "on your PC" (09-08, 09-15). No R5 or spike-plan wording.

## 08-widget: 12 quests became 15

| old | new |
|---|---|
| 01-11 | same ids, lessons rewritten (sibling code blocks, `handle.update` inputs named, fn-pointer field and `(f)(x)`, copy-then-change in snap, `.w_full`/`.h_full`, tuples, `match` as a value, `!` bits). XP/minutes lowered for one-line tasks (06, 09, 10). |
| — | **12-stats-in-bar-order** (new bridge exercise): `bar_stats(order) -> Vec<StatKind>`, filter_map + match + `*kind` |
| — | **13-skip-empty-headings** (new bridge exercise): `grouped(stats) -> Vec<(Group, Vec<StatKind>)>`, loop over `GROUPS`, `continue` |
| — | **14-read-then-let-go** (new read): why `self.bar.read(cx)` must end before `widget_header(cx)` (E0502), the block that ends it |
| 12-boss-desktop-widget | **15-boss-desktop-widget** (slug kept). `widget.rs` now also holds `bar_stats` and `grouped` (starter and solution). The hints no longer use `Option::filter`, `and_then`, `.map(&row)`, `unwrap_or_else` or `collect::<Vec<_>>()`. They use a `for` loop over `grouped(&bar_stats(..))`, a row closure, and `el = el.child(..)` inside `if let` chains. The solution is unchanged. |

Code in the new quests: 12 and 13 start from 11's solution and move the previous quest's function into `widget.rs`. Each adds one pure function and two `#[test]`s to `lib.rs`. 13 continues from 12, and the boss gains both functions in `widget.rs`.

## 09-overlay: 15 quests, same ids

All lessons were rewritten as described above. 09-04 now mentions the `is_empty` early return and slices. 09-15's peak-temperature hint uses a three-arm `match` instead of `Option::or`. The boss lesson's code block shows the lowest ping, not `self.auto.step`.

## Id changes and references

- `gpui/08-widget/12-boss-desktop-widget` → `15-boss-desktop-widget`. Updated `content/tracks/gpui/worlds/09-overlay/01-how-fps-is-counted/quest.toml` `requires`.
- **Not updated (parent):** `docs/tracks/gpui.md` (line ~298 world 08 table) and `docs/tracks/gpui-terms.md` still name `12-boss-desktop-widget`.
- No other `requires` name an 08 or 09 quest except `10-dashboard/01` → `09-overlay/15-boss-game-overlay`, which is unchanged.
