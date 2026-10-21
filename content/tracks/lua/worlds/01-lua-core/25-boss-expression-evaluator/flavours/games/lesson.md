Your recipe calculator can now count, loop, store and complain. One feature is left: reading a formula like `2 * (plates + 3)` typed by a person and turning it into a number. Factorio writes its own technology costs this way, so this boss is the real thing. It uses almost everything from this world.

**Three small machines.** The text goes through three machines in a row: a **tokenizer** cuts it into pieces, a **parser** arranges the pieces into a tree, and an **evaluator** walks the tree and gives the number.

**Machine one: tokens.** The tokenizer is a generator, built with `coroutine.wrap`. Each `yield` hands out one token, a table like `{kind = "num", text = "3.5", value = 3.5, pos = 6}`. The starter already reads numbers. You add the operators, brackets and names.

**Machine two: the tree.** The parser has one function per rule, loosest rule first, and each calls the next-tighter one. That is how `*` ends up binding harder than `+`.

```lua
-- one function per rule, loosest first; each calls the next-tighter one
-- expr  = term  { ("+" | "-") term }
-- term  = unary { ("*" | "/") unary }
-- unary = "-" unary | power
-- power = atom [ "^" unary ]
-- atom  = number | name | "(" expr ")"
-- a node looks like {type = "bin", op = "+", left = ..., right = ...}
```

**Machine three: walking the tree.** The evaluator is recursive. A number node returns its value. A `bin` node evaluates its left and right sides, then applies the operator. A `neg` node negates what is below it.

**Errors are part of the job.** Bad input raises `"syntax error at position N: ..."` with `error(message, 0)`, and positions count characters from 1. `1 / 0` raises `"division by zero"` instead of giving `inf`.

**Six stages.** The objective list ticks off as you go: tokens, then `+ - * / ^`, then brackets and unary minus, then variables, then error messages, then the zero check. Start with `scan` in `src/calc.lua`. The number branch is already there, and the `TODO (stage N)` comments are your path.

## Your turn

- In `src/calc.lua`, follow the `TODO (stage N)` comments from stage 1 upwards.
- Save after each stage and watch the next objective turn green.
- When all are green, press Run to try the engine on real formulas.
