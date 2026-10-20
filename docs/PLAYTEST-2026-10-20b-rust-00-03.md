# Strict "explain, then ask" playtest: Rust worlds 00-03 (2026-10-20, round b)

Scope: `00-tutorial` (3), `01-foundations` (12), `02-ownership` (10), `03-collections` (13) = 38 quests, 4 of them read quests. Method: `docs/PLAYTEST-CHECKLIST.md`, with `docs/CONTENT.md` "Explain, then ask" as the bar.

## Method and honesty notes

- I am an LLM, not a beginner, and I know Rust. For every quest I read lesson, objective labels, starter and tests (games flavour where it exists, then the plain lesson, solution and starter diff), then opened `solution/` and listed each call, keyword, macro, pattern and import that neither the lesson nor an earlier quest of the track taught.
- **I did not copy each quest to `%TEMP%` and run `cargo test` per quest.** The audit is a read-through plus the existing CI (`content-check` compiles and runs every starter/solution). "Solved" below means "the solution's techniques are all reachable from the lesson plus earlier quests (after the fix)", not "I typed and ran it". The temp folder was never created, so there was nothing to delete.
- No hints were used (I never needed to solve blind). Hints were audited afterwards: where a hint was the only place a needed call was named, the item counts as "missing".
- Quizzes (00 has none; read quests are `01/06`, `02/06`, `03/04`; `02/04`, `03/*` boss are exercises): answered from the lesson text. All answers are decided by lesson sentences. Two had the right answer as the longest choice; fixed.
- All 38 quests re-checked in both flavours. `content-check` is green for 00, 01, 02, 03 after the fixes.

## Verdicts

Before fixes: 22 ok, 16 unclear (a needed item first appeared in a TODO or a hint), 0 broken. After fixes all 38 ok.

| quest | verdict (before) | missing before | fixed |
|---|---|---|---|
| 00/01-hello | ok | | |
| 00/02-fix-it | ok | | |
| 00/03-boss-name-tag | unclear | mixing `{}` with a function call argument and `{level}` | lesson sentence + `double(4)` example (both flavours) |
| 01/01-variables | ok | | |
| 01/02-numbers | ok | | |
| 01/03-functions | ok | | |
| 01/04-if-and-loops | unclear | `.to_string()` (every branch returns String), `%`, `!=`, `/=` (plain) | bullets in both lessons |
| 01/05-tuples-and-arrays | ok | | |
| 01/06-how-rust-thinks (read) | unclear | right choice was the longest in both questions | quiz choices reworded |
| 01/07-structs | ok | | |
| 01/08-enums | ok | `use ...PI` given in plain starter; hint names it | none needed |
| 01/09-match | unclear | `A \| B` or-pattern, `.clone()` on `&String`, `String::new()`, `char` literal (plain) | three bullets |
| 01/10-option | ok | `\|` now taught in 09 | |
| 01/11-result | unclear | `parse()` inferring its type from context (no turbofish) | bullet with `let n: u32 = ...` |
| 01/12-boss-guessing-game | unclear | `.iter().find(..)`, `split_once`, early `return Err`/`break` (games); `.map_err` only optional (plain) | toolbox bullets, TODO names `.iter().find` |
| 02/01-moves | ok | | |
| 02/02-borrowing | unclear | `.sum()` only in solution; TODO did not say how to add up | games TODO says `mut` total + `for` loop |
| 02/03-mutable-borrows | unclear | plain TODO said `push('!')` (a `char`), lesson shows `push_str` | plain TODO says `push_str` |
| 02/04-string-vs-str | ok | | |
| 02/05-slices | unclear | empty slice `&[]` for the guard | sentence in both lessons |
| 02/06-borrow-checker (read) | unclear | right quiz choice was the longest (Q2, both flavours) | wrong choices lengthened, still wrong |
| 02/07-clone-and-copy | ok | | |
| 02/08-ownership-in-structs | ok | | |
| 02/09-lifetimes-intro | ok | | |
| 02/10-boss-text-adventure | unclear | `iter_mut`, `position`, `remove`, `is_empty`, `for &line in`; plain also `split_once` and tuple closures | "Toolbox you have not met yet" section in both lessons |
| 03/01-vec-basics | ok | | |
| 03/02-vec-ops | ok | | |
| 03/03-hashmap | unclear | `split_whitespace` named only in TODO; `*count` deref when the slice yields `&u32` | two bullets |
| 03/04-iterators-are-lazy (read) | unclear | right quiz choice was the longest in Q1 | shortened |
| 03/05-iterator-chains | unclear | `&&T` in `filter` closures (needed again in 06) | sentence about `**w` and field auto-deref |
| 03/06-closures | ok | covered by the 05 fix | |
| 03/07-more-adaptors | ok | | |
| 03/08-find-and-fold | ok | quest title says "fold", the quest teaches find/position/any/all (fold is 11) | left, see below |
| 03/09-strings-and-chars | unclear | `to_vec()`, inline `{name:<12}`, `str::contains(char)` and `*c` on `&char` (plain) | three bullets |
| 03/10-hashset-and-btreemap | unclear | `.copied()` after `intersection`; plain `is_alphabetic`, `to_ascii_lowercase` | bullets |
| 03/11-fold-and-max | ok | | |
| 03/12-question-mark | ok | stray `to_uppercase` paragraph is correct but sits oddly | left |
| 03/13-boss-csv-stats | unclear | `lines()`, `min_by`, `extend`; plain also `sort_by` with `total_cmp` | bullets in both lessons |

## Deliberately left

- `03/08` title mentions "fold" though fold is taught in `03/11`. Renaming the quest would break save data keyed on the folder name; the lesson itself is honest. Suggest a title tweak later.
- `03/12` repeats the `to_uppercase` paragraph that `03/09` has; harmless in the games flavour, needed in plain.
- `.sum()` and `.ok_or_else` still appear in a few solutions as the *idiomatic* form; the TODOs and lessons allow the longer `for`/`match` form, so they are not required knowledge.
- Minutes were not changed. Added lesson lines are one-line bullets; the three boss/toolbox additions (`01/12`, `02/10`, `03/13`) add roughly one minute of reading each, within the current 20-40 minute boss estimates.

## Commits

One per world: `Rust world 00 ...`, `01 ...`, `02 ...`, `03 ...` (explicit paths only).
