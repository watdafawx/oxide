# Rust world 06: rewrite map

World 06 was rewritten in the golden shape (`docs/CONTENT.md`, `content/golden/`). That means one idea per quest, an unlabeled intro, kicker cards, one code block that shows a similar example rather than the solution, a short Your turn, two hints and two points. Every exercise has a plain flavour (an outliner: plans, notes, tags) and a games flavour (the Factorio machine graph: belts, inserters, assemblers, power poles, tech tree). Both flavours have the same objectives in the same order. Audit: `docs/audit-2026-10-24/rust-05-07.md`. Ledger rows: the scratchpad `ledger/rust-06-smart-pointers.md`, which the parent merges.

Each pointer quest now opens with the problem it solves:

- `Box`: a recursive type has infinite size.
- `Rc`: two owners give `E0382`.
- `RefCell`: why `&` normally cannot mutate. A `Vec` that reallocates under a reader leaves the reader pointing at freed memory.
- `Weak`: a cycle of `Rc`s never reaches 0, which is a leak.

## 11 quests became 17

| old | new |
|---|---|
| 01-box (Box + recursion + `use List::{..}`, 3 objectives) | 01-box (Box, starter fails with E0072 on purpose. It has a one-card stack/heap/drop refresher, and "pointer", "compile time" and "infinite size" are explained. Based on the audit's rewrite), 02-walk-the-chain (recursion, base case) |
| 02-deref-and-drop (no Drop in it) | 03-deref (title "Act like what you hold": why a `Box` acts like its value, plus deref coercion as the reason `&String` works as `&str`) |
| 10-drop-and-cleanup (after Arc, `&'static str`) | 04-drop (Drop at the brace, scope, reverse order), 05-drop-early (`drop(x)`, E0499). Drop now comes right after Box/Deref, and `'static` is gone (the name is a `String`) |
| 08-cow | 06-cow (moved up next to Deref, since `Cow` derefs to `str`. Allocation is explained) |
| 03-rc | 07-rc (opens with E0382. `ptr_eq` moved to 16) |
| 04-interior-mutability (read) | 08-interior-mutability (read: why the rule exists, runtime vs compile-time check, `already borrowed` panic, `Cell` as a for-the-curious card. 3 quiz questions) |
| 05-refcell (3 objectives, 9 min) | 09-refcell (`borrow_mut`, `*`, 2 objectives), 10-busy-or-free (`try_borrow_mut().is_err()`, 1 objective) |
| 06-rc-refcell-graph (4 objectives, 12 min) | 11-shared-and-changeable (`Rc<RefCell>`, type alias), 12-total-of-a-tree (recursion through borrows, uses a `for` loop instead of `.map(total).sum()`) |
| 07-weak (12 min, `*cell.borrow_mut() =`, "leak" never explained) | 13-weak (cycle, what a leak is and what it costs, downgrade/upgrade), 14-pointing-back-up (Weak parent set when the node is built, so `*cell.borrow_mut() = ..` is not needed) |
| 09-arc-preview | **cut**. Threads were never explained, and world 07 teaches `Arc` again. 07-rc says in one sentence that `Rc` is single-threaded and that world 07 has `Arc` |
| 11-boss-tree-editor | 17-boss-tree-editor (slug kept, renumbered to stay last) |

New bridge quests before the boss: **15-climb-to-the-root** (`while let`, walking up `Weak` parents, `.reverse()`, one borrow per statement) and **16-remove-that-exact-one** (`Rc::ptr_eq` vs `==`, `retain`, taking the parent in its own `let`).

Boss lesson changes:

- "Three things the boss needs" and the "More small tools" glossary are gone. Each `// TODO:` now names the helper calls that method needs (`.cloned()`, `repeat`, `push_str`, `HashMap::retain`, `any`, `map[&key]`).
- Hints no longer use `let ... else` or `Option::map`, because neither is taught in the track. They use `match` and `?` instead.
- A for-the-curious card on arenas (ids instead of `Rc<RefCell>`, `slotmap`/`petgraph`) says that `Doc`'s id map is halfway there.
- The earlier `'\n'` and "pole and machine" fixes are kept: the `'\n'` escape is in the rules card, and no games text appears in plain.

## Id changes and references

- `rust/06-smart-pointers/11-boss-tree-editor` → `17-boss-tree-editor`. Updated `requires` in `content/tracks/gpui/worlds/03-live/01-state-in-a-view/quest.toml`. **Not updated:** `docs/tracks/gpui.md` line 122 still names the old id. It is a doc table outside this world, so the parent should fix it.
- No other `requires` anywhere named a 06 quest. `01-box` keeps no `requires`, so it still chains to the last quest of 05. The old `10-drop-and-cleanup`'s `requires = ["06-smart-pointers/09-arc-preview"]` is gone.
- `src/lib/mock.ts` has its own fake `rust/06-smart-pointers` quest data and does not read these ids.

## Notes

- `01-box` has `starter_fails_to_compile = true` on purpose: E0072 is the lesson.
- Solutions in 13/14 use `match upgrade() { Some(x) => Some(..), None => None }` instead of `.map`, because `Option::map` is untaught. Clippy would suggest `manual_map` here, which is fine.
