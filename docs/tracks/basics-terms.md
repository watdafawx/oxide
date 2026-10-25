# Programming 101 track (`basics`): term ledger

Every term, keyword and piece of syntax, in the order the track introduces it, with the quest
that does. A lesson may use only rows at or above its own quest. Add a row when you introduce
something. Helpers a starter contains (marked *starter*) are shown, not taught.

The track is optional and has no locks (`requires = []` on every quest), but it is written to be
read in order: a quest may assume every row above it. Paths below are relative to `basics/`.

| term | quest | how it is introduced |
|---|---|---|
| program, instructions run top to bottom, source code, `.lua` file, running / executing, "computers are literal" | 01-what-a-program-is/01-instructions-in-order | a welcome screen as a recipe |
| `--` note (named only; full lesson in 02/08) | 01/01-instructions-in-order | one sentence on the first code block |
| `print("...")`, quotes mark text, output, Run button, one print = one line | 01/02-say-hello | two lines on the shop's screen |
| processor, machine code, interpreter, compiler, compiling / building, compile error vs runtime error, Java's in-between code | 01/03-translate-or-read-aloud | read aloud vs translate a book |
| terminal / console / shell, command, `$` prompt, folder, path `src/main.lua`, `cargo run`, `cargo test`, `gradlew test` | 01/04-the-terminal | `lua main.lua` and its output |
| error message `file:line: message`, call (run by name), `nil` = nothing, "attempt to call global ... (a nil value)", read-fix-one-thing habit, **why?** shows the full text | 01/05-read-the-red | `prnt` on line 4 |
| function (named block), `function M.name()` / `end` (*starter* lines), `return` hands a value back, print vs return (first mention), `local M = {}` / `return M` packaging (*starter*), test, `t.eq(actual, expected)`, `expected X, got Y`, green = done | 01/06-what-a-test-is | read tests.lua to find the answer |
| value, number literal, decimals with `.`, `+ - * /`, precedence, brackets, expression | 02-values-and-names/01-numbers-and-maths | 4 tickets and a popcorn, split 4 ways |
| variable, `local name = value`, `local` (named; why in 05/04), naming rules (`_`, no spaces/dashes/leading digit) | 02/02-a-name-for-a-value | the ticket price written once |
| assignment `=` (store, not "equals"), reassignment without `local`, tracing, Rust `let mut` / `-=` | 02/03-the-value-changes | seats down, till up |
| input in the brackets (informal parameter), the caller fills it, two inputs matched in order | 02/04-an-input | price for any number of tickets |
| string, single or double quotes, `..` concatenation, numbers glue as text, Java/JS `+` for text | 02/05-text | "Up, seat 14" |
| type, `type(v)`, `"number"` / `"string"`, `"3" == 3` is false, `==` (first use), `tonumber`, `tostring`, `tonumber` gives nil, why types exist, static (Rust/Java) vs run-time checks (Lua) | 02/06-kinds-of-values | the keypad sends text |
| boolean `true` / `false`, comparisons `< > <= >= == ~=`, `!=` in other languages | 02/07-true-or-false | sold out? old enough? |
| comment `--`, commenting out, `--[[ ... ]]`, `//` and `/* */` elsewhere, comments say why | 02/08-notes-for-people | switch the staff discount on |
| `if cond then ... end`, condition, `return` leaves early, indentation, Rust/Java `{ }` | 03-decisions/01-only-sometimes | stop riders under 120 cm |
| `else`, branch, exactly one branch runs, edges | 03/02-this-or-that | green or red light |
| `elseif`, first true branch wins, narrowest test first, `else if` in Rust/Java | 03/03-more-than-two | three ticket prices |
| `if x = 1` syntax error ("'then' expected near '='"), syntax error found before running, returning a comparison directly | 03/04-equal-or-not | the staff door code |
| `and`, truth table, named boolean parts, `and` stops early | 03/05-both-must-hold | 140 cm and 10 years |
| `or`, inclusive or, `x == a or "b"` trap (any text counts as yes), `and` before `or`, brackets | 03/06-either-will-do | toddlers or season pass |
| `not`, `not (a and b)`, De Morgan's laws (named), `!` `&&` `||` in Rust/Java | 03/07-not | is the park not full |
| loop, `while cond do ... end`, iteration, infinite loop, 5 s limit, zero iterations | 04-repetition/01-while-not-there-yet | weeks until 10 km |
| `for i = a, b do ... end`, accumulator / running total (made before the loop), for vs while, Rust `1..=30` / Java `for (...)` | 04/02-count-with-for | push-ups over a month |
| step `for i = a, b, step`, negative step, empty `""` text accumulator, where a loop ends | 04/03-counting-down | "5 4 3 2 1 go", breaks every 15 min |
| counter, `if` inside a loop, `local function` helper (*starter*, named), `%` remainder (*starter*, named), nesting | 04/04-count-the-matches | running days in a month |
| `break`, `return` inside a loop, nil for "not found", why stopping early matters | 04/05-stop-early | first day over 10 km |
| `continue` (other languages), skip with an `if` in Lua, two accumulators, average, divide-by-zero check (`nan`) | 04/06-skip-a-round | average without ill days |
| tracing table, off-by-one error, check first/last/zero rounds, `print` to trace | 04/07-trace-a-loop | trace `total + day * 2` |
| define vs call, writing `function M.name() ... end` yourself, `M.` puts it in the package, call needs `()`, function without `()` is the function itself, why functions | 05-functions/01-define-and-call | base price and topping price |
| parameter, argument, matched by position, missing argument is nil, "attempt to perform arithmetic on local ... (a nil value)" | 05/02-parameters-and-arguments | pizza price by toppings |
| print vs return (the bug), a call is an expression, calling your own function from another, print only at the top | 05/03-return-or-print | the printed delivery fee |
| scope, `local` lives until its block's `end`, global, "global 'x' (a nil value)", why globals are risky, share by returning or passing | 05/04-where-a-name-lives | price only inside base_price |
| decomposition, one job per function, test the pieces, build up smallest first | 05/05-split-the-work | food, discount, delivery |
| list / array, `{a, b, c}`, index `list[i]`, counting from 1 (0 in Rust/Java), `#list`, `list[#list]`, past the end is nil | 06-collections/01-a-list | a basket of prices |
| `list[i] = v`, `table.insert(list, v)`, `table` library, `{}` empty list, a function changes the list it is given (why: 07/02) | 06/02-grow-the-list | scan and correct a price |
| loop over `1, #list`, `ipairs`, `_` for an unused name, the accumulator pattern on lists, Rust/Java for-each | 06/03-every-item | basket total, expensive items |
| champion (max/min), start at the first item, flip `>` to `<`, nil for an empty list | 06/04-the-biggest | dearest and cheapest item |
| map / dictionary / lookup table, key, value, `{k = v}`, `t[key]`, key from a variable, missing key is nil, `~= nil` check, Rust `Option` / Java `null` (named) | 06/05-look-it-up | price of "milk" |
| add/change `t[k] = v`, remove with `= nil`, dot `t.k` = `t["k"]`, brackets for spaces/variables, module (M is a table of functions), values of any type | 06/06-the-dot-and-the-package | add and discontinue products |
| `pairs`, no promised order, one key looks up two tables, ipairs vs pairs | 06/07-every-key | stock count and value |
| memory / RAM, address, variable as a label, reference / pointer, garbage collector, stack, heap | 07-how-the-computer-holds-things/01-numbered-boxes | where a picture lives |
| numbers copy, tables share, two names one table, tables passed to functions by reference, real copy in a loop | 07/02-two-names-one-table | the undo bug |
| table `==` is identity, why, compare contents by length then slots, Java `==` vs `.equals` | 07/03-same-or-alike | the unsaved-changes star |
| bit, binary place values, byte (0-255, 256 values), multi-byte numbers, Rust `u8` / `u32`, KB / MB / GB | 07/04-bits-and-bytes | a colour channel |
| hexadecimal, digits A-F, two hex digits = one byte, `0xFF`, `tonumber(text, 16)`, `string.sub(s, i, j)` | 07/05-hex-colours | "#FF8800" to {255, 136, 0} |
| leak, use after free, owner, move, borrowing (Rust, named), Java references, `null`, "billion-dollar mistake", Rust `Option` | 07/06-why-it-matters | copy or share in each language |
| reference manual, signature, `[optional]` arguments, default, "returns a copy", `string.rep(s, n [, sep])`, `string.upper(s)`, try it small | 08-build-something/01-read-the-manual | header underline and capitals |
| debugging as a search, test the pieces, `print(a, b, ...)` several values, output under **why?**, compare with paper, one change at a time | 08/02-narrow-it-down | 9 points instead of 7 |
| record (object / struct), field `t.name`, list of records `teams[1].name`, "attempt to index a nil value" | 08/03-tables-inside-tables | teams and most wins |
| (boss: nothing new) new record, find by name, points, result with three branches, leader, table lines; `src/main.lua` Run | 08/04-boss-league-table | games-night league table |
