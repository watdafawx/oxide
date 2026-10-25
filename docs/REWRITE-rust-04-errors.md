# Rust world 04-errors: rewrite map

The world was rewritten in the golden shape (`docs/CONTENT.md`, `content/golden/`), following the
audit `docs/audit-2026-10-24/rust-02-04.md` (it scored 4/10). One idea per quest, an unlabeled
intro, kicker cards, one code block that shows a similar example (not the solution), a short Your
turn, two hints, two points. Every exercise keeps both flavours in step: the plain flavour is a
**todo list that remembers** from start to finish (it used to change topic from quest to quest), and
the games flavour is the **modpack manifest loader** (TRACKS.md). Ledger rows for the parent to merge:
`scratchpad/ledger/rust-04-errors.md`.

## 10 quests became 19

| old | new |
|---|---|
| (new) | 01-errors-are-values (read): the why of Rust's model. No exceptions, errors are values in the return type, panic previewed |
| 01-custom-error-enum (8 ideas, 12 min, 3 objectives) | 02-a-reason-per-failure (an error enum), 03-a-promise-of-methods (**bridge: traits**), 04-teach-it-to-print (`Display`) |
| 03-panic-vs-result (read) | 05-panic-or-result (read, in cards). Moved after `Display` so `?` follows it straight away |
| 02-from-and-into | 06-convert-with-from (`From`, `Self`, `<T>` explained where it appears, one-card `?` refresher, `Into` as "for the curious") |
| 04-box-dyn-error | 07-one-box-for-any-error (why it must be boxed: unknown size; heap, pointer and `dyn` glossed; `main -> Result` and `Send + Sync` for the curious) |
| (part of 01: `impl Error`) | 08-your-error-fits-the-box (starter fails to compile with E0277; empty impl, default methods, `source()` for the curious) |
| 09-cargo-and-crates (read) | 09-crates-and-cargo (read). Moved **before** thiserror/anyhow, the first quests that use a crate. Adds what a crate is |
| 05-thiserror | 10-thiserror (attributes explained; the off-topic `find` closure lookup was cut) |
| 06-anyhow (4 objectives) | 11-anyhow-and-bail (`anyhow::Result`, `bail!`, what `!` macros are), 12-add-context (`context` vs lazy `with_context`, root cause, `{:#}`) |
| 07-modules (4 objectives) | 13-a-room-for-code (`mod` + `pub`, starter fails with E0603), 14-paths-between-rooms (`super`, `self`, `crate`, `use`) |
| 08-files-in-modules | 15-one-file-per-module (the learner now writes the `mod` lines; starter fails with E0433 and its help line) |
| (new) | 16-read-a-file (**bridge**: `Path`, `fs::read_to_string`, `io::Error`, `display()`, `lines()`) |
| (new) | 17-a-missing-file-is-fine (**bridge**: `io::ErrorKind::NotFound`, match guards) |
| (new) | 18-write-it-back (**bridge**: `fs::write`, `()`, building the text) |
| 10-boss-list-manager | 19-boss-list-manager (same slug, renumbered so it stays last; same 8 objectives and test names) |

## Fixes the audit asked for

- **Traits before they are used.** 03 teaches "a trait is a promised set of methods; `impl Trait for Type`
  keeps the promise", and why (other code can ask for the promise, checked at compile time), plus what
  `#[derive(Debug, PartialEq)]` does. 04, 06, 08 and 10 then keep `Display`, `From` and `Error`. No quiz or
  point uses "trait" before 03.
- **`Box<dyn Error>`, `dyn`, `<T>`, `Self`, heap** are explained where they first appear (06, 07).
- **Old 01's third objective passed with no work.** Gone: every objective now needs its own code
  (04's second objective is `explain`, which uses the error inside a `format!`).
- **The boss used ~9 untaught tools.** File I/O now has three bridge quests (16-18). The rest was cut from the
  boss or taught in its cards: `#[derive(Default)]`/`Self::default()` → `Store { todos: Vec::new() }`;
  `iter().map().max()` → a `for` loop; `iter_mut().find()` → `for todo in &mut self.todos` (boss card);
  `rest.parse().map(Command::Done)` → a `match`; `split_once(..).unwrap_or(..)` → given `split_verb` helper;
  `let … else` removed from the given `parse_line`; `enumerate` and `!` get a boss card each; `join` is named in the TODO.
- Housekeeping: no "Open `src/lib.rs`", "Press Run" or `**Why now:**` labels; `todo_cli` in the Cargo example
  is now `todo_list`; every exercise is 3-7 minutes with 1-3 objectives; quiz answers vary.

## Ids and references

- Every quest id in this world changed except the boss slug. The boss moved from
  `rust/04-errors/10-boss-list-manager` to `rust/04-errors/19-boss-list-manager` (renumbered so it stays the
  last folder, as world 01 did; approved by the coordinator).
- I grepped `content/` (all tracks) for `requires` naming any `04-errors` quest: there are none. `01-errors-are-values`
  has no `requires`, so it still chains to the last quest of 03-collections; `05-traits/01-first-trait` chains to the new boss.
- `src/lib/mock.ts` names `rust/04-errors/01-load-manifest`, a fake id in mock data that did not exist before either.
- Cargo package names are new (`err_reason_per_failure`, ...); the boss keeps `err_boss_todo` / `err_boss_modlist`.

## Notes for other worlds

- This world assumes 03-collections teaches `?`, closures (`|| ...`), `for x in &list`, `Vec<T>` and `&[T]` slices.
  06 gives a one-card `?` refresher; 12 calls a closure "the nameless function from the collections world".
- 05-traits/01 can now say "you met traits in the errors world" and build on `impl Trait for Type`, default methods
  (04/08) and `dyn` (04/07).
