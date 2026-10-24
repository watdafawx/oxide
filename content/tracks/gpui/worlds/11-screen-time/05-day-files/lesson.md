Kinetik keeps screen time in `%LocalAppData%\KinetikStats\ScreenTime`, one small JSON file per day. Nothing leaves the PC.

**Named by date.** A day's file is `2026-10-22.json`: year, month, day, so the files sort by date even as plain text, and a person can find a day by eye. chrono's `date.format(..)` writes a date with `%Y` for the year, `%m` the month and `%d` the day, and `dir.join(..)` puts the name inside the folder. A file per month would be:

```rust
dir.join(format!("{}.csv", date.format("%Y-%m"))) // ScreenTime\2026-10.csv
```

**Names only.** Window titles say what you read, who you messaged and which film you watched. Kinetik never even asks for them: `Desk` has no title method, and `Day` holds program names and seconds, nothing else. The second test checks the file for exactly that.

**The folder first.** On a new PC the folder does not exist yet, and `fs::write` would fail. `std::fs::create_dir_all(dir)` makes it, with any missing parents, and does nothing if it is already there.

**Then the same as Settings.** `serde_json::to_string_pretty(day)` turns the day into JSON text, and `std::fs::write` writes it, as in the Settings world's save quest. Plain `?` works on both: `to_string_pretty`'s error converts into an `io::Error` through `From`, so `save_day` can give back one `io::Result`.

**Saved every sample.** A day file is a few hundred bytes, and `sample` writes it after each counted sample. A crash or a power cut loses 5 seconds, not a day. `load_day` reads today's file back on start, so a restart carries on where it was.

## Your turn

- Write `day_path`: the date as year-month-day plus `.json`, inside `dir`.
- Write `save_day`: make the folder, turn the day into JSON, and write it to its `day_path`. Save.
