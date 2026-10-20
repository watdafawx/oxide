# Blind playtest, strict pass: Java 00-setup, 01-basics, 02-oop, 03-collections (2026-10-20)

Third pass after [PLAYTEST-2026-10-20.md](PLAYTEST-2026-10-20.md) and [PLAYTEST-2026-10-20-java-lua.md](PLAYTEST-2026-10-20-java-lua.md). Method: `docs/PLAYTEST-CHECKLIST.md`. Question for every quest: could a learner solve it from the lesson, the earlier Java quests, the labels, the starter and its tests alone?

## Method and honesty notes

- Scope: 42 quests (8 + 11 + 12 + 11), both flavours each (84 lesson/solution pairs). 6 are read quests (quiz only).
- I am an LLM and know Java. I did **not** re-run `gradlew test` per quest from a copy in `%TEMP%\oxide-playtest`. For each quest I read lesson, labels, starter and tests as a learner sees them, decided how I would solve it, then opened `solution/` and listed every call, keyword, syntax and import that neither the lesson nor an earlier quest taught. Starters and solutions are compiled and tested by `content-check` (below), which is how the "starter compiles / solution green" half was verified. Last round's real Gradle runs of 02 and 03 (games) are the evidence for solvability there; nothing in this pass changed any test.
- To catch compile traps mechanically I diffed the `import` lines of every starter against its solution (all 84): the only two mismatches were the two fixed below.
- For every read quest I checked each answer against a sentence in the lesson and that the right choice is not the longest. all 6 failed the "not longest" rule on at least one question and were fixed.
- No hints were opened (nothing solved by running). "Hint audit" = I read each hint after the solution and checked it names the thing.

## Verdicts

| | ok | unclear (fixed) | broken |
|---|---|---|---|
| 00-setup (8) | 3 | 5 | 0 |
| 01-basics (11) | 7 | 4 | 0 |
| 02-oop (12) | 6 | 6 | 0 |
| 03-collections (11) | 8 | 3 | 0 |

"unclear" = something in the solution or the quiz that the lesson never showed. All fixed in place (list below) except where marked "left".

## Per quest

Columns: used-before-taught / quiz problems found, then what was fixed. "both" = games and plain flavour. Verdict is the state before the fix.

### 00-setup

| quest | verdict | missing / problem | fix |
|---|---|---|---|
| 01-whats-a-jdk (read) | unclear | quiz Q3 right answer longest (both) | choices reworded |
| 02-hello-oxide | ok | none | |
| 03-reading-red (read) | unclear | Q2 and Q3 right answer longest (both) | choices reworded |
| 04-compile-error | ok | none | |
| 05-project-layout (read) | unclear | Q3 right answer longest (both) | choices reworded |
| 06-main-and-args | unclear | solution uses `args.length == 0`; `==` never shown | lesson bullet on `==` (both) |
| 07-printing | ok | none | |
| 08-boss (status report / tip) | unclear | `%%` only in the last hint; plain: `double` vs `int` and `100.0` vs `100` only in a hint | `%%` line (both); double/int line (plain) |

### 01-basics

| quest | verdict | missing / problem | fix |
|---|---|---|---|
| 01-primitives | ok | none | |
| 02-integer-division | ok | none | |
| 03-how-java-runs (read) | unclear | Q2, Q3 right answer longest (both) | choices reworded |
| 04-if-and-ternary | ok | plain lesson's short-circuit example called `.length` on a list | example now uses `text.length()` |
| 05-switch-expressions | ok | none | |
| 06-loops | unclear | games: `c == '§'` char compare never shown; plain: `"aeiou".indexOf(...)` never taught, TODO silent | char-compare bullet (games); `indexOf` bullet + TODO names it (plain) |
| 07-methods-and-overloads | ok | none | |
| 08-strings | unclear | `a.equals(b)` call syntax never shown (only "use equals") | `equals` line in the example (both) |
| 09-text-blocks-and-format | ok | none | |
| 10-arrays | ok | none | |
| 11-boss-wordle | unclear | `char[]`, `Arrays.fill`, `new String(char[])`, `import java.util.Arrays` all unshown; **starter had no import**, so the natural solution failed to compile | "New tools" block in both lessons; `import java.util.Arrays;` added to both starters |

### 02-oop

| quest | verdict | missing / problem | fix |
|---|---|---|---|
| 01-classes-and-fields | ok | none | |
| 02-constructors-and-this | unclear | `Math.min` never shown (lesson showed only `Math.max`); other constructors in the starter assign fields before the TODO, which is a compile error once `this(...)` follows | `Math.max/min` bullet (both); TODOs say to delete the lines below (both) |
| 03-encapsulation (read) | unclear | Q1 and Q2 right answer longest (both), plain Q2 too | choices reworded |
| 04-static-members | unclear | solution `nextId++` (postfix) but the lesson shows only `++fired` | postfix vs prefix bullet (both) |
| 05-interfaces | unclear | **plain:** `Math.round` untaught, every TODO was a bare `"TODO"`, the names `"Circle"` and `"Rectangle"` only in hints. Games was fixed last round | `Math.round` bullet; TODOs name the text/formula (plain) |
| 06-inheritance-and-super | ok | none | |
| 07-polymorphism | ok | none | |
| 08-equals-hashcode-tostring | ok | `Objects` import was fixed last round and is still there | |
| 09-enums | unclear | `toUpperCase(Locale.ROOT)` in the hint/solution but **no `import java.util.Locale;` in the starter**; `switch (this)` unshown | import added to both starters; two lesson bullets (both) |
| 10-records | ok | none | |
| 11-sealed-and-pattern-switch | ok | none | |
| 12-boss-bank-ledger | unclear | `Map.merge(..., Integer::sum)`, method refs, `List<String>.add`, `String.join` all unshown (Maps arrive in world 03). Plain: `String.join`, `-=` | "New tools" block (both) |

### 03-collections

| quest | verdict | missing / problem | fix |
|---|---|---|---|
| 01-lists | ok | none | |
| 02-sets | ok | none | |
| 03-maps | unclear (nit) | lambda `k -> new ArrayList<>()` and `Integer::sum` appear two quests before lambdas are taught | one bullet saying what they are, "copy the shape" (both) |
| 04-order-and-hashing (read) | unclear | Q1 right answer longest (both) | choices reworded |
| 05-lambdas-and-method-refs | ok | none | |
| 06-sorting | ok | none | |
| 07-stream-basics | ok | none | |
| 08-collectors | ok | static imports are explained in the lesson | |
| 09-immutable-collections | ok | none | |
| 10-flatmap-reduce | ok | none | |
| 11-boss-log-analyzer | unclear | plain: `line.split(" ", 4)` (limit argument) never shown, and it is what keeps the message in one piece | one sentence in the lesson |

## Left as is (deliberate)

- 00 lessons never explain `public static`, `class` or `package`. The starters hand over every signature, the learner only writes bodies, and a full explanation is off-topic for an install-and-first-green world. Revisit if a real learner stalls here.
- 01/09 (`table`): index `servers[i]` is used one quest before arrays get their own lesson; `args[0]` from 00 covers it.
- 02/05, 02/12: games `status()` / `report()` text is fully specified in the TODOs and labels, so no change.
- Minutes were not changed: nothing added more than about one minute of reading, and the boss budgets (35, 45, 75, 50) were already raised last round.

## Not covered by this pass

- No real beginner has tried any of this. Whether a learner actually finds the new bullets is untested.
- Gradle was not run blind per quest (see method notes).
