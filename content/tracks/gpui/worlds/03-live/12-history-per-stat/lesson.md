Hover a stat and Kinetik shows its last minute. You built `History` in Readings; now the bar needs one per stat, filled on every tick, so the hover card has data the moment you point at it.

**A map of histories.** `histories` is a `HashMap<StatKind, History>`. CPU usage gets its own history, GPU temperature its own, and so on.

**Made on demand.** The first reading of a stat has no history yet. `entry(kind)` finds the slot for `kind`; `or_insert_with(|| History::new(60))` fills it with a new 60-reading history only if it is empty, and hands back a `&mut History` either way. The same shape, for a list of the ticks at which each stat went missing:

```rust
self.missed.entry(kind).or_insert_with(|| Vec::new()).push(self.ticks);
```

**Only real readings.** A sensor that gives `None` adds nothing. A gap in the graph is honest; a fake zero would draw a dip that never happened.

**Not drawn yet.** No graph shows this world. The Interaction world draws the hover card from these histories.

## Your turn

- In `tick`, in the `Some(v)` arm, push `v` into the history for `kind`, making a 60-reading `History` first if it has none.
- Save.
