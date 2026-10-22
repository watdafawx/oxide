Kinetik shows network traffic as `↓ 3.4 MB/s` and `↑ 50.8 KB/s`. You already know how to turn 3565158 into `3.4 MB`. A speed is that same text with an arrow in front and `/s` behind.

**Reuse, don't copy.** You could copy the loop from `format_bytes` into a new function. Then a fix to one copy misses the other. Call the function you have instead: `format_bytes(3565158)` is `3.4 MB`.

**The arrow comes in.** `format_speed` gets the arrow as text, so the same function serves download and upload. `reading.rs` already passes `↓` for `NetDown` and `↑` for `NetUp`.

**From f64 to u64.** Speeds arrive as `f64`, and `format_bytes` takes whole bytes, a `u64`. `as u64` drops the fraction of a byte, which nobody misses:

```rust
format!("{arrow} {}/s", format_bytes(bytes_per_sec as u64))
```

**One fix, both places.** If Kinetik ever writes `MiB` instead of `MB`, that change happens once in `format_bytes`, and RAM and network both follow.

## Your turn

- In `format_speed`, replace `String::new()` with the `format!` line.
- Save. Press Run to see download and upload.
