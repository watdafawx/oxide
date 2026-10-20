# Blind playtest, strict pass: Rust worlds 04-06 (2026-10-20)

Follow-up to `PLAYTEST-2026-10-20-rust-03-06.md`, method from `docs/PLAYTEST-CHECKLIST.md` ("Explain, then ask"). Scope: `rust/04-errors` (10 quests), `05-traits` (12), `06-smart-pointers` (11) = 33 quests, 5 of them quizzes. Both flavours were checked: the games lesson against the games solution, the plain lesson against the plain solution.

## Method and honesty notes

- **I am an LLM, not a beginner.** I know all of this Rust, so "solved" below means "I could write the solution from lesson + labels + starter + tests without outside knowledge of the *words* it needs", judged by reading, not by a stopwatch. 0 hints were opened.
- I did **not** copy every quest to `%TEMP%\oxide-playtest` and run `cargo test` on my own attempt. I read lesson, labels, starter and tests, then diffed `solution/` against `starter/` and listed every call, keyword, macro, pattern and import the solution adds that this lesson or worlds 00-03 (and earlier quests of the same world) never showed. `content-check` (starter fails / solution passes, both flavours) is green for all three worlds after the edits. Edits touched only lessons, TODO comments, hints, quiz choices and minutes, never code or tests.
- Rule for "taught": shown in this lesson, in an earlier quest of the Rust track, or in worlds 00-03 lessons (I re-read all of them first).
- Nothing is broken. Every issue below is a hidden requirement: a name that only appears in a hint, a TODO or the solution.

## Verdict summary

| | ok | unclear (fixed) | broken |
|---|---|---|---|
| 04-errors (10) | 4 | 6 | 0 |
| 05-traits (12) | 9 | 3 | 0 |
| 06-smart-pointers (11) | 6 | 5 | 0 |
| **total (33)** | **19** | **14** | **0** |

Commits: `Rust world 04`, `Rust world 05`, `Rust world 06` (one per world, explicit paths).

## Per quest

Columns: quest, solved, hints used, verdict, what was missing (used before taught), what was fixed.

| quest | solved | hints | verdict | missing | fixed |
|---|---|---|---|---|---|
| 04-errors/01-custom-error-enum | yes | 0 | unclear | `split_once`, `ok_or`, `is_empty`, `.next()` on the word iterator used before taught (games solution); TODO said what, not how | Both lessons got a "text tools" block; games TODO names `split_whitespace().next()`/`split_once`; 10 -> 12 min |
| 04-errors/02-from-and-into | yes | 0 | ok | nothing: `From`, `?` and the loop are taught | - |
| 04-errors/03-panic-vs-result | quiz | - | unclear | right answers were the longest choice in 3 of 4 questions | choices rebalanced, both flavours |
| 04-errors/04-box-dyn-error | yes | 0 | ok | `split_once`/`ok_or` now taught in 01 | - |
| 04-errors/05-thiserror | yes | 0 | unclear | closure `\|(name, _)\| *name == x` on `find` over tuples (double reference) never shown | lesson example with `find(\|(who, _)\| *who == ..)` in both flavours |
| 04-errors/06-anyhow | yes | 0 | ok | - | - |
| 04-errors/07-modules | yes | 0 | ok | - | - |
| 04-errors/08-files-in-modules | yes | 0 | ok | closure pattern `\|(name, version)\|` for `max_by_key`/`map` only in hint | TODO now names it |
| 04-errors/09-cargo-and-crates | quiz | - | unclear | right answer to the semver question was the longest | choices rebalanced, both flavours |
| 04-errors/10-boss-list-manager | yes | 0 | unclear | `Default`/`Self::default()`, `iter().map().max()`, `iter_mut`, `fs::write`, `path.display()`, list-line format had no hint | "More tools" bullet in both lessons, extra hint 9 for List |
| 05-traits/01-first-trait | yes | 0 | ok | - | - |
| 05-traits/02-default-methods | yes | 0 | ok | - | - |
| 05-traits/03-generic-functions | yes | 0 | unclear | plain `count_matching`: `filter(\|x\| *x == target)` double reference not hinted; games lesson's `weakest` example is close to `strongest` (kept: it teaches the bound, solution still needs the flip and `roster`/`sum`) | plain TODO names the dereference |
| 05-traits/04-generic-structs | yes | 0 | unclear | `PartialOrd` / `Display` as bounds were named in the TODO but never explained | one bullet in both lessons |
| 05-traits/05-static-vs-dynamic-dispatch | quiz | - | ok | right answers are not the longest; each is decided by a lesson sentence | - |
| 05-traits/06-trait-objects | yes | 0 | ok | - | - |
| 05-traits/07-std-traits | yes | 0 | ok | - | - |
| 05-traits/08-operator-overloading | yes | 0 | ok | - | - |
| 05-traits/09-iterator-trait | yes | 0 | ok | `rem_euclid` is taught in the lesson | - |
| 05-traits/10-associated-types-and-consts | yes | 0 | ok | - | - |
| 05-traits/11-impl-trait | yes | 0 | ok | - | - |
| 05-traits/12-boss-shape-renderer | yes | 0 | unclear | `max_by`, `sort_by`, `any`, `as f64`, `PI` only in hints / solution | "Tools" bullet in both lessons (generic example, not the solution) |
| 06-smart-pointers/01-box | yes | 0 | unclear | recursion (a function matching and calling itself on `rest`) never named; the lesson's matching sentence was the whole pattern | recursion paragraph in both lessons; 4 -> 6 min |
| 06-smart-pointers/02-deref-and-drop | yes | 0 | ok | title says "drop" but only `Deref` is taught here; `Drop` is quest 10 (left, renaming would break ids/saves) | - |
| 06-smart-pointers/03-rc | yes | 0 | ok | block-as-expression (`let inside = { .. }`) is optional; a `mut` + assignment also works | - |
| 06-smart-pointers/04-interior-mutability | quiz | - | ok | right answers are not the longest (Q1 is the middle one, Q2 the shortest); each decided by a lesson sentence | - |
| 06-smart-pointers/05-refcell | yes | 0 | unclear | `is_err()` only in the hint | bullet in both lessons |
| 06-smart-pointers/06-rc-refcell-graph | yes | 0 | unclear | TODO for the recursive total did not say how to open the `Rc<RefCell<..>>`; `sum::<u32>()` turbofish follows `collect::<Vec<_>>()` | TODO now says `borrow()` first, then add the children's totals |
| 06-smart-pointers/07-weak | yes | 0 | unclear | assigning a new `Weak` into a `RefCell` field (`*x.borrow_mut() = ..`) was only in hint 1 | lesson bullet in both flavours |
| 06-smart-pointers/08-cow | yes | 0 | ok | - | - |
| 06-smart-pointers/09-arc-preview | yes | 0 | ok | slicing an `Arc<Vec>` is covered by a lesson bullet | - |
| 06-smart-pointers/10-drop-and-cleanup | yes | 0 | ok | - | - |
| 06-smart-pointers/11-boss-tree-editor | yes | 0 | unclear | `while let`, `Option::cloned`, `String::repeat`, `push_str`/`push`, `Vec::reverse`, `HashMap::retain`/`[&k]` only in hints or the solution | "More small tools" bullet in both lessons |

## Deliberately left

- `06-smart-pointers/02-deref-and-drop` teaches only `Deref`; `Drop` arrives in quest 10. Renaming the folder would change quest ids.
- `05-traits/03-generic-functions` (games): the lesson's `weakest` example is the mirror of `strongest`. Kept: it is the bound lesson, and the quest still needs `roster` (map/collect/join) and `total_health` (sum).
- Minutes: only `04/01` (10 -> 12) and `06/01` (4 -> 6) changed. The bosses already carry 45-50 min and the added tool bullets turn guesses into reading, so I left them.
- Tests and solutions are untouched; the first builds of `04/05` and `04/06` still download `thiserror` and `anyhow` (cached here, offline first run not checked).

## Patterns worth copying into new quests

1. A helper that appears only in `// TODO:` or in a hint is "used before taught". Put each in a one-line bullet in the lesson, with an example that is not the quest's answer (`split_once`, `ok_or`, `is_err`, `max_by`, `retain` on maps).
2. A closure over a tuple or a reference of a reference (`find(|(k, _)| *k == key)`) needs one sentence the first time it appears, in whatever world.
3. Quiz choices: the right one must not be the longest. Both fixed quizzes (04/03, 04/09) had the right answer as the longest choice.
