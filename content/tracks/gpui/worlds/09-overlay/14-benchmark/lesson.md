Kinetik's Ctrl+Shift+F11 "records every stat, each update, to a CSV file", for comparing graphics settings or drivers: run the same scene twice, compare the files. `Benchmark` keeps the rows; writing them to Documents is one `fs::write` at the end.

**On and off.** `toggle` starts a recording with a fresh header, or stops it. While stopped, ticks change nothing.

**One row per tick.** Each update while recording appends a row: the tick number, CPU and GPU usage, GPU temperature and FPS. A missing reading is an empty cell, as in the session summary.

**Two small closures.** The row needs the same two steps four times: read a stat from the snapshot, and write an `Option<f64>` as a cell. A closure kept in a `let` is a small helper that lives only inside this method: `get` reads a stat with `snapshot.get(&kind).copied()`, and `cell` writes a value with one decimal, or empty for `None`, the way the session summary did. Here two such helpers make a status line:

```rust
let pct = |v: f64| format!("{v:.0}%");
let line = format!("CPU {} GPU {}", pct(cpu), pct(gpu));
```

**Same scene, twice.** A benchmark is only useful as a comparison: record, change one setting, record again, and compare the two files.

## Your turn

- In `Benchmark::tick`, return at once when not recording.
- Push a row of the tick number and the four cells, then add 1 to `ticks`.
- Save.
