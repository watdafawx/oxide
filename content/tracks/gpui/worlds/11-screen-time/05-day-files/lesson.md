Kinetik keeps screen time in `%LocalAppData%\KinetikStats\ScreenTime`, one JSON file per day. Nothing leaves the PC.

**Named by date.** `2026-10-22.json`: year, month, day, so the files sort by date as plain text. chrono's `format` writes it:

```rust
dir.join(format!("{}.json", date.format("%Y-%m-%d")))
```

**Names only.** Window titles say what you read, who you messaged and which film you watched. Kinetik never even asks for them: `Desk` has no title method, and `Day` holds program names and seconds, nothing else.

**Saved every sample.** A day file is a few hundred bytes, and `sample` writes it after each counted sample. A crash or a power cut loses 5 seconds, not a day.

**The folder first.** On a new PC the folder does not exist yet, and `fs::write` would fail. `create_dir_all` makes it, and does nothing if it is already there.

**Back on start.** `load_day` reads today's file, so restarting Kinetik carries on from where it was.

## Your turn

- Write `day_path`: the date as `%Y-%m-%d` plus `.json`, inside `dir`.
- Write `save_day`: make the folder, `serde_json::to_string_pretty`, write the file.
- Save.
