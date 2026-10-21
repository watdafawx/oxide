# Rust worlds 00 and 01: rewrite map

Both worlds were rewritten in the golden shape (`docs/CONTENT.md`, `content/golden/`): one idea
per quest, an unlabeled intro, kicker cards, one code block, a short Your turn, two hints and
one or two points that ask for what the card above just showed. Every exercise has a plain twin
(textbook objects: crates, sheets, boxes, pallets, orders) and a games flavour (the crafting
calculator). Both have the same objectives in the same order. Term ledger: `docs/tracks/rust-terms.md`.

## 00-tutorial (3 quests, same ids)

| old | new |
|---|---|
| 01-hello | 01-hello (explains the save loop, `todo!()`, `format!`; no `fn` teaching) |
| 02-fix-it | 02-fix-it (plain twin now `max_tag_len`, games `max_name_len`) |
| 03-boss-name-tag | 03-boss-name-tag (cards instead of a bullet list) |

## 01-foundations (12 quests became 28)

| old | new |
|---|---|
| 01-variables (`let`, `mut`, shadowing) | 01-a-name-for-a-number, 02-the-count-changes, 04-same-name-new-value |
| 02-numbers (types, `f64`, `as`, wrapping) | 05-how-big-can-a-number-be, 06-full-stacks-and-leftovers, 07-stop-at-zero, 08-halves-and-decimals, 09-turn-one-number-into-another |
| 03-functions | 03-a-rule-you-can-reuse (after `let`/`mut`, as in the golden) |
| 04-if-and-loops (`if`, `for`, `while`, `loop`/`break`, `%`) | 10-either-or, 11-count-the-steps, 12-repeat-until |
| 05-tuples-and-arrays | 13-a-pair, 14-a-row |
| 06-how-rust-thinks (read) | 15-how-rust-thinks (read) |
| 07-structs (struct, impl, `new`, `&mut self`) | 16-a-thing-with-parts, 17-give-it-a-verb, 18-start-from-scratch, 19-change-it-in-place |
| 08-enums | 20-one-of-a-few, 21-choices-that-carry-data |
| 09-match (ranges, guards, `if let`, `\|`) | 22-match-a-range, 25-only-if-there-is-one (`match` itself moved into 20) |
| 10-option (`Some`, `unwrap_or`, `map`) | 23-maybe-nothing, 24-a-fallback |
| 11-result (`Ok`/`Err`, `match`, `expect`) | 26-worked-or-not, 27-handle-both-paths |
| 12-boss-guessing-game (games: crafting calculator) | 28-boss-crafting-calculator (plain twin: packing planner) |

The first three games quests are the approved goldens; their plain twins are new.

## Cut on purpose (not taught in 00 / 01 any more)

These were in a dense quest and are not needed for a beginner's first pass. If a later world
needs one, teach it there as its own quest.

- `wrapping_add` and bit-truncating casts (`300 as u8`). `as` now teaches `u32 -> f64` and `f64 -> u32`.
- `loop { break value }` and a `mut` parameter (`fn f(mut n)`). `while` makes a `let mut x = x;` copy.
- Slices (`&[i32]`) and array destructuring. Arrays are indexed and `.len()`-ed only.
- Match guards (`n if n < 0`) and `A | B` patterns.
- `Option::map` and closures (03-collections/06-closures teaches closures), and `expect` (04-errors/03-panic-vs-result teaches it).
- The old boss "toolbox" (Vec, BTreeMap, closures, `split_once`, `div_ceil`): the boss now teaches `Vec`,
  `return`/`break` and recursion in its cards, and the rest is given in the starter or named in a `// TODO:`.

## Notes for other worlds

- No `requires` anywhere named an old 01-foundations id. `02-ownership/01-moves` chains to the last
  quest of 01 implicitly, which is now `28-boss-crafting-calculator`.
- `content/tracks/rust/worlds/10-apps/03-serde-models` and `11-boss-oxide-in-rust` contain the string
  `01-foundations/01-variables` as test data only (a fake progress record). It is not read as a quest id.
- `src/lib/mock.ts` imports `00-tutorial/01-hello` and still resolves, but the mock's quest data
  (`answer_is_right`) is from before the rewrite.
- 05-how-big-can-a-number-be has `starter_fails_to_compile = true` on purpose: the two compiler errors are the lesson.
