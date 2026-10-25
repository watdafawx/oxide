Under the hover graph Kinetik prints the last minute's minimum, average and maximum: `min 4% · avg 13% · max 61%`. All three come from one walk over the history.

**Start values.** Before the first reading, `min` has to be bigger than anything real, so the first reading always replaces it: `f64::MAX`. `max` starts at `f64::MIN`, the most negative number, and `total` at `0.0`.

**Keep the best so far.** `a.max(b)` is the bigger of two numbers and `a.min(b)` the smaller. Walk the readings and keep the best one so far. Here is the loop for the highest value alone:

```rust
let mut highest = f64::MIN;
for v in self.values() {
    highest = highest.max(v);
}
```

**One loop, three numbers.** The summary needs the smallest, the biggest and the sum in the same walk, so the loop changes three variables, and all three need `mut`.

**The average.** After the loop, the average is `total / self.len() as f64`: the length is a count, a `usize`, and `as f64` makes it a number you can divide by.

**Empty is a trap.** Run the starter's tests: an empty history comes back with min still `f64::MAX`, max still `f64::MIN` and avg `NaN`, "not a number", because the loop never ran and 0.0 / 0 has no answer. With no readings there is nothing to summarise, so check for that first and give back `None`. The hover card then says "no data yet".

**For the curious.** Why not `self.values().max()`? Iterator `max` needs `Ord`, and `f64` is only `PartialOrd`, because NaN is not smaller or bigger than anything (the Traits world). `fold(f64::MIN, f64::max)` would work, and is this same loop in one line.

## Your turn

- In `summary`, give back `None` at the start when the history is empty.
- Then walk `self.values()` once, keeping `min`, `max` and `total` up to date, and build the `Summary` from them.
- Save.
