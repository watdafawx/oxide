Under the hover graph Kinetik prints the last minute's minimum, average and maximum: `min 4% · avg 13% · max 61%`. All three come from one walk over the history.

**Start values.** Before the first reading, `min` has to be bigger than anything real, so the first reading always replaces it: `f64::MAX`. `max` starts at `f64::MIN`, the most negative number, and `total` at `0.0`.

**One loop, three updates.** For each reading, `min.min(v)` is the smaller of the two, `max.max(v)` the bigger, and `total` adds up:

```rust
for v in self.values() { min = min.min(v); max = max.max(v); total += v; }
```

**The average.** After the loop, the average is `total / self.len() as f64`: the length is a count, and `as f64` makes it a number you can divide by.

**Empty is None.** With no readings there is nothing to summarise, and the first line already gives back `None`. The hover card then says "no data yet" instead of `min 0%`.

## Your turn

- In `summary`, replace the TODO and `None` with the loop over `self.values()`.
- After the loop, give back `Some(Summary { min, avg: total / self.len() as f64, max })`.
- Save.
