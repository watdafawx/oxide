Windows reports CPU usage as numbers like 12.638. A bar that shows `12.638%` jumps around every second and is hard to read; Kinetik shows `13%`. The number has to be rounded when it is written, not when it is measured.

**Try the obvious thing.** `format!("{value}%")` writes every decimal the number has. Run the starter and you get `12.6%` and `12.4%`.

**Ask for fewer digits.** Inside the braces, a colon, a dot and a number set how many digits come after the point. Rust rounds to that many while writing, and `value` itself stays 12.638. One digit gives `12.6%`:

```rust
format!("{value:.1}%")
```

**Measure exact, show round.** The history keeps 12.638, so the hover card's average stays exact. Only the text on the bar is rounded.

**Already in use.** Look at `reading.rs`: degrees and milliseconds are written whole the same way. Its `text` now picks a formatter by unit, and calls your `format_percent` for every percentage on the bar.

**For the curious.** An exact half rounds to the even neighbour: `{:.0}` writes 12.5 as `12` and 13.5 as `14`. That keeps a long run of halves from drifting upward. Most readings are never an exact half, so the bar rarely shows it.

## Your turn

- In `format_percent`, make the `format!` write the value with no digits after the point.
- Save. Press Run to see CPU, GPU and battery written as whole percentages.
