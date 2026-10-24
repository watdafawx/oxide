# Rust world 11-mastery: rewrite map

World 11 was rewritten in the golden shape (`docs/CONTENT.md`, `content/golden/`), following the audit
`docs/audit-2026-10-24/rust-11-12.md` (it scored 5/10). One idea per quest, an unlabeled intro, kicker cards, one
code block that shows a similar example (not the solution), a short Your turn, two hints (exact code only in the
last), two points with varied answers. Every exercise keeps both flavours in step: plain uses textbook names
(`square!`, `Item`, `Describe`, a text index), games is the Minecraft **chunk indexer** from start to boss. Ledger
rows for the parent to merge: `scratchpad/ledger/rust-11-mastery.md`.

## 12 quests became 25

| old | new |
|---|---|
| 01-macro-rules-basics (two ideas, TODO gave the answer, no "why") | 01-your-first-macro (`$x * $x`, works on `i32` and `f64`; the audit's "why not a function?" card; an expression stays one), 02-pasted-twice-runs-twice (the test shows `next()` ran twice; *side effect* and hygiene glossed; `{{ let v = $x; .. }}`) |
| 02-macro-rules-repetition (5+ ideas, 4 objectives, 12 min) | 03-any-number-of-arguments (repetition, `*` vs `+`), 04-a-trailing-comma (`$(,)?`, starter fails to compile on purpose), 05-a-word-in-the-pattern (literal tokens), 06-paths-that-work-anywhere (`$crate::`, why the leading `::std::`; starter fails with E0422/E0433), 08-a-macro-that-calls-itself (recursion, base case first; `::std::cmp::max` named in the TODO) |
| 05-block-registry (after the proc macros, `requires` 05-attribute-macro) | 07-a-registry-macro, moved right after the macro_rules basics as the audit asked; `requires` dropped |
| 03-read-proc-macros-overview | 09-code-as-tokens (read: token with the `let x = 4 * 2;` picture, token stream, syntax tree, syn/quote), 10-a-crate-for-the-macro (read: the three kinds, why a crate of its own, *workspace* and *target folder* glossed) |
| 04-derive-macro (derive pre-written; learner wrote match + format!) | 11-write-a-derive (the learner writes the whole `quote!` impl; the match is a given helper; starter fails with E0599), 12-generics-in-a-derive (`split_for_impl`, why generics appear twice; starter fails with E0107) |
| 05-attribute-macro (14 min, 3 TODOs spelled out) | 13-wrap-a-function (rebuild `#vis #sig { bump; #block }`, `#(#attrs)*`), 14-a-name-per-function (`format_ident!`; starter fails with E0428 when two functions are tagged) |
| 09-wasm-ready-crate (Fibonacci + one copied attribute; "overflow" label) | 15-only-on-some-targets (**bridge**: *target*, `#[cfg]`, `not()`, what `#[cfg(test)]` means), 16-wasm-ready-crate (`cfg_attr`, wasm-bindgen, glue, cdylib/rlib, target-only dependency; exports a real function: `word_count` / `chunk_index`). Fibonacci cut. The export test still reads the source (no wasm toolchain in the sandbox) but now checks the attribute sits on the function |
| 10-no-std-taste | 17-no-std-taste (house-shape lesson, const generics taught, TODOs in words; same tests) |
| 07-read-profiling | 18-read-profiling (moved after no_std so the performance block runs straight into the boss; adds *linear scan*, *hot spot*, O(n)/O(n²) with the boss's numbers; third quiz question) |
| 06-counting-allocations (two one-word fixes, the allocator never touched) | 19-counting-allocations (the learner writes `allocations_in`, reading the meter; three tests read the numbers 0 / 1 / 6; `thread_local!`, `#[global_allocator]`, `FnOnce` explained), 20-reserve-once (why a Vec reallocates: 4, 8, ... 128, 124 copies; `with_capacity`, `join`) |
| 08-fast-paths (4 objectives, 15 min) | 21-one-lookup-not-three (`entry`), 22-clone-the-winner (references while searching), 23-borrow-dont-copy (`&str`, only `collect` allocates). Same meters (`Counter`, `Name`, allocation counter) and design the audit praised; `squares` moved to 20 |
| (new) | 24-one-buffer-for-every-word (**bridge** before the boss: reuse a `String`, `flat_map(char::to_lowercase)`, `get_mut(&str)`, clone a key only for a new word) |
| 11-boss-optimise-a-slow-crate | 25-boss-optimise-a-slow-crate (slug kept, renumbered to stay last; same code, tests and 8 objectives; lesson in cards, quest references updated, games lesson's `top_words` typo fixed, "Press Run" and "Why now" dropped, point answers varied) |

## Ids and references

- Every quest id changed except the boss slug: `rust/11-mastery/11-boss-optimise-a-slow-crate` → `rust/11-mastery/25-boss-optimise-a-slow-crate`.
- Grepped `content/` (all tracks, gpui included): no `requires` names any 11-mastery quest. `12-broadcast/01` requires `10-apps/11-boss-oxide-in-rust` explicitly, so it is unaffected. `01-your-first-macro` has no `requires`, so it still chains to the last quest of 10-apps.
- Package names are new (`mastery_*`); proc-macro sub-crates have unique package names per quest. Only the proc-macro, wasm and boss quests keep a `Cargo.lock`.

## Notes

- Starters that fail to compile on purpose: 04, 06, 11, 12, 14, 15 (the compiler error is each lesson's first card).
- 13 and 14 put a global atomic in the tests' crate, so 13 has a single objective (one tagged function), and 14 tags three.
