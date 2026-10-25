Before any GUI library: the task manager has to decide what a row says. Windows reports a process as numbers (12.46 percent, 536,870,912 bytes), and nobody wants to read those. A plain function that turns numbers into the row's text works with every library you will try.

**The process.** `Process` holds what Windows tells you: `pid`, `name`, `cpu` as an `f32` percent, and `memory_bytes` as a `u64`. Later worlds fill it from the real PC; today the tests make one up.

**The row.** `ProcessRow` holds what the screen shows: three `String`s, `name`, `cpu` and `memory`. A struct like this, the screen's words with no drawing code, is a **view model**: the model of one view.

**One decimal.** `{:.1}` inside `format!` prints a float with one digit after the point, rounded: `12.46` prints as `12.5`. Text after the `}` prints as it is, so a unit or a sign can follow the number.

```rust
format!("{:.1} GHz", 3.67_f32) // "3.7 GHz"
```

**Whole megabytes.** A megabyte here is 1,048,576 bytes (1024 times 1024). Dividing two `u64` values drops the remainder, so 536,870,912 bytes becomes `512`. Write `1_048_576` with underscores; Rust ignores them, you can read it.

**Why it matters.** egui and GPUI will both draw a `ProcessRow`. If the words live here, switching libraries means rewriting only the drawing, not the rules.

## Your turn

1. In `ProcessRow::from_process`, set `cpu` with `format!` and `{:.1}`, then a `%`.
2. Set `memory` to the whole megabytes followed by ` MB`.
3. Save.
