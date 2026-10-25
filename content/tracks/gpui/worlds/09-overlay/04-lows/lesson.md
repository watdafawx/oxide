The capture averages 118 FPS, yet it has five hitches you would feel. Kinetik shows the 1% and 0.1% lows next to FPS for exactly that reason: they are the speed of the worst moments, which an average smooths away.

**The slowest share.** Sort the frame times slowest first and keep the slowest 1 %: 12 of 1200 frames. Average them and turn that average into FPS with `1000 / average`. The 0.1 % low uses a share of 0.001, here a single frame.

**Sort a copy.** `ms` is borrowed, a `&[f64]`, so you cannot reorder it in place. `ms.to_vec()` makes your own `Vec` copy, and `let mut` lets you sort it.

**Sorting floats.** `f64` has no plain `sort()`: `NaN` compares as neither smaller nor bigger than anything, so there is no single right order. `total_cmp` gives every float a place, `NaN` included. `sort_by(|a, b| ..)` takes a closure that compares two items, as in the Rust track's [Decimals compare differently](/quest/rust/03-collections/24-decimals-compare-differently). Here it sorts fastest first; swap `a` and `b` to sort slowest first:

```rust
let mut fastest = ms.to_vec();
fastest.sort_by(|a, b| a.total_cmp(b));
```

**How many, at least one.** `ms.len() as f64 * share` is how many frames the share is, as a decimal; `.round() as usize` makes it a whole count. On a short run, 0.001 of the frames rounds to 0. `.max(1)` keeps at least one, so a 0.1 % low always exists.

**The first n.** `slowest[..n]` is a **slice** of the first `n` items: read `..n` as "from the start up to, not including, n". `.iter().sum::<f64>()` adds them up, and dividing by `n as f64` gives the average.

**Nothing yet.** With no frames at all there is nothing to average; give back `0.0` at once.

## Your turn

- In `low_fps`, give back `0.0` for no frames. Otherwise copy the frame times and sort them slowest first.
- Work out `n`, the share of the count, rounded, at least 1. Average the first `n` and give back `1000.0 / average`.
- Save.
