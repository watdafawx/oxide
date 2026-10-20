**Why now:** a modpack ships with a mod list, and everyone wants answers from it: how many downloads per loader, which mods are the heaviest, what is the smallest Fabric mod, and a clean report to paste in a README. The boss is that tool. You get the list as CSV text and turn it into answers, using everything from this world.

```rust
let csv = "name,loader,downloads,size_mb
sodium,fabric,9000,1.2";

let fields: Vec<&str> = "sodium, fabric".split(',').map(str::trim).collect();

fn half(text: &str) -> Result<u32, String> {
    let n: u32 = text.parse().map_err(|_| format!("bad number '{text}'"))?;
    Ok(n / 2)          // `?` above returned early if parse failed
}

// a list of Results collects into ONE Result: first Err wins
let all: Result<Vec<u32>, String> = ["2", "4"].iter().map(|t| half(t)).collect();
```

- **`?`** after a `Result` means: on `Ok(v)` give me `v`, on `Err(e)` return that error from this function.
- **`map_err`** rewrites the error, e.g. to add a line number.
- Collecting `Result` items into `Result<Vec<_>, _>` stops at the first `Err`.
- `BTreeMap` keeps loaders sorted. Compare `f64` with `total_cmp`. `sort_by_key` with `std::cmp::Reverse(..)` sorts biggest first.
- `csv.lines()` walks a text one line at a time, like `split_whitespace()` does for words.
- `min_by(|a, b| a.x.total_cmp(&b.x))` picks the smallest by a float field and returns an `Option`; `max_by` is the biggest.
- `lines.extend(iter)` pushes every item of an iterator onto a `Vec`; `lines.push(x)` adds one.

## Your turn

- Open `src/lib.rs`. `Mod` is given. The file lists eight functions with doc comments and examples.
- Go top to bottom: the first test is a one-liner, and each later one builds on earlier ones.
- Use iterator chains, not index loops. The doc comments say the exact error and report formats.

Press Run to play it.
