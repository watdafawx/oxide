# Java 02-oop: golden-shape rewrite

12 quests became 30 (2 reads, 27 exercises, the boss), one idea each, in the card shape of
`docs/CONTENT.md`: unlabeled intro, kicker cards, one code block that shows a similar example,
a short Your turn, two hints (exact code only in the last), points that ask what the card above
showed. Every quest has a plain and a games flavour with the same objectives in the same order.
`world.toml` now names the project (storage chest with an audit log; plain story: a bank ledger).
Term ledger rows: `/tmp/claude-0/-home-user-oxide/b3e0b197-0d34-5418-b363-ad5eaa19cc19/scratchpad/ledger/java-02-oop.md` (the parent merges them) (for `docs/tracks/java-terms.md`).

## Old quest → new quests

| old | new |
|---|---|
| (new) | 01-methods-and-classes (read: what a method, a call and a class are, the words used since 00-setup/02) |
| 01-classes-and-fields | 02-an-object-remembers (field, class vs object, `new`, a method without `static`, the TODO `throw` line), 03-private-means-mine (`private`, `public`, check-then-change) |
| 02-constructors-and-this | 04-what-new-runs (constructor, the free default one), 05-this-is-the-field (`owner = owner;` trap, `this.`), 06-set-once (`final`), 07-many-ways-to-build (overloading, `this(...)`) |
| 04-static-members | 08-shared-by-every-object (static field; non-static first gives every ticket id 1), 09-no-object-needed (static method, factory, private constructor, why 00-01 was `public static`, why `main` is static) |
| 03-encapsulation (read) | 10-dont-hand-out-the-array (read; leak through a returned `String[]`, fixed with `clone()`; `List.copyOf` is a "for the curious" pointer to world 03) |
| 05-interfaces | 11-a-shared-promise (interface, `implements`, `@Override`), 12-a-default-body (`default`) |
| 06-inheritance-and-super | 13-is-a (`extends`, overriding, parent code runs the child's version), 14-the-parents-version (`super(...)`, `super.describe()`) |
| 07-polymorphism | 15-a-half-finished-parent (`abstract`), 16-one-loop-every-kind (array of objects, polymorphic loop) |
| 08-equals-hashcode-tostring | 17-equal-by-value (`Object`, identity vs value, `instanceof` pattern), 18-hash-matches-equals (contract, one `HashSet` card), 19-readable-text (`toString`, `%02d`) |
| 09-enums | 20-a-fixed-set (constants, `==`, switch over `this`), 21-constants-with-data (fields, constructor, `values()`) |
| 10-records | 22-data-in-one-line (record), 23-reject-bad-values (compact constructor, `throw`, `IllegalArgumentException`), 24-change-means-copy (with-method) |
| 11-sealed-and-pattern-switch | 25-the-compiler-counts (sealed + type patterns, the "does not cover" error), 26-a-case-with-a-condition (`when`, "dominated" error), 27-unpack-in-the-case (record patterns; games: nested) |
| (bridge) | 28-a-switch-that-does (switch statement, block arms, `-=`): the boss's `apply` needs it |
| (bridge) | 29-check-then-change (own exception `extends RuntimeException`, throw before any change, do the step that can fail first): the boss's "rejected changes nothing" rule |
| 12-boss-bank-ledger | 30-boss-bank-ledger (same slug; games: chest ledger) |

## Used-before-taught fixes

- `List`, `ArrayList`, generics, `List.copyOf`, `HashMap`, `merge`, `Integer::sum`, `String.join`: gone
  from every lesson and starter. 16 uses `Employee[]` / `Mob[]`; the boss keeps its history in an
  array through a written private helper `record(...)` (`Arrays.copyOf`) and returns `clone()`; the
  games chest counts items in two parallel arrays through a written helper `change(item, delta)`;
  statements are built with `StringBuilder` (01-basics/22). Tests still use `HashSet`/lambdas; the
  learner never writes them.
- `HashSet` is explained once, at point of use, in 18 (buckets by hash, then equals; "world 03 teaches it").
- `protected` dropped (abstract parents have a public constructor). `Locale.ROOT` and the enum
  `parse`/`valueOf` objective dropped; `valueOf`/`ordinal` are a "for the curious" card in 21.
- `throw new UnsupportedOperationException("TODO: ...")` is explained in 02 ("replace the whole line");
  `throw` and exceptions themselves in 23, own exception classes in 29.
- Braces everywhere (the old 03/08 one-line `if`s are gone). No "Open `src/...`" steps.

## Boss changes

- Objective order: `transactionIsSealed` (games `eventIsSealedToThreeKinds`) moved to first, because
  a switch with no `default` only compiles once the interface is sealed (taught in 25). Five objectives, 60 min.
- `history()` / `log()` return arrays now (`Transaction[]`, `Event[]`); tests adjusted (`log()[0]`).
- Lesson has no "New tools" block: everything it needs is taught in 02-29 or written in the starter.

## Ids and references

- Every old quest id except the boss is gone. Boss renumbered `12-boss-bank-ledger` → `30-boss-bank-ledger`.
- Updated `content/tracks/java/worlds/03-collections/01-a-list-that-grows/quest.toml`:
  `requires = ["02-oop/30-boss-bank-ledger"]`. No other `requires` in `content/` named a 02-oop quest.
- `01-methods-and-classes` has no `requires`, so it still chains to the 01-basics boss.
- `docs/tracks/java-25.md` (a plan, not content) still names old ids such as `java/02-oop/06-inheritance-and-super`
  and `11-sealed-and-pattern-switch` as "builds on"; the nearest new ones are 14 and 25-27.

## Validation

`content-check --fast` is green. Gradle cannot fetch the JUnit engine here, so every starter and
solution (both flavours) was compiled with `javac` against the cached junit-api/assertj jars and its
tests run with a small reflection runner: every starter compiles and fails all its objectives, every
solution passes all of them.
