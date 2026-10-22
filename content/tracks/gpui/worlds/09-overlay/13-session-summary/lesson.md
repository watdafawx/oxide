When a game closes, Kinetik shows a notification with how long you played, the average FPS, the 1% low and the peak GPU temperature, and appends it to `Documents\Kinetik Stats\Sessions.csv`. One line per session, readable by any spreadsheet.

**Match the header.** `SESSIONS_HEADER` names five columns. Every line must have the same five, in the same order, or the spreadsheet shifts columns.

**Readable numbers.** Minutes with one decimal (2550 seconds is 42.5), FPS with one decimal, temperature as a whole degree:

```rust
format!("{},{:.1},{:.1},{:.1},{}", self.game, self.seconds as f64 / 60.0, self.average_fps, self.one_percent_low, temp)
```

**Missing is empty.** A PC without a readable GPU temperature writes an empty cell: `map` the `Option` to text, then `unwrap_or_default()` gives `""` for `None`.

**Appended, never rewritten.** Each session adds one line at the end of `Sessions.csv`, so years of sessions stay in one file you can chart.

## Your turn

- In `csv_line`, write the temperature cell (empty for `None`).
- Give back the five fields with `format!`, in the header's order.
- Save.
