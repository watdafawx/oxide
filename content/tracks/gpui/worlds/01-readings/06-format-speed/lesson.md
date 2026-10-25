Kinetik shows network traffic as `↓ 3.4 MB/s` and `↑ 50.8 KB/s`. You already know how to turn 3565158 into `3.4 MB`. A speed is that same text with an arrow in front and `/s` behind.

**Reuse, don't copy.** You could copy the loop from `format_bytes` into a new function. Then a fix to one copy misses the other. Call the function you have instead, inside a `format!`. A battery line built from `format_percent` looks like this:

```rust
format!("BAT {}", format_percent(level))
```

**The arrow comes in.** `format_speed` gets the arrow as text, so the same function serves download and upload. `reading.rs` already passes `↓` for `NetDown` and `↑` for `NetUp`. A `{arrow}` in the text writes it.

**From f64 to u64.** Speeds arrive as `f64`, and `format_bytes` takes whole bytes, a `u64`. `as u64` turns one into the other and drops the fraction of a byte, which nobody misses.

**For the curious.** `as` from a float to an integer never panics: it cuts the fraction, a negative number becomes 0, and NaN becomes 0 too. A glitchy sensor reading of -3.0 bytes per second shows `0 B/s`, not a crash.

## Your turn

- In `format_speed`, replace `String::new()` with a `format!` that writes the arrow, a space, the speed as bytes text, then `/s`.
- Save. Press Run to see download and upload.
