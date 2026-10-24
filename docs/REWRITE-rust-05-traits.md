# Rust world 05-traits: rewrite map

Rewritten in the golden shape (`docs/CONTENT.md`, `content/golden/`, `docs/REWRITE-rust-00-01.md`) to fix
`docs/audit-2026-10-24/rust-05-07.md`: an unlabeled intro, kicker cards, one idea per quest, one code block
(plus a `text` block where the learner meets a compiler error), a short Your turn, two hints with exact code
only in the last one, and points that ask what the card above showed. Every exercise has a plain flavour
(drawing app: shapes, layers, pens, plus a few textbook types) and a games flavour (the entity system:
mobs, chunks, hitboxes) with the same objectives in the same order. Ledger rows:
`scratchpad/ledger/rust-05-traits.md` (to merge into `docs/tracks/rust-terms.md`).

## 12 quests became 21

| old | new |
|---|---|
| 01-first-trait | 01-first-trait (adds the "why": no classes, no inheritance; mentions `Display` from Errors but works without it) |
| 02-default-methods | 02-default-methods (golden shape, adds the intro "why") |
| 03-generic-functions (3 bounds, `where`, `count_matching`) | 03-one-function-any-type (generic fn + own trait bound, what generics buy you), 04-no-bound-no-greater-than (meet `E0369`, std traits as bounds, `+`, `where`) |
| 04-generic-structs (`Pair<T>`, `swap(self)`, bounded impls) | 05-a-pair-of-anything (`impl<T>`, `Self`, `self` by value), 06-methods-for-some-types (bounded impl blocks; blanket impls for the curious) |
| 05-static-vs-dynamic-dispatch (read, before any `dyn` was written) | 09-static-vs-dynamic-dispatch (moved after 07/08; uses the audit's rewritten dispatch card; dyn compatibility for the curious) |
| 06-trait-objects (4 objectives, 10 min) | 07-a-list-of-mixed-shapes (`Box<dyn Shape>` from the `E0308` mixed-`vec!` error; heap and "slots of one size" explained), 08-borrow-a-trait-object (`&dyn Shape`, `as_ref` for boxed items) |
| 07-std-traits (derive, Display, From: 6 ideas, 4 objectives) | 10-a-sensible-default (`Default`), 11-equal-and-hashable (`Eq` + `Hash`, NaN, set/map keys) |
| 08-operator-overloading (4 objectives; inconsistent `PartialOrd`) | 16-make-plus-work (`Add`, `Neg`, `self` by value + `Copy`), 17-times-a-number (`Mul<f64>`, `f64 * Vec2`, generic `Rhs` vs associated `Output`; orphan rule for the curious), 18-which-is-closer (`PartialOrd` that agrees with `==` via a `Length(f64)` key, `Option<Ordering>`, NaN) |
| 09-iterator-trait (12 min) | 14-build-your-own-iterator (`next`, `type Item`, why `&mut self`), 15-adaptors-for-free (adaptors are default methods; `IntoIterator` for the curious) |
| 10-associated-types-and-consts (4 objectives, taught after it was used) | 12-a-type-inside-a-trait (why not `Convert<Output>`; bound on `C::Output` now shown), 13-a-constant-inside-a-trait. Both now come before Iterator (`Item`) and `Add` (`Output`). |
| 11-impl-trait (4 objectives) | 19-impl-trait (names the idea behind `impl Fn`; the one-concrete-type `E0308` error shown) |
| (new bridge) | 20-sort-by-a-float (`Ord` vs `PartialOrd`, `total_cmp`, `max_by`, `sort_by`): the boss's "Tools:" bullet as a quest |
| 12-boss-shape-renderer | 21-boss-shape-renderer (same slug and 8 objectives; formulas moved into starter doc comments, hints give structure only, "Tools:" bullet removed) |

## Ids and references

- Boss renumbered `12-boss-shape-renderer` -> `21-boss-shape-renderer` (slug kept). Updated
  `content/tracks/gpui/worlds/00-setup/01-what-gpui-is/quest.toml` `requires`. Still naming the old id in prose
  (not quest ids, not edited, outside this world): `docs/tracks/gpui-terms.md` line 5 and `docs/tracks/gpui.md` line 121.
- No other `requires` anywhere named a 05-traits quest. `01-first-trait` keeps no `requires`, so it still chains to
  the last quest of 04-errors.
- Cargo package names are now `traits_*` (the boss keeps `trait_boss_shape_renderer`, which its `main.rs` imports).

## Cut on purpose

- `count_matching` (`PartialEq` bound with a reference-of-reference closure): the `==` bound idea is covered by 11.
- Derive `Debug`/`Clone`/`PartialEq`, `Display` with `write!`, `From`/`.into()` from old 07: taught in
  02-ownership/07 and 04-errors/01-02; re-teaching them was the audit's "low value" finding.
- `adder` / `compose` closures from old 11: taught in 03-collections/06-closures.
- The 2D `ChunksInRadius` walk (games 09): replaced by a one-row `ChunkRow` so the quest is about `next`, not the walk.
  `rem_euclid` is named in a TODO in 15 (games) instead of a lesson bullet.
- The old 08 objective that taught a `PartialOrd` disagreeing with `==`: 18 now teaches why they must agree.

## Notes

- 13 and 15 have three cards each (short, easy-win quests); everything else has four to six.
- `08-borrow-a-trait-object`: `&Box<dyn Shape>` does not coerce to `&dyn Shape` in a function argument, so the
  lesson and tests use `.as_ref()`.
