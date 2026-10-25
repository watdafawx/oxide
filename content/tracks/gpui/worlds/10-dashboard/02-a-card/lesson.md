Each dashboard card shows the reading now, its average and its peak. The model from the Readings world already writes any number in its unit, and every stat has a history, so this quest only joins them up.

**Three numbers, one writer.** `Reading { kind, value }.text()` writes a value as `30%`, `70°C` or `8.0 GB`, and gives `--` for `None`. A closure, the unnamed function from the Rust track, lets all three numbers use it without spelling out the `Reading` three times. It captures `kind` from the code around it.

```rust
let as_temp = |value: Option<f64>| Reading { kind: StatKind::GpuTemp, value }.text();
as_temp(Some(70.0)); // "70°C"
as_temp(None);       // "--"
```

**Two maybes deep.** The summary hides behind two `Option`s. `bar.histories.get(&kind)` gives `Option<&History>`, because a stat that was never read has no history. `summary()` on a history gives `Option<Summary>`, because an empty history has no average.

**and_then, not map.** `map` runs a function on what is inside a `Some` and wraps the answer in a new `Some`. Here the function, `|h| h.summary()`, already gives an `Option`, so `map` would hand back `Option<Option<Summary>>`. `and_then` is for a next step that may itself come up empty: it gives a plain `Option<Summary>`.

**Then map.** From the `Option<Summary>`, `.map(|s| s.avg)` picks out the average and leaves a `None` as `None`. `Summary` is `Copy`, so you can map it twice: once for `avg`, once for `max`.

**No history, no guess.** A stat that has not been read yet shows `--` for its average and peak, not `0`. A `0` would claim the stat was measured and came out at nothing.

**The same words everywhere.** The bar, the hover card, the widget and now the dashboard all write numbers through `Reading::text`, so `30%` looks the same in every window.

## Your turn

- In `card_numbers`, write a closure that turns an `Option<f64>` into this kind's text.
- Get the summary of `kind`'s history, through both `Option`s.
- Give back `CardNumbers`: now from the snapshot, avg and peak from the summary. Save.
