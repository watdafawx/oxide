# Lua track: term ledger

Every term, keyword and piece of syntax, in the order the track introduces it, with the quest
that does. A lesson may use only rows at or above its own quest. Add a row when you introduce
something. Helpers a starter contains (marked *starter*) are shown, not taught.

Paths below are relative to `lua/01-lua-core/`.

| term | quest | how it is introduced |
|---|---|---|
| `tests.lua`, `src/`, save and watch, red/green objective | 01-how-quests-work | the two files and the loop |
| module: `local M = {}`, `function M.name(...)`, `return M`, `end` (*starter*, shown) | 01-how-quests-work | one code block, only the bodies are the learner's |
| `return`, `nil` as "nothing" (named only), `--` comment | 01-how-quests-work | `-- TODO` and `return nil` in every starter |
| `local`, variable, `*`, `+`, `-` | 02-a-name-for-a-count | 4 batches get a name, eggs = batches * 3 |
| `/`, one number type with decimals, `math.floor(x)`, call | 03-halves-and-floors | 7 eggs / 2 is 3.5, round down |
| string `"..."`, `..` glue (numbers glue too, spaces are pieces) | 04-sticking-text-together | "3 eggs" from a number and a word |
| `>=`, `>`, `<`, `==`, `~=`, boolean `true`/`false` | 05-yes-or-no | is the carton full |
| `if` / `elseif` / `else` / `end`, `return` inside a branch | 06-pick-a-branch | eggs per dish by name |
| `nil` as a value, missing argument is nil, `== nil`, falsy (only nil and false), `not` | 07-nothing-is-nil | an empty shelf slot |
| table `{...}`, list, index from 1, `list[i]`, `#list` | 08-a-list | first, last and count of a recipe |
| `for _, v in ipairs(list) do ... end`, `_`, running value | 09-walk-the-list | add up a shopping list |
| `table.insert(list, v)`, `table.remove(list, i)` (returns the item), hole (named, avoided) | 10-add-and-remove | grow and shrink a list |
| giving an existing name a new value (`x = x or 1`), `or` as default, `or` treats false as missing, `value == nil` check | 11-defaults | batches default to 1, a false setting survives |
| function as a value, anonymous `function(a, b) ... end`, `table.sort(list, rule)`, function parameter `fn(x)` | 12-functions-are-values | sort biggest first, apply to each |
| returning a function, closure, upvalue | 13-a-function-that-remembers | multiplier and counter |
| dict `{key = v}`, `t.key`, `t[key]`, `t["a-b"]`, `["a-b"] = v` in a constructor, missing key is nil, `t.key = nil` deletes | 14-a-chest-of-items | count, put in, empty out |
| `pairs`, no promised order, changing existing keys while looping | 15-every-item | total the stock, clear zeros |
| a second name for one table, `==` on tables asks "same table", shallow copy | 16-two-names-one-table | copy the pantry |
| `type(v)`, `"table"`, recursion, deep copy | 17-copy-inside-copy | copy a recipe with its ingredient list |
| `string.match`, pattern, `%d`, `%S`, `+`, `( )` captures, `tonumber`, several values `local a, b = f()`, `return a, b` | 18-pull-numbers-out | "12 brown-sugar" to 12 and "brown-sugar" |
| `string.gmatch`, `[^,]`, set, collecting pieces | 19-split-the-line | cut a recipe line at the commas |
| `function t:name()`, `self`, `t:name()` (colon call), method | 20-a-table-that-acts | a recipe that adds up and describes itself |
| `setmetatable`, metatable, `__index` | 21-share-the-methods | one copy of each method |
| `require`, public (on `M`) vs private (`local`) | 22-files-keep-secrets | a registry that hides its table |
| `error("msg")`, `error("msg", 0)`, `pcall(fn, args)`, `ok, result` | 23-catch-the-error | refuse to bake without eggs |
| `coroutine.yield`, `coroutine.wrap`, generator in a `for ... in` | 24-pause-and-resume | one scaled count at a time |
| `while` (*starter*, boss scanner), `string.sub`, `:match` (*starter*) | 25-boss-expression-evaluator | already written in the boss starter |
| recursive descent, token, parser, evaluator | 25-boss-expression-evaluator | three machines in a row |

## 02-factorio-data

Paths below are relative to `lua/02-factorio-data/`.

| term | quest | how it is introduced |
|---|---|---|
| item, stack, entity, recipe, technology, tech tree, science pack, lab | 01-how-factorio-is-built | the five pieces the game is made of (read) |
| mod folder, `info.json` (named), `data.lua`, `control.lua` (named), prototype, data stage, `data.raw[type][name]`, control stage (named), no `game` in the data stage, vanilla / `base`, `mock_factorio` | 02-how-a-mod-loads | blueprints, not things; why the list is frozen (read) |
| global (a name without `local`), why globals clash and hide typos, `data` as the global Factorio hands you, `local data` hides it | 03-a-table-every-file-sees | read stack sizes from `data.raw`, stop a leaked `total` |
| JSON (quoted keys, `:`, `[ ]`, no trailing comma), mod portal, `[[ long string ]]` | 04-json-is-not-lua | fix Lua habits in info.json text |
| `name`, `version` (three numbers), `title`, `author`, `factorio_version = "2.0"`, `dependencies`, `"base >= 2.0.0"`, `?` / `!` prefixes, dependencies set load order | 05-the-mods-id-card | the mod's ID card |
| `data:extend({ ... })` (method on `data`), array (docs' word for list), `type`, `name`, `icon`, `stack_size`, checked at end of stage | 06-your-first-item | kit-alloy |
| prototype docs: mandatory, optional, default, "Mandatory if", inherits from, `double` | 07-reading-the-prototype-docs | read a page in a minute (read) |
| item group (tab), subgroup (row), `order` string sorting, `"z[name]"` bracket habit | 08-where-it-shows | kit-alloy in the parts row, sorted last |
| file path root `__base__/`, `__kit-mod__/`, forward slashes | 09-paths-start-with-a-root | point at the mod's own picture |
| `icons` layers (bottom to top), `tint`, colour `{r, g, b, a}` 0..1 (or 0..255) | 10-layered-icons | vanilla plate with a colour wash |
| recipe, `ingredients`, `results`, entry `{type = "item", name, amount}`, fluid (named), `energy_required` (default 0.5), old `{name, amount}` / `result` rejected in recipes | 11-a-recipe | 2 iron + 1 copper -> kit-alloy |
| `table.deepcopy` (a Factorio global), silent overwrite on same type+name | 12-copy-and-tweak | kit-gear from vanilla's gear |
| `enabled = false`, technology, `effects`, `unlock-recipe`, `prerequisites`, `icon_size = 256` | 13-locked-behind-research | kit-metallurgy after automation |
| `unit` (`count`, `time`, `ingredients`), `tool` type, short `{name, amount}` pair (research only), `research_trigger` / `count_formula` (for the curious) | 14-the-research-bill | 30 red packs |
| load order (dependencies, then name), `data-updates.lua`, `data-final-fixes.lua`, rounds over every mod, `string.sub` (*starter*) | 15-wait-for-the-other-mod | catch an add-on's item in the kit-parts row |
| item vs entity prototype, `place_result`, `minable` (`mining_time`, `result`), `container`, `inventory_size`, `flags`, `placeable-neutral`, `player-creation` | 16-item-and-entity | the kit-crate chest |
| tiles, `collision_box`, `selection_box`, y grows downward, centred boxes | 17-how-much-room-it-takes | kit-crate's boxes |
| `print` (named, *starter* `main.lua`) | 18-boss-new-item-chain | the whole Kit Mod |
