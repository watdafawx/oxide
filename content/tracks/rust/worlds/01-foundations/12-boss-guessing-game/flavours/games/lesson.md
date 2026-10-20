**Why now:** this is the finished calculator. Everything from this world goes into one small program: a **struct** (`Recipe`), **match**, **Option** (recipe lookup), **Result** (parsing a recipe line), and a **loop** that drives it all. Give it "8 torches" and it tells you the raw coal and logs to gather.

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
- `raw_totals` is **recursive**: a function may call itself. `expand(torch)` calls `expand(stick)` and `expand(coal)`, and an item with no recipe is where it stops (the `None` arm). `totals.entry(name).or_insert(0)` gives a counter you can add to.
- Bad lines print their error but change nothing.
- `quit` stops the run: "1 recipe learned", but "2 recipes learned".

## Toolbox you have not met yet

`run` and `raw_totals` use a few collection moves. Skim them now, copy the shape later.

```rust
// Vec: a growable list
let mut lines: Vec<String> = Vec::new();
lines.push("Learned stick".to_string());
for line in &lines { println!("{line}"); }      // `&` = look at each, don't take the list

// BTreeMap: name -> count, kept sorted by name
let mut totals: BTreeMap<String, u32> = BTreeMap::new();
*totals.entry("log".to_string()).or_insert(0) += 2;   // missing key starts at 0, then add 2
for (name, n) in &totals { println!("{name} x{n}"); } // yields (&String, &u32); `n * 2` works on a &u32

// Turn a map into one line of text
let parts: Vec<String> = totals.iter().map(|(name, n)| format!("{name} x{n}")).collect();
let text = parts.join(", ");                    // "log x2" (or "coal x2, log x1" with more keys)

// &mut: lend something so another function can change it
fn add_log(totals: &mut BTreeMap<String, u32>) { *totals.entry("log".to_string()).or_insert(0) += 1; }
add_log(&mut totals);                           // recursion passes `totals` down the same way
```

- `line.split(',')` walks the pieces of a string; `starts_with("need ")` tests a prefix.
- `.iter().map(...).collect()` builds a new list from an old one.

## Your turn

- Open `src/lib.rs`. `Recipe`, `Calculator`, `parse_part` and `parse_need` are given.
- Do the objectives top to bottom: `new`, `find_recipe`, `batches_needed`, `expand`, `parse_recipe`, then `run`.
- The first objective is one line. Take the win, then keep going.

Press Run to play it.
