When a game closes, Kinetik shows a notification with how long you played, the average FPS, the 1% low and the peak GPU temperature, and appends it to `Documents\Kinetik Stats\Sessions.csv`. One line per session, readable by any spreadsheet. `csv_line` writes that line.

**Match the header.** `SESSIONS_HEADER` names five columns: game, minutes, FPS, 1% low, GPU temperature. Every line must have the same five, in the same order, or the spreadsheet shifts columns and puts FPS under minutes.

**Readable numbers.** Minutes with one decimal (2550 seconds is 42.5), FPS and the low with one decimal, temperature as a whole degree. `{:.1}` in `format!` rounds to one decimal and `{:.0}` to none, as in the Readings world; `seconds as f64 / 60.0` gives minutes as a decimal.

**Missing is empty.** A PC without a readable GPU temperature writes an empty cell, so a spreadsheet sees "no value" instead of a fake 0. `peak_gpu_temp` is an `Option<f64>`.

**Change what is inside.** `Option::map(|t| ..)` runs the closure on the value inside a `Some` and wraps the answer in `Some` again; a `None` stays `None` without running anything. Then `unwrap_or_default()` gives the value, or the type's default for `None`, which for a `String` is `""`. Here the same pair writes a ping cell, then a two-field line:

```rust
let ping = self.best_ping.map(|ms| format!("{ms:.0}")).unwrap_or_default();
format!("{},{:.1},{}", self.server, self.hours, ping)
```

**Appended, never rewritten.** Each session adds one line at the end of `Sessions.csv`, so years of sessions stay in one file you can chart.

## Your turn

- In `csv_line`, write the temperature cell: whole degrees, or empty for `None`.
- Give back the five fields with one `format!`, in the header's order.
- Save.
