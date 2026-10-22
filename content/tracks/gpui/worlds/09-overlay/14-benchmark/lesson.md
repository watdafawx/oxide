Kinetik's Ctrl+Shift+F11 "records every stat, each update, to a CSV file", for comparing graphics settings or drivers: run the same scene twice, compare the files. `Benchmark` keeps the rows; writing them to Documents is one `fs::write` at the end.

**On and off.** `toggle` starts a recording with a fresh header, or stops it. While stopped, ticks change nothing.

**One row per tick.** Each update while recording appends a row: the tick number, CPU and GPU usage, GPU temperature and FPS. A missing reading is an empty cell, as in the session summary.

**Two small closures.** `cell` writes an `Option<f64>` with one decimal or empty; `get` reads a stat from the snapshot. With them, the row is one `format!`:

```rust
let cell = |v: Option<f64>| v.map(|v| format!("{v:.1}")).unwrap_or_default();
```

**Same scene, twice.** A benchmark is only useful as a comparison: record, change one setting, record again, and compare the two files.

## Your turn

- In `Benchmark::tick`, return at once when not recording.
- Push a row of the tick number and the four cells, then add 1 to `ticks`.
- Save.
