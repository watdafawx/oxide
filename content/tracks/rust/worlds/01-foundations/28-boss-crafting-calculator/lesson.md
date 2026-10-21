This is the finished planner. Everything from this world goes into one program: a struct (`Kit`), a match, an Option for lookups, a Result for reading kit lines, and a loop that drives it all. Give it "8 bundles" and it tells you how many labels and crates to order.

**What it does.** You teach it kits as lines of text, ask what a batch needs, and quit. The logic is separate from typing, so the tests drive it with a script and no terminal is needed:

```rust
parse_kit("box x4 = sheet x2");  // Ok(Kit { output: "box", yields: 4, .. })
parse_kit("pizza");                // Err("'pizza' is not a kit")

run(&["sheet x4 = crate x1", "pizza", "need 5 sheet", "quit"]);
// ["Learned sheet", "'pizza' is not a kit", "5 sheet needs: crate x2", "Bye! 1 kit learned"]
```

**Raw and packed.** A kit line reads `output xN = ingredient xN, ingredient xN`. An item with no kit is **raw**: you gather it. A packed item needs enough packs to make what you want, rounded up, and each pack needs every ingredient, which may be packed too. That is the tree.

**A list that grows.** The planner keeps its kits in a `Vec`, a list that can grow. `Vec::new()` makes an empty one, and `.push(x)` adds to the end. The same goes for the messages `run` collects.

**Walk a list.** `for kit in &self.kits { ... }` visits each item in turn. The `&` means you look at them without taking the list apart.

**Leave early.** Inside a function, `return x;` hands back the answer right away, even from inside a loop. Inside a loop, `break` leaves just the loop. You will use `return` to give back a match, and `break` when the script says quit.

**A function that calls itself.** Working out `bundle` means working out `box`, which means `sheet`, which means `crate`. `expand` calls `expand` for each ingredient. It stops at raw items, whose arm does not call itself. This is called **recursion**.

**Take the first win.** Start from the top. The first objective is one line. Take it, then keep going; the TODO comments name each helper, and `parse_part`, `parse_need`, `add_raw` and `totals_line` are already written for you.

## Your turn

- Do the objectives top to bottom: `new`, `find_kit`, `batches_needed`, `expand`, `parse_kit`, then `run`.
- Save after each one and watch another objective turn green.
- Press Run to talk to your finished planner.
