**Why now:** the recipe calculator reads formulas like `2 * (iron + 3) ^ 2` (Factorio's own tech costs are written like that). Time to build the engine that turns text into a number, using everything from this world: tables, closures, patterns, coroutines and errors.

Three small machines in a row:

```
"2 * (x + 1)"  →  tokens  →  tree  →  number
                  scan       parse    evaluate
```

1. **Tokenizer** (`scan`): a generator (`coroutine.wrap`) that yields `{kind, text, pos, value}` one token at a time.
2. **Parser**: *recursive descent*, one function per grammar rule, lowest priority first:

```
expr   = term  { ("+" | "-") term }
term   = unary { ("*" | "/") unary }
unary  = "-" unary | power
power  = atom [ "^" unary ]
atom   = number | name | "(" expr ")"
```

   Each rule calls the next-tighter one, which is how `*` ends up binding harder than `+`. A node looks like `{type = "bin", op = "+", left = ..., right = ...}`.
3. **Evaluator**: a function that walks the tree: numbers return themselves, `bin` evaluates both sides and applies the operator.

Errors are part of the job: a bad input raises `"syntax error at position N: ..."` (positions count characters from 1), and `1 / 0` raises `"division by zero"`.

**Stages** (the list ticks off as you go): 1 tokens → 2 `+ - * / ^` → 3 parentheses and unary minus → 4 variables → 5 error messages → 6 zero check. Start with `scan` in `src/calc.lua`; the number branch is already there.

## Your turn

- Open `src/calc.lua`. The comments marked `TODO (stage N)` are your path.
- When the tests are green, open `src/main.lua` and press Run to see it work on real formulas.

Press Run to play it.
