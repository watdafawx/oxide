# Java 00-setup and 01-basics: golden-shape rewrite

Both worlds rewritten to `docs/CONTENT.md` (one idea per quest, card lessons, term ledger in
`docs/tracks/java-terms.md`). Plain and games flavours exist for every exercise with the same
objective count and order. `content-check java/00-setup` and `java/01-basics` green, `--fast` green.

## 00-setup (8 quests, same count)

| old | new |
|---|---|
| 01-whats-a-jdk | 01-whats-a-jdk (cards; JRE dropped) |
| 02-hello-oxide | 02-hello-oxide (meets the first red: `expected` / `but was`; "the shape every method has for now") |
| 03-reading-red (read) | folded into 02-hello-oxide |
| 04-compile-error | 03-compile-error (one error, a missing `;`; the `int`/`String` mismatch is gone) |
| 05-project-layout | 04-project-layout |
| 06-main-and-args | 05-glue-text (parameter, `+`) and 06-main-and-args (`args[0]`, `args[1]`; the empty-args `if` moved to 01-basics) |
| 07-printing | 07-fill-the-slots (`String.format` with `%s` and `%d`; `printf`, `PrintStream`, `%.2f`, `Locale.ROOT` dropped) |
| 08-boss-tip-calculator | 08-boss-first-report (games: status report; plain: event badge). Smaller on purpose: 4 objectives, 150 XP, 15 min. No doubles or parsing. |

## 01-basics (11 quests -> 24)

| old | new |
|---|---|
| 01-primitives | 01-a-name-for-a-count (new: variables), 02-numbers-that-wrap, 03-bigger-numbers |
| 02-integer-division | 04-integer-division, 05-fractions-with-double (`int[]` result and negative truncation dropped) |
| 03-how-java-runs (read) | 10-how-java-runs (JIT and garbage collector only; bytecode and JVM are in 00-setup/01) |
| 04-if-and-ternary | 06-true-or-false (new: boolean), 07-if-and-else, 08-the-one-line-choice (short-circuit note dropped) |
| 05-switch-expressions | 09-switch-arms (`default -> throw` is written in the starter) |
| 06-loops | 11-counted-loop, 12-while-loop (new), 13-each-character (vowels via `indexOf` became a direct `char` compare) |
| 07-methods-and-overloads | 14-same-name-different-inputs, 21-any-number-of-values (after arrays) |
| 08-strings | 15-text-never-changes, 16-equals-not-double-equals, 17-flip-it-round (`charAt`, `substring` dropped) |
| 09-text-blocks-and-format | 22-build-a-table, 23-text-blocks |
| 10-arrays | 18-an-array-of-values, 19-largest-so-far, 20-flip-in-place (2D arrays and `transpose` dropped) |
| 11-boss-wordle-scorer | 24-boss-wordle-scorer (code unchanged, lesson rewritten, 4 hints) |

No `requires` named an old id; 02-oop/01-classes-and-fields chains to the boss implicitly.

## Dropped, and where later worlds may notice

- `%.2f`, `Locale.ROOT`, `printf` to a `PrintStream`, `Double.parseDouble` / `Integer.parseInt`: no longer taught in these worlds. 02-oop/09-enums mentions `toUpperCase(Locale.ROOT)` (it explains it inline).
- `charAt`, `substring`, `indexOf`: 03-collections/03-maps and 11-boss use `charAt` / `substring` (03-maps explains `charAt` inline; the boss does not explain `substring`).
- 2D arrays, `Arrays.sort`, `Arrays.toString`.

Candidate quests to add if wanted: "Decimals in a slot" (`%.2f`, `Locale.ROOT`), "Pick a letter" (`charAt`, `substring`), "Text to number" (`parseInt`, `parseDouble`).
