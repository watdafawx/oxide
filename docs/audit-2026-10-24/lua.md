# Curriculum audit: `lua` track (01-lua-core, 02-factorio-data)

Scope: every `quest.toml` and `lesson.md` in both worlds. Starter/solution diffed for 01/02, 09, 13, 19, 20, 21, 22, 25 and 02/02-06, 08, 11. Flavours skimmed: 01/01, 01/02, 01/14, 01/21 (games).
Already known, so not repeated as new findings: `docs/PLAYTEST-2026-10-20*-lua.md` (pre-rewrite world 01; examples equal to solutions in world 02, now fixed) and `docs/REWRITE-lua-01.md`. Several old findings come back in a new form below, and I flag them where they do.

---

## World 01: Lua core (25 quests, "Recipe calculator")

| Beginner clarity | Experienced-dev value | Coherence/sequencing | Task-lesson fit | **Overall** |
|---|---|---|---|---|
| 7 | 5 | 6 | 7 (quests 02-24: 8; boss: 3) | **6.5** |

**Verdict.** The rewrite works. Quests 02-24 follow the golden shape closely: a situation intro, kicker cards, one code block, and a bold term with a plain gloss. Most of them pass the blind test for a true beginner. The weak spots are the basics that are *used* before anyone explains them: what a function, parameter and argument are, dot-access on `M`, and giving an existing name a new value. Each of these appears several quests before it is taught, or is never taught at all. The boss is the real problem. Quests 02-24 are 3-6 minute one- or two-liners, then the boss asks for a recursive-descent parser. It uses `while`, `and`, `local function`, forward declarations, `string:sub`/`:match(pat, init)`, escaped pattern sets, `%a`/`%w`, `^` and a written-out grammar notation, and no lesson teaches any of them. An experienced dev gets accurate Lua-isms (1-based indexing, `~=`, the `or`/false trap, holes, `:` vs `.`, `__index`, `pcall`, coroutines), but at a crawl and with no fast track. Several staples are missing: numeric `for`, `while`, `string.format`, `gsub`, varargs, `table.concat`, globals.

### Worst quests
1. **`01-lua-core/25-boss-expression-evaluator`**: a difficulty cliff, with things used that were never taught (checked against the solution):
   - `while true do … end` loops. `while` is in no lesson. The ledger marks it "*starter*", but the learner must *write* two of these loops in `term`/`expr`.
   - `and` (`nxt.kind == "op" and (nxt.text == "*" or …)`). `and` is never taught. `or` was only taught as "default" (11), not as "either condition".
   - The `^` power operator. It appears in no lesson, though tests and stages depend on it.
   - Pattern sets with escapes `[%+%-%*/%^]`, classes `%a`/`%w`, and `src:match("^[%w_]+", i)` (anchored at an index). Stage 4 needs these and 18/19 never showed them.
   - Method-style string calls `c:match(...)`, `src:sub(i, i)`, `("…"):format(...)`, and `local function` / `local expr, term, unary, power` forward declaration (in the starter, never explained).
   - Grammar notation `{ … }`, `[ … ]`, `|`, plus "tree", "node", "precedence" and "right-associative", with no worked example of one rule turned into code. The old playtest already asked for a 10-line worked example. The rewrite still does not have one.
   - Spine drift: "a formula engine" barely uses the recipe calculator. Only `eggs` in the env ties it in.
   - 75 minutes after 23 quests of about 5 minutes each.
2. **`01-lua-core/01-how-quests-work`**: the module card shows `local M = {}`, `function M.double(n)` and `return M`. It calls the module "a table of functions" seven quests before tables (08) and thirteen before dot-keys (14). It never explains what a function, a parameter (`n`) or a call is, or who receives `return M`. Every later starter depends on these ideas.
3. **`01-lua-core/09-walk-the-list`**: the example `biggest = count` and the hint `total = total + count` reassign an existing name without `local`. The ledger says this is introduced in `11-defaults`, and 11 only shows it in a code comment. Nothing explains why the second assignment has no `local`, or that `total = total + count` reads the old value first. Objective 2 needs a counter (`n = n + 1`), and the lesson never shows one; its only example is a "biggest" tracker. `ipairs` is never unpacked ("index pairs"), and the generic `for … in` form is never explained as "the loop gives you a new pair each round".
4. **`01-lua-core/24-pause-and-resume`**: the "why" is weak, and slightly misleading for a beginner ("a lot of memory if the calculator built every scaled recipe up front"; recipe lists are tiny). The lesson never shows the two halves running side by side (who is paused, who resumes), and no plain loop-with-a-list version comes first to motivate the generator. The TODO is the whole answer.
5. **`01-lua-core/19-split-the-line`**: `{count = count, name = name}` (key name = variable name) is never explained. To a beginner it looks like a no-op. Calling a function on the module itself (`M.split_commas(line)` inside `M.parse_recipe`) is also new and passes without comment.
6. **`01-lua-core/14-a-chest-of-items`** (plain flavour): the plain test uses `{eggs = 12, ["brown-sugar"] = 3}`, but the plain lesson only shows `pantry["brown-sugar"]` for reading. The constructor form `["a-b"] = v` (in the ledger) is only in the *games* lesson.
7. **`01-lua-core/23-catch-the-error`**: no "why". The lesson never says when to raise an error versus return `nil`, and "protected" in "protected way" is not glossed. The TODO comments in 21, 22, 23 and 24 contain the full solution line (`-- TODO: return pcall(M.bake, eggs, needed)`), so the exercise turns into copy-typing.

### Best quests (models)
- **`10-add-and-remove`**: a situation, the move, then the trap and *why not `= nil`* (holes break `#`). One idea, the reason is given, and the test checks the reason.
- **`11-defaults`**: it teaches `or`, then deliberately breaks it with `false`, then shows the correct `== nil` check. It has the house style's "make the obvious mistake" beat.
- **`16-two-names-one-table`**: "watch it go wrong" before the term. The aliasing explanation suits a beginner, and the `==`-on-tables card is a good insight for experienced devs.
- **`03-halves-and-floors`** and **`05-yes-or-no`**: each new symbol gets a gloss, and the "surprise" card (3.5) teaches *why* `math.floor` exists.
- **`21-share-the-methods`**: it motivates metatables with a real cost (200 copies, built up in 20), and the TODO path is clear.

---

## World 02: Factorio data stage (11 quests, "Kit Mod")

| Beginner clarity | Experienced-dev value | Coherence/sequencing | Task-lesson fit | **Overall** |
|---|---|---|---|---|
| 3 | 8 | 5 | 7 | **5.5** |

**Verdict.** World 02 was not rewritten. It still uses the old dense style: `**Why now:**` labels, field-reference bullet lists, several code blocks and no kicker cards. It breaks the CONTENT.md shape that world 01 now follows, so the jump between worlds is jarring. For someone who already plays Factorio and knows some modding, it is excellent: accurate 2.0 details (the recipe `{type,name,amount}` shape, `factorio_version` with two numbers, stage order, `deepcopy` vs reference, reading "Mandatory if…" in the docs). Someone who has **never modded Factorio** cannot follow it. Core words are never defined: *prototype*, *vanilla*, *entity* vs *item*, *subgroup*/*order*, *science pack*/"tool item", *flags*, *mod portal*, *JSON*. The game model behind them is never sketched (items sit in inventories, entities stand on the map, recipes turn items into items, technologies unlock recipes, labs consume science packs). Most TODOs and labels give the exact values, so the tasks are achievable, though often by transcription.

### Worst quests
1. **`02-factorio-data/01-setup`**:
   - `requires = [] # TMP`: a leftover. The world unlocks without any Lua, although `REWRITE-lua-01.md` says it should require `01-lua-core/23-catch-the-error`.
   - "Prototype" is used ("a prototype is plain data") and never defined.
   - It never says what `data`, `mods` or `settings` are, or that they are **globals** (world 01 said "Always write `local`" and never explained globals). Same for `data:extend` (colon call on a table nobody created).
   - `print` is used here for the first time in the track.
   - It throws in `settings*.lua`/`control.lua` stage names without context.
   - The "`pairs` order" sentence at the end is out of place.
2. **`02-factorio-data/02-info-json`**:
   - JSON is never explained (what it is, why it is not Lua, quotes around keys, `:` not `=`, `[ ]` lists, no trailing commas).
   - The starter is `return [[ … ]]`. Lua long strings `[[ ]]` were never taught, and the file returns a string rather than a module, which contradicts 01/01.
   - "Mod portal" and "lints clean" (objective label) are not glossed. The dependency grammar `[prefix] name [operator version]` is given as notation without a single "read it like this".
3. **`02-factorio-data/09-first-entity`**:
   - "Entity" vs "item" is the key Factorio idea (same name `kit-crate`, two prototypes) and gets one sentence.
   - The box coordinate system is never introduced: units are tiles, y grows downward, and why 0.35/0.4 rather than 0.5.
   - "`placeable-neutral` is what the docs tie the mandatory icon to" is circular and means nothing to a beginner.
   - "The entity base" (inheritance) is mentioned before quest 10 explains inheritance.
4. **`02-factorio-data/10-reading-the-prototype-docs`**: in the wrong place. It is a meta-skill that explains "mandatory", defaults and inheritance, all of which 03-09 already used. Its own intro says the boss needs it, but the boss gives every field anyway. Move it to right after 03.
5. **`02-factorio-data/05-recipes`** and **`07-technologies`**:
   - 05 is framed as migrating "the old `{name, amount}` recipe". A beginner has never seen the old shape, so the starter teaches the wrong shape first.
   - 07 then *requires* `{name, amount}` pairs for `unit.ingredients`, directly after 05 taught that this shape is "rejected".
   - 07 piles on noise the task does not need: `count_formula`, `research_trigger`, "tool items", "lab speed", `icon_size = 256` with no explanation of why 256.
6. **`02-factorio-data/08-stage-order`**:
   - The objective (halve every vanilla stack over 50) has nothing to do with the Kit Mod chain, and the boss never uses stages.
   - "mod A, then mod B" never says what decides load order (dependencies, then name).
   - The starter's TODO comment *is* the solution. The test computes `stack_size / 2` while the lesson insists on `math.floor`; this is consistent only because vanilla stacks are even (already known).
7. **Spine drift** (world level): the blurb says "turn the recipe calculator into a real mod", but nothing from world 01's calculator is reused except the boss's read-only `main.lua`. Lessons in 04 (own icon files), 06 (`deepcopy`) and 08 (stages) feed nothing into the boss.

### Best quests (models)
- **`06-copy-and-tweak`**: it ties back to world 01's alias and deep-copy lessons (16/17), the starter contains the bug, and the TODO says exactly where to wrap. It is the best bridge between the worlds.
- **`11-boss-new-item-chain`**: a clear chain diagram, staged objectives (objective 1 green at the start), helpful run output and an optional "load it in Factorio" reward. It is a good shape for a beginner boss.
- **`04-icons-and-paths`**: short, a quick win, and the starter's broken path is a real, common bug.

---

## Recurring unexplained concepts and syntax

| term / syntax | first used | explained | notes |
|---|---|---|---|
| function, parameter, argument, "call your own function" | 01/01 (`function M.double(n)`) | never as a concept. "Call" is glossed in 01/03 (for `math.floor`), "parameter" first used unglossed in 01/07 | foundational |
| `M.name` dot access, module = table | 01/01 | 01/08 (table), 01/14 (`t.key`) | 13 quests early |
| reassigning an existing name (`x = x + 1`, no `local`) | 01/09 (example and hints) | 01/11, only as a code comment | ledger says 11 |
| counter pattern `n = n + 1` | 01/09 objective 2 | 01/13 (inside a closure) | |
| `math.` / `table.` / `string.` as "libraries" | 01/03 | never ("`table` library" in a 01/10 hint) | links to modules |
| `while` loop | 01/25 boss (learner must write it) | never | ledger marks it *starter* only |
| numeric `for i = 1, n` | never in world 01. The boss test helper uses `for i, tok in ipairs` | never | a Lua staple, missing |
| `and`, `or` as boolean combinators | `or` 01/11 (default only); `and` in 01/25 solution and world 02 tests | never | |
| `^` power | 01/25 | never | |
| `local function f`, forward declaration | 01/25 starter, 01/24 test helpers | never | |
| `s:sub`, `s:match`, `("…"):format`, `%d` format codes | 01/25 starter and solution | never (01/20 taught `:` only for own tables) | |
| pattern classes `%a`, `%w`, `%s`, escaped sets `[%+%-]`, `match(s, p, init)` | 01/25 | 01/18-19 teach only `%d`, `%S`, `[^,]`, `+` | |
| `print`, `tostring` | 01/25 `main.lua`, 02/01 lesson | never | |
| globals (what they are, why to avoid them, why `data` is one) | 01/02 ("always write local"), 02/01 `data`, `table.deepcopy` | never | |
| `[[ long strings ]]` | 02/02 starter | never | |
| JSON | 02/02 | never | |
| "array" (vs "list") | 02/03 | never; world 01 says "list" | vocabulary switch |
| prototype, vanilla, entity, subgroup, order string `a[x]-z[y]`, science pack/tool, flags | 02/01-09 | partly (entity 02/09, subgroup in a hint) or never | Factorio model |
| "in place" | 01/12 and 01/15 labels | never | |
| tree, node, precedence, associativity, grammar notation | 01/25 | named only | |

Recurring "why" gaps: why `local` (no global ever shown going wrong), why 1-based (it is a Lua fact; say so), when to `error` vs return `nil` (01/23), why coroutines (01/24), why a stage file and not `data.lua` (02/08 explains it, good), why `icon_size` is 256 (02/07).

Pattern: **TODO comments that are the solution** (01/21-24; 02/03, 02/05, 02/06, 02/08, 02/11). CONTENT.md says the TODO states *what* in plain words and the last hint states the exact call. Many here state the exact call already.

---

## Prioritized improvements

1. **Fix `02-factorio-data/01-setup` `requires = [] # TMP`.** Restore `requires = ["01-lua-core/23-catch-the-error"]`, or the boss of 01.
2. **Bridge into the world 01 boss: add 3-4 small quests before 25.**
   - (a) `while` and `and`/`or` in conditions ("keep taking while the next token is +").
   - (b) string methods `s:sub(i, i)`, `s:match(p, i)`, and the classes `%a %w %s` plus escaped sets.
   - (c) `local function` and forward declaration, plus `^`.
   - (d) a "mini parser": nested `{type = "bin", …}` tables as a tree, and evaluating it recursively. This is a worked example of one grammar rule turned into code.
   - Alternatively, cut the boss's stage 4 (names) and give the scanner fully written.
3. **Rewrite world 02 in the golden shape**: an unlabelled intro, kicker cards, one code block, `## Your turn`, and a bold gloss for every Factorio term. Add a read quest or a first card, "How Factorio is built": items, entities, recipes, technologies, science packs and labs, prototypes as "blueprints the game reads at start-up". Move `10-reading-the-prototype-docs` to just after `03-first-item`.
4. **Fix the basics in 01/01-01/09.** Add one card in 01/01 or 01/02: "A function is a named recipe of steps; the names in brackets are its inputs (parameters), and the caller fills them in (arguments)." In 01/09, add a card on reassignment ("no `local` the second time: you are changing the name you already have") and show `n = n + 1` counting.
5. **Teach globals once** (a card in 01/22): what happens without `local`, why modules avoid globals, and the fact that Factorio hands you `data`, `mods` and `table.deepcopy` as globals. World 02 relies on this.
6. **Make TODOs say what, not the code** (01/21-24, 02/05-08): move the exact call into the last hint.
7. **02/05 and 02/07**: teach the 2.0 recipe shape straight (the starter has an empty `ingredients = {}`, with "old tutorials" as a side card), then use 07 to say explicitly "research ingredients are the one place the short pair is still used". Cut `count_formula` and `research_trigger`.
8. **Tie 02 to the spine**: let the boss's `main.lua` call the learner's own world 01 functions (for example the `parse_recipe` totals), or drop the "calculator becomes a mod" promise. Make 08 serve the Kit Mod (move `kit-*` items into the subgroup) instead of halving vanilla.
9. **Experienced devs**: add a "Lua for programmers" read or skip test at 01/01 that unlocks 01/08 or 01/14 directly, and put the dropped 5.2 staples (`string.format`, `gsub`, varargs/`select`, `table.concat`, numeric `for`, `goto`, `__newindex`/`__tostring`) into an optional "Lua more" world.
10. Small fixes: show `["a-b"] = v` in the plain 01/14 lesson; explain `{count = count}` in 01/19; gloss "in place"; teach `print`; call it "list (Factorio docs say *array*)" in 02/03.

### Example rewrite: `02-factorio-data/03-first-item`, intro and first cards

> Your calculator knew recipes because you typed them into a table. Factorio does the same: before a map loads, it reads one giant table of everything that can exist, and your mod adds a row to it.
>
> **Blueprints, not things.** Every kind of thing in Factorio (an iron plate, a recipe, a chest) starts as a **prototype**: a plain table that describes it, like a blueprint describes a house. The game reads all prototypes once at start-up, before any map exists, and builds the world from them.
>
> **A table handed to you.** Factorio gives your file a ready-made table called `data`. You did not write `local data = ...`: the game made it and lets every mod see it. A name like that, shared by every file, is a **global**.
>
> **Add your row.** `data:extend({ ... })` adds prototypes. It takes a *list* of tables (the docs call a list an "array"), so even one item goes inside an outer `{ }`. Forget the outer brackets and the game says "Expected array of prototypes".
>
> **What an item needs.** Every prototype has a `type` ("item") and a `name` that no other item uses. An item also needs a picture (`icon`) and how many fit in one inventory slot (`stack_size`, a whole number of at least 1).

### Example rewrite: `01-lua-core/09-walk-the-list`, new card after "Keep a running value"

> **Change, don't remake.** Inside the loop, `total = total + count` has no `local`. `local` makes a *new* name, and you want the one you made before the loop. Lua works out the right side first, using the old total, then stores the answer back under the same name. Counting works the same way: `n = n + 1` adds one to `n` each time the `if` says yes.

---

## What a true non-programmer is missing (input for a "general programming for beginners" world)

- **What a program is**: lines run top to bottom, one at a time; what "run" means; that a test is just another program that calls yours.
- **Functions as a concept**: define vs call, parameters vs arguments, return vs print, where the returned value goes. The track assumes this from 01/01.
- **Variables and state**: assignment as "store into a box", reassignment, reading a name's value at a moment in time, scope (`local` inside a function vs a file), what a global is and why it is risky.
- **Reading errors**: how to read `file:line: message`, a stack trace, "attempt to index a nil value", "attempt to call a nil value", "attempt to perform arithmetic on a string". These are the errors Lua beginners hit most, and no quest names them.
- **Control flow in general**: `while`, numeric `for`, `break`, combining conditions with `and`/`or`/`not`, nesting.
- **Data shapes**: list vs dictionary vs nested table, drawing a table on paper, the words "array", "object" and "record" that docs use.
- **Text vs number**: why `"3"` is not `3`, converting both ways (`tonumber`, `tostring`), `print` for looking inside.
- **Debugging habits**: print the value, change one thing at a time, read the test to see what it expects.
- **Files and formats**: what a file, a folder and a path are (Factorio's `__base__/…` paths assume this), and what JSON is.
- **Reading documentation**: how to read a function signature, "optional", "default", "mandatory", "inherits".
- **Vocabulary**: library, module, call, expression vs statement, syntax error vs runtime error, "in place", "iterate".
