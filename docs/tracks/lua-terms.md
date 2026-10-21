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
