Text is a list too: lines, words, characters. Chains work on all of it, and that's how you'll read the CSV in the boss quest.

```rust
let line = " 3, 4 ,five, 6 ";

let nums: Vec<i32> = line
    .split(',')                          // " 3", " 4 ", "five", " 6 "
    .filter_map(|s| s.trim().parse().ok())   // 3, 4, 6 (bad ones dropped)
    .collect();

let letters = "héllo".chars().count();   // 5: chars, not bytes
let words = vec!["red", "green"];
let joined = words.join("+");            // "red+green"

println!("[{:<6}]", "ab");   // [ab    ]  left, width 6
println!("[{:>6}]", "ab");   // [    ab]  right
println!("[{:>6.2}]", 3.14159);   // [  3.14]  width 6, 2 decimals
```

- **`split(sep)`**, **`trim()`**, **`split_whitespace()`** cut text up. They return iterators.
- **`parse::<T>()`** returns a `Result`. **`.ok()`** converts it to `Option`; **`filter_map`** keeps the `Some` values and unwraps them.
- **`chars()`** iterates characters. **`join(sep)`** glues a list of strings with a separator.
- In `format!`, `{:<10}` pads left-aligned to width 10, `{:>5}` right-aligned. Perfect for tables.

## Your turn

- Open `src/lib.rs`.
- Fill in `parse_numbers`, `count_vowels`, `join_sorted` and `table_row`.
