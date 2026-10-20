# Blind playtest: Java 02-03 and Lua 01-02 (2026-10-20)

Second pass of [PLAYTEST-2026-10-20.md](PLAYTEST-2026-10-20.md), same question: can a learner who did the earlier worlds of the track solve each quest from the lesson, the `quest.toml` labels and hints, the starter code and its tests?

## Method and honesty notes

- Scope: Java `02-oop` (12 quests), `03-collections` (11); Lua `01-lua-core` (12), `02-factorio-data` (11). 46 quests, 40 with code, 6 read-quests. The games flavour was used for every Java quest that has one (all exercises and bosses).
- Java: each starter went to `%TEMP%\oxide-playtest3\java\<world>-<quest>` and ran with `gradlew.bat test` on JDK 21. I read the test failure text from the Gradle XML results, as the app shows it. Lua: I copied the whole lua track to a scratch `OXIDE_CONTENT`, deleted every `solution/` folder first, then (the catalog refuses a quest without `solution/`) refilled each one with a copy of its **starter** and wrote my attempt over that. I ran `cargo run -p content-check -- lua/<world>/<quest>`. I never opened an original solution.
- I saw only `lesson.md`, `quest.toml` labels, starter files and tests. `mock_factorio.lua` is a library on the learner's `require` path, not a solution: I read its header comment and later grepped two facts from it.
- **I am an LLM, not a beginner.** I know Java, Lua and the Factorio data stage, so almost everything passed first time. "Minutes" are estimates from concept count and from what the lesson does not teach, not timings. I revealed **no hints while solving**. Where a beginner would need one, I say which, and I read that hint afterwards. Those reads are marked "audit".
- Where I did get stuck for real (compile or test failure on my own first attempt) I say so. Four times: Java `02/05` (guessed the armor stand's name), Java `02/08` (`Objects`), Java `03/08` (`Collectors`), Lua `01/08` (the lesson's own pattern).
- Slips of mine, not content faults: a wrong `sed` broke the static imports in `03/08`, and an escaped `\n` in `02/12`. Both fixed by me.
- Gradle ran offline from the default cache, so the first-build time was not measured.
- Nothing is unsolvable. Every quest passed. All 23 Lua quests pass in `content-check` with my attempts; all 23 Java ones pass `gradlew test`.

## Verdict summary

| | ok | unclear | broken |
|---|---|---|---|
| Java 02-oop (12) | 9 | 3 (`05-interfaces`, `08-equals-hashcode-tostring`, `12-boss-bank-ledger`) | 0 |
| Java 03-collections (11) | 9 | 2 (`03-maps` nit, `08-collectors`) | 0 |
| Lua 01-lua-core (12) | 9 | 3 (`07-dicts-and-iteration`, `08-string-patterns`, `12-boss-expression-evaluator` for time) | 0 |
| Lua 02-factorio-data (11) | 10 | 1 (`11-boss-new-item-chain`: unstated icon, time) | 0 |

(`03-maps` is counted ok in the tables below; its note is a nit.)

## Minutes: plan (quest.toml) vs estimate for a beginner

| world | quests | plan | estimate | gap |
|---|---|---|---|---|
| java 02-oop | 12 | 152 | 178 | +26 (boss +15, 08 +3, 11 +3) |
| java 03-collections | 11 | 125 | 140 | +15 (boss +5, 08 +3, 03 +2) |
| lua 01-lua-core | 12 | 162 | 189 | +27 (boss +15, 08 +6, 07 +4) |
| lua 02-factorio-data | 11 | 127 | 100 | **-27** (boss -20) |
| **total** | **46** | **566** | **607** | **+41** |

Lua `02-factorio-data` is over-budgeted: the boss reuses the shapes of quests 03 to 09 and is mostly assembly. Java 02 and Lua 01 bosses are under-budgeted.

## Java, `02-oop` (games flavour)

| quest | solved | est (plan) | hints | verdict | friction |
|---|---|---|---|---|---|
| 01-classes-and-fields | yes | 5 (4) | 0 | ok | Clear. `UnsupportedOperationException("TODO: ...")` stub, as `00/03-reading-red` promises. |
| 02-constructors-and-this | yes | 7 (6) | 0 | ok | The starter's other constructors assign `this.maxStack = 0` before the TODO says to call `this(...)`. A beginner must delete those lines. Lesson says `this(...)` must be first and it is shown. |
| 03-encapsulation | read | 4 (4) | 0 | ok | All three quiz answers are the second choice. |
| 04-static-members | yes | 8 (8) | 0 | ok | `this.id = nextId++` is not shown (lesson uses `++fired` in a prefix form), but the TODO says "take nextId, then move it forward". |
| 05-interfaces | yes | 12 (10) | audit 2 | unclear | **Hidden strings.** `status()` must print `"Zombie: 20 hp"` and `"Armor stand: 6 hp"` (capital A, lower-case s, a space). Neither the lesson, the labels nor the TODOs say so; they exist only in the test. I guessed `"ArmorStand"` and failed: the diff (`expected: "Armor stand: 6 hp" but was: "ArmorStand: 6 hp"`) shows it, hint 2 states it. Half damage for odd amounts (`damage(3)`) is unspecified; the test only uses 4 and 100. The title says "Write a Damageable interface", but the interface exists; only `status()` is written. |
| 06-inheritance-and-super | yes | 9 (10) | 0 | ok | The TODO comments give the exact values. |
| 07-polymorphism | yes | 9 (10) | 0 | ok | `Wave.totalDamage` loop is the lesson's `Tool` loop. |
| 08-equals-hashcode-tostring | yes | 15 (12) | audit 3 | unclear | **Compile trap.** The lesson and hint 3 say `Objects.hash(x, y, z)`, but the starter has no `import java.util.Objects;` and no lesson has taught imports. First build: `cannot find symbol  symbol: variable Objects`. (`10-records` ships the import; this starter does not.) Everything else is well covered; the contract paragraph is good. |
| 09-enums | yes | 10 (8) | 0 | ok | The colours/weights are in the TODO comments, so nothing is guessed. `valueOf(text.trim().toUpperCase())` needs `trim` and `toUpperCase` from `01/08`. |
| 10-records | yes | 9 (8) | 0 | ok | The starter still imports `Objects`, which is unused after the rewrite (no warning shown). `BlockPos` rewrite is easy after `08`. |
| 11-sealed-and-pattern-switch | yes | 15 (12) | 0 | ok | The guard order rule (guard before the plain case) is stated and is needed: `ItemDropped d when d.count() == 0` before `ItemDropped d`. The nested record pattern in `where` is in the lesson. Labels are clear. |
| 12-boss-bank-ledger | yes | 75 (60) | audit 1-6 not read | unclear | See notes below. |

**12-boss notes.**

- Not stated anywhere: the target chest's own log must get an `ItemPlaced` for the moved stack (`b.log()` is asserted), while the source logs the `ItemMoved`. The lesson only says "a take here plus a placement in the `to` chest".
- `apply` needs a **switch statement** with block bodies (`case ItemMoved m -> { ... }`). Lesson 11 only teaches switch **expressions** that return a value. It works, but a beginner has to know a switch can be a statement.
- Order "take first, then place, log last" is required for "rejected leaves no trace". Nothing says to put the log line last, only the TODO "add event to log once it has succeeded", which does. Good.
- `report()` needs `StringBuilder`, `String.join` or a `+=` loop. `String.join` is taught in the `00-setup` boss lesson and `StringBuilder` is only named in `01/08` ("next quests"), so a `+=` loop is the likely route. Fine, but the "1. " numbering needs a counter next to a for-each.
- A rejected move to itself (`a` to `a`) is not tested.
- 5 objectives, tests are readable, failure text is good. About 75 minutes for a beginner.

## Java, `03-collections` (games flavour)

| quest | solved | est (plan) | hints | verdict | friction |
|---|---|---|---|---|---|
| 01-lists | yes | 5 (4) | 0 | ok | The `remove(slot)` vs `remove(Integer.valueOf(id))` trap is taught and tested (`5,6,7,3` with slot 0, id 3). The order matters (slot first, then id) and the TODO says so. |
| 02-sets | yes | 5 (4) | 0 | ok | Lesson names `Arrays.asList` and `split`; `split(" ")` was first taught in `00/08` hints only. |
| 03-maps | yes | 12 (10) | 0 | ok | `name.charAt(0)` is never used in this lesson (taught in `01/08`), and the map key is a `Character` (autoboxing from `char` is silent). Low. |
| 04-order-and-hashing | read | 4 (4) | 0 | ok | Quiz answers are 1, 0, 0. |
| 05-lambdas-and-method-refs | yes | 10 (8) | 0 | ok | Lesson explains captured effectively-final variables, and the `Predicate` factory, so `isOutdated(latestById)` is one line. |
| 06-sorting | yes | 8 (8) | 0 | ok | The three examples cover everything. |
| 07-stream-basics | yes | 9 (8) | 0 | ok | The TODO text is the whole pipeline. |
| 08-collectors | yes | 15 (12) | audit 1 | unclear | **Compile trap.** The lesson writes `Collectors.groupingBy(...)`, `Collectors.joining(...)`; the starter has `import static java.util.stream.Collectors.groupingBy` etc. and **no** `import java.util.stream.Collectors`. Copying the lesson gives 5 `cannot find symbol: variable Collectors` errors. Hint 1 mentions `import static ...Collectors.*`, but after the first hint a learner has already hit 5 errors. TODOs use the short form, the lesson uses the long form. |
| 09-immutable-collections | yes | 5 (5) | 0 | ok | `// TODO: return a read-only list instead` sits on a line that already compiles and passes `containsExactly`. The read-only objective is what fails, which is a fine signal. |
| 10-flatmap-reduce | yes | 12 (12) | 0 | ok | The lesson teaches `IntStream.rangeClosed(1, 3).boxed()` and `Optional.isPresent/orElse`, but no task uses them. Reading cost with no payoff. |
| 11-boss-log-analyzer | yes | 55 (50) | audit 1-6 not read | ok | Fair: the lesson spells out all four methods and gives the sort and the `TreeMap::new` collector. `Integer.parseInt` was last taught in the `00-setup` boss hints (see the earlier report), not in a lesson. `TreeMap::new` as a collector factory argument (`groupingBy(f, TreeMap::new, counting())`) is only in the boss lesson, never practised. The boss is basically a gap-fill: the TODO strings tell each pipeline. |

## Lua, `01-lua-core`

| quest | solved | est (plan) | hints | verdict | friction |
|---|---|---|---|---|---|
| 01-setup | read | 3 (3) | 0 | ok | Quiz is easy. Good Lua 5.2 summary (`goto` exists, no integer type, `table.unpack`). |
| 02-values-and-numbers | yes | 5 (5) | 0 | ok | Test output `tests.lua:9: expected 3, got nil` shows the file and line, as promised. |
| 03-arrays | yes | 9 (10) | 0 | ok | |
| 04-holes-and-nil | yes | 10 (10) | 0 | ok | `compact` has no worked example but the TODO ("loop 1..n, keep the non-nil values") is complete. `table.pack`/`table.unpack(p, 1, p.n)` are 5.2 and the lesson shows them. |
| 05-functions-and-varargs | yes | 9 (10) | 0 | ok | The `or` default trap and `option(false, true)` are both explained and tested. |
| 06-closures | yes | 9 (8) | 0 | ok | Per-iteration `i` in a numeric `for` is explained. |
| 07-dicts-and-iteration | yes | 12 (8) | audit 1, 3 | unclear | **Used before taught.** `count_words` needs `string.gmatch(text, "%S+")`: patterns, and the `%S` class, are the next quest, and `%S` (upper-case negation) is never listed even there. `deep_copy` needs `type(v) == "table"`: `type()` appears in no Lua lesson in the track (only in hint 3 and in the pre-written boss code). Hints 1 and 3 give both. The lesson does give the shallow copy loop as the starting point. |
| 08-string-patterns | yes | 18 (12) | audit 1 | unclear | **The lesson's own example fails the test.** The lesson shows `^(%w+)%s*=%s*(.-)$` and the first test (`"  name=iron-plate  "`) requires leading spaces ignored and trailing spaces trimmed: `expected "name", got nil`. The first hint has the right pattern `^%s*([%w_%-]+)%s*=%s*(.-)%s*$`. Also needed: `[^,]+` followed by a hand-written trim (`^%s*(.-)%s*$`), and `escape_pattern` needs a character set with escapes (`[%^%$%(%)%%%.%[%]%*%+%-%?]`), but sets with `%` escapes inside are never shown and the one `[...]` in the lesson is `[^,]`. The "wrap in `( )`" `gsub` count trap is stated well. |
| 09-metatables-and-oop | yes | 12 (12) | 0 | ok | Everything is in the lesson. `error("read-only")` adds a position prefix; `t.raises` matches by substring, so fine. |
| 10-modules-and-errors | yes | 14 (12) | 0 | ok | `load(code, "chunk", "t", env)` is the 5.2 form and is spelled out. `find_globals` is nearly the lesson's snippet. |
| 11-coroutines | yes | 13 (12) | 0 | ok | `accumulator` needs the rule that the first `resume(co, 5)` value arrives as the body's parameter and later ones as the result of `yield`. The lesson example shows both, but the TODO only says "the value you pass to resume comes back from yield", which is half of it. The test message is clear. |
| 12-boss-expression-evaluator | yes | 75 (60) | audit hints 1-5 seen | unclear (time) | Everything the boss needs is taught, and the stage list and the TODO markers are excellent. Recursive descent has no worked example in the lesson, only the grammar, so the first `power`/`unary` pair is the hard part. The number branch is pre-written (`src:match("^%d*%.?%d*", i)` uses a pattern anchored at an index, which the pattern lesson did not show). 75 minutes is more realistic than 60. `main.lua` demo is a nice finish. |

## Lua, `02-factorio-data`

| quest | solved | est (plan) | hints | verdict | friction |
|---|---|---|---|---|---|
| 01-setup | read | 4 (4) | 0 | ok | Explains what the mock fakes, honestly. `requires` a lua-core quest, so the world cannot start before `10-modules-and-errors`. |
| 02-info-json | yes | 6 (6) | 0 | ok | The starter ships `"factorio_version": "0.12"`, a bad default that is easy to spot. The lesson example is nearly the answer. The label names the exact string `kit-mod`. |
| 03-first-item | yes | 7 (8) | 0 | ok | Lesson example is the answer with the name `kit-alloy`. The first quest that leaves `icon` for you to look up: the lesson shows the exact path. |
| 04-icons-and-paths | yes | 4 (4) | 0 | ok | The starter's `icon = "graphics/icons/kit-alloy.png"` (no root) is the "bug". `__kit-mod__` must match `info.json`'s name; nothing checks it. |
| 05-recipes | yes | 9 (10) | 0 | ok | 2.0 shape taught well. |
| 06-copy-and-tweak | yes | 8 (10) | 0 | ok | `table.deepcopy` is a Factorio global (not in plain Lua 5.2); the lesson says so, and the mock supplies it. |
| 07-technologies | yes | 11 (12) | 0 | ok | After quest 05 teaches `{type, name, amount}` tables, `unit.ingredients` goes back to `{name, amount}` pairs. The lesson comment says so; `validate_prototype` would flag the wrong shape. A learner could stumble here. |
| 08-stage-order | yes | 8 (8) | 0 | ok | `item.stack_size / 2` is a float for odd sizes. The mock rejects non-whole `stack_size`; the vanilla fixture has only even stacks above 50 so it never triggers. Real Factorio would reject 75/2. Not mentioned: a Lua 5.2 trap without a test. |
| 09-first-entity | yes | 10 (12) | 0 | ok | Needs `flags`, `minable`, boxes: all in the lesson. |
| 10-reading-the-prototype-docs | read | 3 (3) | 0 | ok | |
| 11-boss-new-item-chain | yes | 30 (50) | audit hints not read | unclear | The `kit-storage` technology needs `icon` and `icon_size` (it fails the "icons_are_valid" validation without), and the boss text never says so; the reminders list "a placeable container needs an icon" but not technologies. Copy-from-earlier-quests is the intended route. The 8 objectives light up in a good order. Plan says 50 minutes; assembly of already-practised shapes took far less. |

## mock_factorio vs the lessons

- Behaviour matches the lessons in everything I touched: end-of-stage validation, silent overwrite on duplicates, 2.0 recipe format, whole-number `stack_size`, `place_result` / `minable.result` reference checks, the stage log order.
- Gaps the lesson does not warn about: fractional `stack_size` from `/ 2` (above), the `__kit-mod__` prefix not being checked against `info.json`, icon files not existing (stated openly), and mock-only helpers (`mock.pristine`, `mock.load_mods`, `validate_all`) that the tests call but no lesson documents. A learner reading `tests.lua` meets those names cold.
- Lua 5.2 traps: the track covers float-only numbers, `table.pack`/`unpack`, `load` with an env and `goto`. Nothing in these two worlds hit an uncovered 5.2 trap, apart from the `/ 2` float above. I did not probe number-to-string formatting of large floats.

## Ranked top 10 fixes (by impact)

1. **Java `02/08`: no `import java.util.Objects;` in the starter.** The lesson and hint 3 say `Objects.hash`; the first compile gives `cannot find symbol: variable Objects` and no lesson has taught imports. Add the import to the starter (as `10-records` already does), or add one lesson line about `import`.
2. **Java `03/08`: lesson writes `Collectors.x(...)`, starter only static-imports `x`.** Copying the lesson gives five compile errors. Make the lesson use the short form, or make the starter also `import java.util.stream.Collectors;`.
3. **Lua `01/08`: the lesson's `parse_key_value` pattern fails the test.** Update the example to `^%s*(%w+)%s*=%s*(.-)%s*$`, or drop the whitespace cases. Also show one `[...]` set containing `%` escapes before `escape_pattern`.
4. **Lua `01/07` uses `gmatch`/`%S+` and `type()` before they are taught.** Swap order with `08`, or add two lines (`%S+` splits on spaces; `type(v) == "table"`) to the `07` lesson.
5. **Java `02/05`: hidden strings `"Zombie"` and `"Armor stand"`.** Put them in the labels or TODO comments. State the rounding for odd damage or add a test.
6. **Java `02/12` boss: unstated rules.** Say that the target chest logs an `ItemPlaced`; teach (or hint) the switch **statement** form. Budget 75 minutes.
7. **Lua `02/11` boss: `kit-storage` needs an icon and `icon_size`.** Add a bullet to the reminders. Cut the plan from 50 to about 30 minutes.
8. **Lua `01/12` boss: no recursive-descent worked example.** Add a 10-line example (parse `1 + 2` into a `bin` node) to the lesson. Budget 75 minutes.
9. **Java `03/10` and `03/03`: lesson content that no task uses** (`IntStream.rangeClosed`, `Optional.get`), and `charAt(0)` plus `char` to `Character` boxing in `03`. Trim or add a use.
10. **Fill-in-the-blank TODOs across Java 03.** `// TODO: collect(groupingBy(Mod::loader))`, `stream().flatMap(List::stream).toList()`, `computeIfAbsent(first letter, k -> new ArrayList<>()).add(name)` are the whole answer. Prefer "what to do". Low urgency. Also the quiz answers are skewed to the second choice (`02/03` all three, `03/04` 1-0-0).

Minor, not ranked: Lua `02/08` float `stack_size`; Java `02/10` unused `Objects` import left behind; Lua `02/07` `{name, amount}` pairs after `{type, name, amount}` tables; Java `02/05` title says "Write a Damageable interface" while the interface is provided.
