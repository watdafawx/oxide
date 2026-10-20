Boss time. You get a CSV of products as text and turn it into answers: totals per category, the biggest line items, a printed report. Everything uses what you just learned.

```rust
let csv = "name,category,price,qty\npen,office,1.50,10";

let fields: Vec<&str> = "pen, office".split(',').map(str::trim).collect();

fn half(text: &str) -> Result<i32, String> {
    let n: i32 = text.parse().map_err(|_| format!("bad number '{text}'"))?;
    Ok(n / 2)          // `?` above returned early if parse failed
}

// a list of Results collects into ONE Result: first Err wins
let all: Result<Vec<i32>, String> = ["2", "4"].iter().map(|t| half(t)).collect();
```

- **`?`** after a `Result` means: on `Ok(v)` give me `v`, on `Err(e)` return that error from this function.
- **`map_err`** rewrites the error, e.g. to add a line number.
- Collecting `Result` items into `Result<Vec<_>, _>` stops at the first `Err`.
- `BTreeMap` keeps categories sorted. Compare `f64` with `total_cmp`.
- `csv.lines()` walks a text one line at a time, like `split_whitespace()` does for words.
- `min_by(|a, b| a.x.total_cmp(&b.x))` picks the smallest by a float field and returns an `Option`; `max_by` is the biggest.
- `lines.extend(iter)` pushes every item of an iterator onto a `Vec`; `lines.push(x)` adds one.
- `sort_by(|a, b| b.x.total_cmp(&a.x))` sorts floats biggest first (swap `a` and `b` for smallest first).

## Your turn

- Open `src/lib.rs`. `Row` is given. The file lists eight functions with doc comments and examples.
- Go top to bottom: the first test is a one-liner, and each later one builds on earlier ones.
- Use iterator chains, not index loops. The doc comments say the exact error and report formats.

Press Run to play it.
