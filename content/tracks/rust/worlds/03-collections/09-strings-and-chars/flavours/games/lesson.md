**Why now:** every item in the game is text: `minecraft:diamond_sword`. Your plugin has to split off the namespace, make a nice display name, read stack counts out of a chat command, and print a tidy inventory table. It is also exactly the skill the boss needs to read a mod list.

```rust
let line = " 64, 16 ,x, 8 ";

let counts: Vec<u32> = line
    .split(',')                              // " 64", " 16 ", "x", " 8 "
    .filter_map(|s| s.trim().parse().ok())   // 64, 16, 8 (bad ones dropped)
    .collect();

let id = "minecraft:oak_log";
let name = id.rsplit(':').next().unwrap();   // "oak_log"
let letters = name.chars().count();          // 7: chars, not bytes
let items = vec!["bow", "arrow"];
let joined = items.join("+");                // "bow+arrow"

let mut c = "log".chars();
let first = c.next();                        // Some('l'): chars() is an iterator
let rest: String = c.collect();              // "og"

println!("[{:<6}]", "ab");   // [ab    ]  left, width 6
println!("[{:>6}]", "ab");   // [    ab]  right
```

- **`split(sep)`**, **`trim()`**, **`split_whitespace()`** cut text up. They return iterators. `rsplit` starts from the right.
- **`parse::<T>()`** returns a `Result`. **`.ok()`** converts it to `Option`; **`filter_map`** keeps the `Some` values and unwraps them.
- **`chars()`** iterates characters, so you can take the first one and keep the rest. **`join(sep)`** glues a list of strings with a separator.
- In `format!`, `{:<12}` pads left-aligned to width 12, `{:>4}` right-aligned. Perfect for tables.

- **`c.to_uppercase()`** on a `char` returns an **iterator** (some letters become two), not a `char` or a `String`. Use `.collect::<String>()` to get a `String`, `.next()` for the first `char`, or `.chain(rest)` to glue it to the remaining chars first.

## Your turn

- Open `src/lib.rs`.
- Fill in `parse_counts`, `display_name`, `chest_summary` and `inventory_row`.
