Windows reports CPU usage as numbers like 12.638. A bar that shows `12.638%` jumps around every second and is hard to read; Kinetik shows `13%`. The number has to be rounded when it is written, not when it is measured.

**Try the obvious thing.** `format!("{value}%")` writes every decimal the number has. Run the starter and you get `12.6%` and `12.4%`.

**Ask for no decimals.** Inside the braces, `:.0` means "zero digits after the point". Rust rounds to the nearest whole number while writing, and `value` itself stays 12.638:

```rust
format!("{value:.0}%")
```

**Already in use.** Look at `reading.rs`: degrees and milliseconds use the same `:.0`. Its `text` now picks a formatter by unit, and calls your `format_percent` for every percentage on the bar.

## Your turn

- In `format_percent`, add `:.0` after `value` inside the braces.
- Save. Press Run to see CPU, GPU and battery written as whole percentages.
