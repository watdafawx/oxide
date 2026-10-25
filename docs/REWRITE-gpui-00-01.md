# GPUI worlds 00 and 01: rewrite map

Both worlds were rewritten in the golden shape (`docs/CONTENT.md`). Each quest now has an unlabeled intro and kicker cards. The one code block shows a similar example, not the solution line. TODOs and "Your turn" say the goal in words, and only the last hint has the exact code. Quiz answer positions are varied. Audit: `docs/audit-2026-10-24/gpui-00-05.md`. Ledger rows: the scratchpad `ledger/gpui-00-01.md`, which the parent merges.

## 00-setup: 6 quests became 7

| old | new |
|---|---|
| 01-what-gpui-is | 01-what-gpui-is. A new "What you need first" card states the prerequisite: Rust through Traits now, and Smart pointers, Concurrency and Async later. A third quiz question checks it. `requires` is unchanged. |
| 02-msvc-toolchain | 02-msvc-toolchain (unchanged) |
| 03-first-build | 03-first-build. The code block is now a `sysinfo` line of the same shape, and "features" is named. The Cargo.toml TODO says the goal in words. |
| 04-hello-window | 04-hello-window. The code block is a 200 by 50 size. New explanations: `cx` as `App` ("you will meet more kinds of cx"), `..Default::default()`, `Bounds::centered`, the subscription / drop-cancels / `.detach()` idea, and why `px(360.)` has a dot (`f32`). Points now ask about `detach` and the dot instead of repeating the answer. |
| 05-a-view (view, Render, div, child, debug_selector, code block = answer) | **05-what-a-view-is** (new read quest): a view is a struct GPUI keeps, made by `cx.new` in `main.rs`; the `Render` trait; the `render` signature (`self`, `window`, `cx: Context<Self>` vs `App`); an element is one frame's description, thrown away; `impl IntoElement`; a for-the-curious card on "no virtual-DOM diff". **06-the-first-pill** (the old exercise, same test): the builder chain (each call takes and returns the box), boxes in boxes, why `debug_selector` takes a closure (it runs only with test-support) and what `.into()` makes (`String`; GPUI 0.2.2's closure returns `String`, not `SharedString` as the audit said). The code block is a `pill-ram` box. |
| 06-it-runs | 07-it-runs. The quiz's "when does render run" answer now says open / resize / when asked, not "whenever". |

## 01-readings: 13 quests became 15

| old | new |
|---|---|
| 01–10, 12 | same ids. Each code block now shows a similar example on different data: `RamUsed => "RAM"`, `"n/a"`, `{value:.1}`, a seconds-to-hours `while`, `format!("BAT {}", format_percent(..))`, a `(used, total)` pair match, a match inside an arm, an alerts `VecDeque`, a `highest` loop, `pair.swap(0, 1)`. The learner now works out the line from the example. For-the-curious lines cover depth for experienced devs: `\|` arms, round-half-to-even, KB vs KiB, saturating `as`, ring buffer, `f64` is not `Ord` / `fold`, the `usize` underflow behind `move_up`'s guard. |
| 10-min-avg-max | The starter no longer has the empty check, so `summary_of_empty_is_none` fails at first. A new card, "Empty is a trap", explains what goes wrong: `f64::MAX` and NaN. |
| 11-bar-order (type 11 values from prose) | The starter now lists the eight stats, and the learner places the three dividers between the groups. The objectives are now `default_order_has_three_line_dividers` and the new `dividers_sit_between_the_groups`. `default_order_starts_with_cpu` was removed, because it would pass in the starter. |
| 13-boss-stat-model (`Option::take`, `let .. else`, type alias, `*kind`, `&'static str` all first seen in the boss) | **13-two-lines** (new bridge): `two_line_columns` with `Option::take`. It meets E0382 first and also explains `&'static str`, `clone` and `*style`. **14-no-data-yet** (new bridge): `hover_summary` with `let .. else`, pointing at `reading.rs`, which has used it since quest 04. **15-boss-stat-model** (slug kept, renumbered): `bar_items` only. Its cards teach the type alias, `.get().copied()` and `*kind`. The code block is a shape skeleton. The objectives are the first five old ones plus `default_bar_on_two_lines`, which uses `two_line_columns`. It is now 30 min / 300 XP. |

The boss solution still has the same public items and signatures (`Snapshot`, `Pill`, `BarItem`, `Column`, `bar_items`, `two_line_columns`, `hover_summary`, the modules), so world 02's starters still continue from it.

## Id changes and references

- `00-setup/05-a-view` → `00-setup/06-the-first-pill` (and new `05-what-a-view-is`). `00-setup/06-it-runs` → `00-setup/07-it-runs`. The only reference was `01-readings/01-what-a-stat-is` `requires`, which is updated.
- `01-readings/13-boss-stat-model` → `15-boss-stat-model`. No `requires` names it. World 02's first quest chains to it by default.
- `00-setup/01-what-gpui-is` keeps its id and its `requires = ["rust/05-traits/21-boss-shape-renderer"]`. `src/lib/mock.ts` names it, and that reference still resolves.

## For the parent (outside this world's scope)

- **Track blurb** (`content/tracks/gpui/track.toml`): add the prerequisite, for example "Needs the Rust track through Traits."
- `docs/tracks/gpui.md` section 3: the world 00 and 01 tables still list the old quests (6 and 13). Update them from the tables above.
- World 02's new `01-what-an-element-is` recaps view, element and builder. World 00 now introduces all three (05 and 06), so 02/01 now reads as a recap. That is fine, but its wording "since the first pill" matches `06-the-first-pill`.

## Build status

- 01-readings: `content-check` passed for all 15 quests (on a minimal content copy, because other worlds broke the shared catalog mid-edit).
- 00-setup: 03 and 04 passed the full check with libc pinned. 06-the-first-pill was **not compiled**: the disk filled up twice. Its code change is a TODO comment only, and the solution is unchanged.
