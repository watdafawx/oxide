# Blind playtest 2026-10-20b: Lua worlds 01-lua-core and 02-factorio-data

Method: `docs/PLAYTEST-CHECKLIST.md`. Each quest solved from lesson + labels + starter + tests only (hints and `solution/`
unopened until the audit). Runner: a throwaway cargo crate in `%TEMP%\oxide-playtest\runner` calling
`oxide_core::lua::run` on a copy of each starter (same embedded Lua 5.2 and `lib/` as the app). Final check:
`cargo run -p content-check -- lua/<world>` green for both worlds.

**Honesty note:** the player is an LLM that already knows Lua and the Factorio data stage, so every quest passed on the first
run with 0 hints. "Missing" below comes from the solution audit (what the solution uses that lesson + earlier quests never
showed), not from getting stuck.

## World 01: Lua core

| Quest | Solved | Hints | Verdict | Missing before fix | Fixed |
|---|---|---|---|---|---|
| 01-setup (read) | n/a | 0 | unclear | quiz right answers were the longest choice; the `local M = {} ... return M` module shape of every starter was never shown | choices rebalanced; module example added to the lesson |
| 02-values-and-numbers | yes | 0 | unclear | `==`, `or`/`and`/`not`, `elseif`, `^`, `%s` and the `("..."):format` form (later quests/boss use them) | one bullet each |
| 03-arrays | yes | 0 | ok | `_` as an unused loop name | one line in the example |
| 04-holes-and-nil | yes | 0 | unclear | `...` in a parameter list, `~=`; TODOs for compact/pack did not say what to write | vararg + `~=` bullet; TODOs rewritten; minutes 10 to 12 |
| 05-functions-and-varargs | yes | 0 | ok | none | none |
| 06-closures | yes | 0 | ok | none | none |
| 07-dicts-and-iteration | yes | 0 | unclear | recursion (a function calling itself) never named | sentence added; TODOs say the steps |
| 08-string-patterns | yes | 0 | unclear | `tonumber` (solution), what a gsub function receives; TODOs vague | bullet + TODOs |
| 09-metatables-and-oop | yes | 0 | unclear | `error("msg")` used before quest 10 teaches it; `__newindex(tbl, key, value)` signature; `__index = t` form | bullets; TODOs name `setmetatable`/`error` |
| 10-modules-and-errors | yes | 0 | ok | none (solution's `assert(load(...))` is covered) | none |
| 11-coroutines | yes | 0 | unclear | `while` loops, first `resume` value arriving as parameter; round_robin TODO too short | two bullets, TODOs |
| 12-boss-expression-evaluator | yes | 0 | ok | `a ^ b` power operator, `elseif`/`else` (covered by 02 now) | operator line in lesson |

## World 02: Factorio data

| Quest | Solved | Hints | Verdict | Missing before fix | Fixed |
|---|---|---|---|---|---|
| 01-setup (read) | n/a | 0 | unclear | quiz Q2 right answer was the longest; "data.lua registers prototypes" not stated | choices rebalanced; sentence added |
| 02-info-json | yes | 0 | broken | lesson example was the solution (name kit-mod, title Kit Mod); starter had no TODO | example now a different mod; TODO comment added |
| 03-first-item | yes | 0 | broken | lesson example was the solution (kit-alloy, stack 100, same order) | example now kit-bolt/copper icon/stack 200; TODO lists fields |
| 04-icons-and-paths | yes | 0 | unclear | example near-identical to solution; starter's broken `icon` root unexplained | example kit-bolt; paragraph on `__kit-mod__`; minutes 4 to 5 |
| 05-recipes | yes | 0 | broken | example was the solution (2 iron + 1 copper, 2 s, 1 alloy) | example kit-bolt 3 iron + 1 copper, 0.75 s, 4 results |
| 06-copy-and-tweak | yes | 0 | broken | example was the solution (gear, 200, 4); TODO did not say deepcopy goes around the right-hand side | example copper-cable to kit-wire; TODO precise |
| 07-technologies | yes | 0 | broken | example was the solution (kit-metallurgy, automation, 30/15/1) | example kit-welding, steel-processing, 50/20/2 |
| 08-stage-order | yes | 0 | broken | example was the solution (halve stacks over 50); TODOs vague | example caps stacks at 200 and shows one-item indexing; TODOs name loop and field |
| 09-first-entity | yes | 0 | broken | example was the solution (kit-crate, 48, 0.2, 0.35 boxes); box format unexplained | example kit-bin; `{ {left, top}, {right, bottom} }` explained |
| 10-reading-the-prototype-docs (read) | n/a | 0 | broken | Q1 and Q2 not decided by the lesson; Q2 and Q3 right answers longest | lesson states "Mandatory if ..." and the recipe example; choices rebalanced |
| 11-boss-new-item-chain | yes | 0 | ok | nothing new; stale "capstone quizzes you" sentence | sentence removed |

## Deliberately left

- Boss 02 `src/main.lua` (Run script) uses `%-10s`/`%g` format codes, `name:find`, `table.concat`; it is a read-only demo, not graded.
- 09 solution uses `error("read-only", 2)` (level 2) and `("(%s, %s)"):format`; tests pass without, the TODO asks only for `error("read-only")`.
- 08 solution matches keys with `[%w_%-]+`; tests only need `%w+`.
- Beginner minutes are estimates from concept count, not a stopwatch; the player is an LLM.
