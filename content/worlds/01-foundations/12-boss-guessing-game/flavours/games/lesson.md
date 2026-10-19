**Why now:** this is the finished calculator. Everything from this world goes into one small program: a **struct** (`Recipe`), an **enum**-style decision, **match**, **Option** (recipe lookup), **Result** (parsing a recipe line), and a **loop** that drives it all. Give it "8 torches" and it tells you the raw coal and logs to gather.

The calculator logic is separate from typing, so tests can drive it with a script. No terminal needed.

```rust
let mut calc = Calculator::new();
calc.add_recipe(parse_recipe("plank x4 = log x1").unwrap());
calc.batches_needed("plank", 5);     // 2 crafts: 4 planks is not enough
calc.raw_totals("plank", 5);         // {"log": 2}

parse_recipe("stick x4 = plank x2"); // Ok(Recipe { output: "stick", yields: 4, .. })
parse_recipe("pizza");               // Err("'pizza' is not a recipe")

run(&["plank x4 = log x1", "pizza", "need 5 plank", "quit"]);
// ["Learned plank", "'pizza' is not a recipe", "5 plank needs: log x2", "Bye! 1 recipe learned"]
```

- A recipe line is `output xN = ingredient xN, ingredient xN`. `parse_part` and `parse_need` are given.
- A **raw** item has no recipe. A **crafted** item needs `ceil(wanted / yields)` crafts, and each craft needs every ingredient (which may be crafted too: that is the tree).
- `raw_totals` is recursive: `expand` calls itself for each ingredient. `totals.entry(name).or_insert(0)` gives a counter you can add to.
- Bad lines print their error but change nothing.
- `quit` stops the run: "1 recipe learned", but "2 recipes learned".

## Your turn

- Open `src/lib.rs`. `Recipe`, `Calculator`, `parse_part` and `parse_need` are given.
- Do the objectives top to bottom: `new`, `find_recipe`, `batches_needed`, `expand`, `parse_recipe`, then `run`.
- The first objective is one line. Take the win, then keep going.
