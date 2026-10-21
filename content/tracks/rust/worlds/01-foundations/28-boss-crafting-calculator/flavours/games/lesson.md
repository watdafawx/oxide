This is the finished calculator. Everything from this world goes into one program: a struct (`Recipe`), a match, an Option for lookups, a Result for reading recipe lines, and a loop that drives it all. Give it "8 torches" and it tells you how much coal and how many logs to gather.

**What it does.** You teach it recipes as lines of text, ask what a craft needs, and quit. The logic is separate from typing, so the tests drive it with a script and no terminal is needed:

```rust
parse_recipe("stick x4 = plank x2");  // Ok(Recipe { output: "stick", yields: 4, .. })
parse_recipe("pizza");                // Err("'pizza' is not a recipe")

run(&["plank x4 = log x1", "pizza", "need 5 plank", "quit"]);
// ["Learned plank", "'pizza' is not a recipe", "5 plank needs: log x2", "Bye! 1 recipe learned"]
```

**Raw and crafted.** A recipe line reads `output xN = ingredient xN, ingredient xN`. An item with no recipe is **raw**: you gather it. A crafted item needs enough crafts to make what you want, rounded up, and each craft needs every ingredient, which may be crafted too. That is the tree.

**A list that grows.** The calculator keeps its recipes in a `Vec`, a list that can grow. `Vec::new()` makes an empty one, and `.push(x)` adds to the end. The same goes for the messages `run` collects.

**Walk a list.** `for recipe in &self.recipes { ... }` visits each item in turn. The `&` means you look at them without taking the list apart.

**Leave early.** Inside a function, `return x;` hands back the answer right away, even from inside a loop. Inside a loop, `break` leaves just the loop. You will use `return` to give back a match, and `break` when the script says quit.

**A function that calls itself.** Working out `torch` means working out `stick`, which means `plank`, which means `log`. `expand` calls `expand` for each ingredient. It stops at raw items, whose arm does not call itself. This is called **recursion**.

**Take the first win.** Start from the top. The first objective is one line. Take it, then keep going; the TODO comments name each helper, and `parse_part`, `parse_need`, `add_raw` and `totals_line` are already written for you.

## Your turn

- Do the objectives top to bottom: `new`, `find_recipe`, `batches_needed`, `expand`, `parse_recipe`, then `run`.
- Save after each one and watch another objective turn green.
- Press Run to talk to your finished calculator.
