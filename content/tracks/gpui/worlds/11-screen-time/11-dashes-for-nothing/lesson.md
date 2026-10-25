The boss draws the screen time page, and its summary lines show text. But most of a `DaySummary` is `Option`s: no first use before the first sample, no most used program on an empty day. Each one becomes text, or `--` when there is nothing. This read is the three small tools that do it.

**A fallback that waits.** `unwrap_or_else(|| "--".into())` gives the value inside a `Some`, and runs its closure only for a `None`. `unwrap_or("--".into())` would build the `"--"` string every time, even when it is thrown away. For a short constant both work; the closure form is the habit for fallbacks that cost something.

**One closure for every line.** The page needs that fallback five times, so it keeps it in a closure of its own: `let or_dashes = |text: Option<String>| text.unwrap_or_else(|| "--".into());`. Then each line is `or_dashes(..)` around its `Option<String>`.

**A function where a closure goes.** `summary.first` is an `Option<u32>` of seconds after midnight, and `clock` turns seconds into `08:30`. `summary.first.map(clock)` passes the function itself: `map` takes any function of the right shape, a named one as well as a closure. It is `.map(|second| clock(second))` without the extra words.

```rust
let shown = summary.last.map(clock).unwrap_or_else(|| "--".into()); // "22:15", or "--"
```

**Two digits.** `format!("{hour:02}:00")` pads the hour with zeros to two digits, so hour 8 shows as `08:00` and lines up under `21:00`. `clock` uses the same `{:02}` for its hours and minutes.

**Named lines.** `summary_line(name, label, value)` draws one grey label and its value, named with a `debug_selector` so the tests can find it: `total`, `first`, `last`, `most-used` and `busiest`.

## Your turn

Answer the questions, then start the boss.
