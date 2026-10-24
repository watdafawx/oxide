# Java 03-collections: golden-shape rewrite

World rewritten to `docs/CONTENT.md` (one idea per quest, kicker cards, one code block, no
"Open `src/...`" step, no `**Why it matters:**` label). 11 quests became 25. Every exercise has a
plain and a games flavour with the same objective count and order. Games spine: the **modpack
organiser** (`project = "Modpack organiser"` in `world.toml`), built on a `Mod` record (id, name,
loader, downloads, version). Plain uses everyday data (shopping list, raffle, scores, café till,
quiz night) and ends on the log analyzer. Ledger rows: see the term ledger for 03-collections.

Starters now use `return null;` / a wrong value with a `// TODO:` (as in 01-basics), not the
`throw new UnsupportedOperationException("TODO")` idiom.

## Old → new

| old | new |
|---|---|
| 01-lists | 01-a-list-that-grows (`ArrayList`, generics and why, diamond), 02-ask-for-a-list (`List` interface on the left/as parameter; starter fails to compile on purpose), 03-numbers-need-a-box (primitives, `Integer`, boxing and why `List<int>` fails), 04-remove-by-slot-or-value (`remove(int)` vs `remove(Object)`) |
| 09-immutable-collections | 05-lists-that-cannot-change (moved forward: it explains why the starters copy `List.of` lists) |
| 02-sets | 06-sets-hold-each-once (`split`/`Arrays.asList` dropped; `add` returns boolean added) |
| 03-maps | 07-maps-key-to-value (`put`/`get`/`getOrDefault`), 08-walk-a-map (`entrySet`, `Map.Entry`), 12-count-with-merge (`merge`, `Integer::sum`), 13-group-into-lists (`computeIfAbsent`); both after lambdas now. `charAt`/`Character` keys dropped (groups by length / loader instead) |
| 04-order-and-hashing (read) | 09-who-keeps-the-order (read; "You will write your own equals/hashCode in the OOP world" replaced, since OOP now comes before) |
| 05-lambdas-and-method-refs | 10-a-rule-as-a-value (lambda, `Predicate`, `removeIf`, effectively final with the reason), 11-method-references (`Function`, `Type::method`). `Runnable` dropped |
| 06-sorting | 14-sort-by-a-key (`comparing`, `comparingInt`, `reversed`), 15-break-ties (`thenComparing`, where `reversed` goes) |
| 07-stream-basics | 16-your-first-stream (why: loop + temp list vs pipeline; `filter`, `toList`, laziness), 17-change-each-item (`map`, `limit`), 18-numbers-from-a-stream (`mapToInt`, `sum`, `count`) |
| 08-collectors | 19-group-a-stream (`groupingBy`, `counting`, static import), 20-join-into-text (`joining`), 21-split-in-two (`partitioningBy`) |
| 10-flatmap-reduce | 22-flatten-lists (`flatMap`), 23-maybe-there-is-none (`max` → `Optional`, `map`, `orElse`). `reduce` and `IntStream.rangeClosed` became a "for the curious" line in 18 |
| (new bridge) | 24-cut-a-line (`split`, `split` with a limit, `List.of(array)`) for the boss |
| 11-boss-log-analyzer | 25-boss-log-analyzer (same slug, renumbered to stay last; tests unchanged) |

## Boss changes

The boss now uses only what the world taught:
- `Map.Entry.comparingByValue()` + stream `sorted` (inference trap) replaced by a starter record
  `SourceCount(source, count)`: loop over `entrySet`, sort the list with
  `comparing(...).reversed().thenComparing(...)`, then `stream().limit(n).map(...)`.
- `groupingBy(f, TreeMap::new, counting())` (constructor reference) replaced by
  `new TreeMap<>(... groupingBy(f, counting()))`.
- `substring(0, 5)` (plain) and `Integer.parseInt` (games) are named in the TODO and the lesson,
  not taught as a quest (parseInt is also glossed in 11-method-references).
- Games: `topDownloaded` sorts a copy with `List.sort` instead of stream `sorted`.

## ids and requires

All quest folders except the boss slug were renamed. Nothing outside this world requires a quest
in it (grep of `content/` for `03-collections` in `requires`), and there is no java world 04 to
chain to the boss implicitly. 01-a-list-that-grows keeps `requires` on the 02-oop boss
(now `02-oop/30-boss-bank-ledger`, the folder the 02-oop rewrite renumbered it to).

## Assumptions about 02-oop

Written against the 02-oop rewrite in progress: interfaces ("a promise of methods"), records and
accessors, `equals`/`hashCode`, `static`, `new`, and exceptions (`throw`, `IllegalArgumentException`,
`UnsupportedOperationException`) are taught there. 02-ask-for-a-list says "like the interfaces in
the OOP world"; 09 says a class of your own used as a key must define `equals` and `hashCode`.

## Validation

`content-check --fast` green. Gradle cannot download JUnit/AssertJ in this sandbox, so starters and
solutions were compiled with `javac` against cached JUnit API and AssertJ jars and their `@Test`
methods run by reflection: every solution passes all its tests, every starter fails each objective
test (02-ask-for-a-list starters fail to compile, as declared).
