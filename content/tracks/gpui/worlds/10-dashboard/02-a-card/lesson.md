Each Kinetik dashboard card shows "the current reading, average, peak and a graph". The model from Readings already knows how to write any number in its unit, and every stat has a history.

**Three numbers, one writer.** `Reading { kind, value }.text()` writes a value as `30%`, `70°C` or `8.0 GB` and gives `--` for `None`. A small closure lets all three numbers use it:

```rust
let text = |value: Option<f64>| Reading { kind, value }.text();
```

**Average and peak.** `History::summary()` gives min, average and max, or `None` for an empty history. `and_then` gets the summary through the `Option` of the history; `map` picks `avg` or `max` out of the summary.

**No history, no guess.** A stat that has not been read yet has no history: the card says `--` for average and peak, not `0`.

**The same words everywhere.** The bar, the hover card, the widget and now the dashboard all write numbers through `Reading::text`, so `30%` looks the same in every window.

## Your turn

- In `card_numbers`, write the `text` closure and get the summary.
- Give back `CardNumbers` with now from the snapshot, avg and peak from the summary.
- Save.
