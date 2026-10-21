# Lua world 01 (lua-core) rewrite

12 quests became 25 (24 exercises/reads + the boss), each one idea, 3-6 minutes, in the golden shape
(unlabelled intro, kicker cards, one `lua` block, Your turn). Every exercise has a plain twin
(kitchen: pantry, eggs, pancakes) and a `games` flavour (Factorio: chest, iron-plate, gears) with the
same objectives in the same order. Term ledger: `docs/tracks/lua-terms.md`.

Through-line: a recipe calculator. Boss = formula engine (`2 * (eggs + 3)`).

| old quest | new quests |
|---|---|
| 01-setup | 01-how-quests-work |
| 02-values-and-numbers | 02-a-name-for-a-count, 03-halves-and-floors, 04-sticking-text-together, 05-yes-or-no, 06-pick-a-branch, 07-nothing-is-nil |
| 03-arrays | 08-a-list, 09-walk-the-list, 10-add-and-remove (`sort`/`concat` moved: sort is in 12) |
| 04-holes-and-nil | holes are a card in 10-add-and-remove; `select("#")` / `table.pack` / `compact` dropped |
| 05-functions-and-varargs | 11-defaults (default with `or`, the `false` trap), 12-functions-are-values; multiple return values appear in 18/23 (`local a, b = f()`); varargs dropped |
| 06-closures | 13-a-function-that-remembers (counter, multiplier); memoize and loop-capture dropped |
| 07-dicts-and-iteration | 14-a-chest-of-items, 15-every-item, 16-two-names-one-table, 17-copy-inside-copy (word counting dropped) |
| 08-string-patterns | 18-pull-numbers-out, 19-split-the-line; `gsub` and `escape_pattern` dropped |
| 09-metatables-and-oop | 20-a-table-that-acts, 21-share-the-methods; `__add`, `__tostring`, read-only proxy dropped |
| 10-modules-and-errors | 22-files-keep-secrets, 23-catch-the-error; error objects, `with_context`, `find_globals` dropped |
| 11-coroutines | 24-pause-and-resume (generator only; two-way values and round-robin dropped) |
| 12-boss-expression-evaluator | 25-boss-expression-evaluator (same engine and stages, new lesson, flavour test names) |

## Notes

- 25 quests is above the "16-20" estimate: the old dense quests split into more one-idea quests than that. Several ideas the old world carried were dropped rather than split further (see above); they are candidates for a later "Lua more" world.
- The boss keeps its five staged hints (the spec says two; a 75-minute boss needs the staged help).
- `02-factorio-data/01-setup` requires `01-lua-core/23-catch-the-error` (was `10-modules-and-errors`): modules and errors are what that world needs.
- `docs/CONTENT.md` still cites `01-lua-core/03-arrays` as the Lua worked example; point it at `08-a-list` (not edited here, outside this task's ownership).
- Moved aside (not in the commit): the old quest folders, still in git history.
