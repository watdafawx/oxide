# Rust world 02-ownership: rewrite map

Rewritten in the golden shape (`docs/CONTENT.md`, `content/golden/`), following
`docs/audit-2026-10-24/rust-02-04.md`: one idea per quest, an unlabeled intro, kicker cards, one code
block that shows a similar example (never the solution), a short Your turn, two hints with the exact
code only in the last, and points that ask what the card just showed. Every exercise keeps a plain
twin (textbook objects: notes, backpacks, labels, jars, a text adventure) and a games flavour (the
server console), with the same objectives in the same order. Ledger rows:
`ledger/rust-02-ownership.md` in the session scratchpad (for the parent to merge into
`docs/tracks/rust-terms.md`).

## 10 quests became 21

| old | new |
|---|---|
| (none: the audit's "memory model" gap) | 01-where-values-live (read: memory, address, stack, heap, pointer, free, leak, double free, garbage collector vs one owner; links Programming 101's "Numbered boxes") |
| (none: scope and drop were never named) | 02-gone-at-the-brace (block, scope, dropping; `E0425`) |
| 01-moves (move + clone + GC in four sentences) | 03-hand-it-over (the audit's rewrite: numbers copy, text moves, one owner, `E0382`, `String::from`, `usize`), 04-a-second-copy (the old `length_and_name` exercise: clone) |
| 07-clone-and-copy (late, "trait" and `#[derive]` unexplained) | 05-copy-your-own-type, moved up to right after moves: attribute, `derive`, `Debug`/`PartialEq` (what the starters' derive lines do), `Clone`, `Copy`, the word trait |
| 02-borrowing | 06-lend-it (reference, borrowing, ties to `&self`; macro and `vec!` glossed; the solution no longer uses a closure) |
| 03-mutable-borrows (3 ideas) | 07-lend-to-change (`&mut`), 08-one-writer-at-a-time (the rule with its why: `push` can move the heap block; `E0502`; `move_last` moved here) |
| (none: `*` was never taught) | 09-follow-the-arrow (`*`, auto-deref through `.`, `&&T`/`**`) |
| (none: the boss used `iter_mut`) | 10-change-every-item (`for x in &mut list`; the three loop forms) |
| 06-the-borrow-checker-is-your-friend (read) | 11-the-borrow-checker-is-your-friend (same slug, kickers, `println!` replaced by `format!`, third quiz question) |
| 04-string-vs-str (3 ideas, "heap" unexplained) | 12-owned-or-borrowed-text (`&str` vs `String`, why `&String` converts, `'static` for the curious), 13-match-on-text (`.as_str()`) |
| (none) | 14-new-text-is-owned (dangling reference, `E0515`: return the `String`) |
| 05-slices (text + lists in one) | 15-part-of-the-text (string slices; UTF-8 byte boundaries for the curious), 16-part-of-a-list (`&[T]`, `&[]`). The games `trim_edges<'a>` signature, used before lifetimes were taught, is now elided `&[String]` |
| (none: the boss toolbox listed `position`/`remove`) | 17-take-it-out (`E0507`, `Vec::remove` hands over ownership, index loop) |
| 08-ownership-in-structs (lifetime pre-written) | 18-structs-own-their-fields (`&str` field fails with `E0106`; choose `String`), 19-a-struct-that-borrows (the learner writes `<'a>` on the struct; `'_`) |
| 09-lifetimes-intro (lifetime pre-written, `let result;` and generics `T` used untaught) | 20-which-input-is-it (starter fails with `E0106`; the learner writes `<'a>` and the three `&'a`; elision rules for the curious) |
| 10-boss-text-adventure | 21-boss-text-adventure (same slug, renumbered so it stays last) |

## The bosses

The "Toolbox you have not met yet" sections are gone. Both bosses now use only this world's tools:
index loops with `remove` (17), `for x in &mut list` (10), `*line` on a `&&str` (09), `Result` and
`match` from world 01. The lesson cards teach the three small helpers it still needs: `join`,
`!` on a `bool` and `split_once`. Closures, `iter_mut`, `position`, `find`, `?`, `ok_or_else`, slice
patterns, `as_slice` and the turbofish were removed from both solutions and from the starter helpers
(which are now plain `for` loops). The games boss gets a written `parse_pos` helper for the three
coordinates instead of slice patterns. Objectives, test names and tests are unchanged.
"Press Run" and "Open `src/lib.rs`" are gone from every lesson.

## Ids and references

- Every quest id except `11-the-borrow-checker-is-your-friend` (renumbered from 06) changed. The
  boss slug is kept and renumbered: `02-ownership/10-boss-text-adventure` -> `02-ownership/21-boss-text-adventure`.
- No `requires` anywhere in `content/` named a 02-ownership quest. `03-collections`' first quest
  chains implicitly to the last quest of 02, which is now `21-boss-text-adventure`. `01-where-values-live`
  chains implicitly to `01-foundations/28-boss-crafting-calculator`, as `01-moves` did.
- `src/lib/mock.ts` uses fake 02-ownership ids (`01-moves`, `02-borrowing`, `03-boss-stack`) that did
  not match the content before either.
- Other worlds' lessons that mention "world 02" (06-smart-pointers 05, 06, 08) still hold: the
  move error, borrowing structs and the one-writer rule are all still taught here.
- Cargo package names are new (`own_<slug>`); the boss keeps `own_text_adventure` (its `main.rs` uses it).

## Not done here (belongs to other worlds)

- Closures, `iter`/`iter_mut`/`into_iter` and `&&T` in iterator chains are for 03-collections. 09
  names `&&T` and promises it there.
- Traits are only named (05 and the derive lines); world 05 teaches them.
