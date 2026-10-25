# Rust world 03-collections: rewrite map

Rewritten in the golden shape (`docs/CONTENT.md`, `content/golden/`): one idea per quest, an unlabeled
intro, kicker cards, one code block, a short Your turn, two hints with the exact code only in the
last one, and points that ask for what the card above just showed. Every quest has a plain flavour
(a shop's stock tool: pens, mugs, lamps, prices in cents) and a games flavour (inventory and loot,
boss: modpack stats), with the same objectives in the same order. Audit:
`docs/audit-2026-10-24/rust-02-04.md`. Ledger rows: `scratchpad/ledger/rust-03-collections.md`
(to merge into `docs/tracks/rust-terms.md`).

13 quests became 35 (33 exercises, 1 read, 1 boss).

## Old → new

| old | new |
|---|---|
| 01-vec-basics (`vec!`, `Vec<T>`, `{:?}`, index, `get`, `copied`, `for &v`, slices) | 01-a-list-of-items, 02-see-whats-inside, 03-follow-the-reference, 04-a-slot-by-number |
| 02-vec-ops (insert, remove, sort, dedup, contains, retain) | 05-move-an-item, 06-tidy-the-pile, 11-keep-what-matches (after closures). `contains` is shown on a `HashSet` in 30 |
| 03-hashmap (`use`, insert/get, entry, zip, `*c`, split_whitespace) | 07-look-up-by-name, 08-count-with-entry. `zip` is a for-the-curious line in 18; `split_whitespace` cut (the input is a slice of names) |
| 06-closures (capture, `impl Fn` param, return with `move`) | 09-a-rule-without-a-name, 10-hand-over-a-rule. Moved **before** every iterator adapter. Returning a closure with `move` is a for-the-curious card in 10 |
| 04-iterators-are-lazy (read) | 13-iterators-are-lazy (read, cards, quiz uses only map/collect) |
| 05-iterator-chains (iter, map, filter, copied, `&&`, collect, turbofish, sum, count) | 12-change-every-item, 14-keep-some, 15-one-number-answers, 16-name-the-type |
| (new) | 17-look-change-or-take: `iter` / `iter_mut` / `into_iter` as lend / lend to change / give away |
| 07-more-adaptors (enumerate, zip, take, skip, rev, chars, windows) | 18-number-the-slots, 19-one-page-at-a-time. `zip` and `windows` are for-the-curious lines in 18; `rev` cut |
| 08-find-and-fold (find, position, any, all) | 20-the-first-match (`position` is a for-the-curious line) |
| 11-fold-and-max (fold, max_by_key) | 21-carry-a-running-value, 22-the-biggest |
| (new, bridge for the boss) | 23-biggest-first (`sort_by`, `Ordering`, `cmp`), 24-decimals-compare-differently (`total_cmp`, `min_by`, why NaN means no `Ord`) |
| 09-strings-and-chars (split, trim, parse, ok, filter_map, chars, join, width, precision, to_vec, `*c`) | 25-cut-text-into-pieces (+ `lines`), 26-keep-what-parses, 27-letter-by-letter (UTF-8 bytes vs chars), 28-glue-it-back, 29-line-up-columns. `to_vec` cut (functions own the list and `sort` it) |
| 10-hashset-and-btreemap | 30-no-repeats, 31-keys-in-order (separate `use` lines, no grouped import) |
| 12-question-mark (`?` + a stray `to_uppercase` paragraph) | 32-pass-the-error-up (self-contained: recaps the `match` it replaces, explains `std::num::ParseIntError`, `?` on Option). `to_uppercase` dropped |
| (new, bridge for the boss) | 33-say-what-went-wrong (`map_err`), 34-all-or-nothing (collect into `Result`) |
| 13-boss-csv-stats | 35-boss-csv-stats (same slug and tests; renumbered so it stays last) |

## Audit items fixed

- Closures are taught (09, 10) before `retain`, `map` and every other adapter. No quest uses `|x|` before 09.
- `impl Fn` comes with the word **trait** (first given in 02 via Display/Debug) and the name **impl Trait** (10).
- `iter` vs `iter_mut` vs `into_iter` has its own quest (17), tied to borrow / mutable borrow / move.
- `usize` (04), `{:?}` and Debug (02), macros and `println!` (01), turbofish and `_` (16), `*` dereference (03), `&&T` (14), `use` and std paths (07, 32) are each explained where first needed. `Self` is not used anywhere in the world.
- No point asks "Which trait…?" without the word being taught; 02's point asks what Rust calls such a skill after the card defines it.
- Glossary lessons (old 02, 03, 05, 07, 09, 10, boss) became single-idea quests; no "Open `src/lib.rs`", "Press Run" or `**Why now:**` labels remain.
- The boss uses only what the world taught: `str::trim` became `|f| f.trim()`, `extend` became a `for` loop with `push`, `map(Row::total)` became a closure, games `top_n` uses `sort_by` with `cmp` instead of `std::cmp::Reverse`. Starter TODOs say the goal in words.
- For the curious (experienced devs): `move` closures (10), `total_cmp` and NaN (24), UTF-8 slicing panics (27), sum overflow and `u64` (15), `max_by_key` ties (22), `sort_by_key`, `zip`, `windows`, `position`, set `union`/`difference`, `{:>w$}`.

## `?` could move to 04-errors

`32-pass-the-error-up`, `33-say-what-went-wrong` and `34-all-or-nothing` are self-contained (they only
assume `Result`, `parse` and `match` from 01, and closures/iterators from this world for 33-34). The
Errors world could open with them instead; the boss here needs all three, so if they move, the 03 boss
should get `parse_line` as a starter helper.

## Ids and references

- Every quest id except the boss changed. No `requires` anywhere in `content/` named a 03-collections
  quest, so nothing needed updating. `01-a-list-of-items` keeps the implicit `requires` (last quest of 02).
- The boss moved from `13-boss-csv-stats` to `35-boss-csv-stats`; `04-errors/01-*` chains to it implicitly.
- Cargo package names are `coll_<slug>`. The old `12-question-mark` used `err_question_mark`; the new
  quest is `coll_pass_the_error_up`, so it cannot clash with a `?` quest in 04.
- `src/lib/mock.ts` names `rust/03-collections/01-inventory` in mock data only (already stale before this rewrite).
