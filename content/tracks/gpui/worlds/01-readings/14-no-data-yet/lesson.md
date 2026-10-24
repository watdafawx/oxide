Hover a stat and Kinetik writes its last minute under the graph: `min 4% · avg 13% · max 61%`. `History::summary` already gives the three numbers, or `None` while there are none. The hover card needs them as one line of text, or `no data yet`.

**Two ways out.** With `None`, the function is done at once. With `Some(s)`, the real work starts, and it needs `s` for every line after. A `match` would put all of that work inside one arm, a level deeper than the rest.

**The word for it.** `let .. else` unpacks a value or leaves. `let Some(s) = history.summary() else { .. };` makes `s` a plain variable for the rest of the function when the pattern fits. When it does not, the `else` block runs, and it must leave: `return` here, or `break` inside a loop. So no line below ever sees a missing value.

**You have seen one.** `text` in `reading.rs` has started this way since the formatting quests: no value, two dashes, done.

```rust
let Some(v) = self.value else {
    return "--".to_string();
};
```

**Each number in its unit.** `Reading::text` already writes a value in its kind's unit. So build a `Reading` for each number, `Reading { kind, value: Some(s.min) }.text()`, and CPU reads `10%` while RAM reads `8.0 GB`. A small closure that does this for one number saves writing it three times.

**For the curious.** `let .. else` arrived in Rust 1.65. Older code writes the same thing as `let s = match history.summary() { Some(s) => s, None => return .. };`.

## Your turn

- In `hover_summary`, unpack `history.summary()` with `let .. else`, and give back `no data yet` when there is none.
- Then write `min`, `avg` and `max` in the format of the doc comment, each number in the kind's own unit.
