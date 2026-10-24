Kinetik paints a value in the warning colour when it crosses a limit you set: CPU temperature at 85 °C, GPU usage at 95 %, ping at 100 ms. To know whether a pill turns red, compare its reading with its kind's limit.

**The limits.** `Limits` holds one number per kind that has a limit, with Kinetik's defaults. `limit_for(kind)` gives `Some(85.0)` for `CpuTemp`, and `None` for kinds with no limit, like `NetDown`.

**Two maybes.** A warning needs both a value and a limit, and each one is an `Option`. Put them side by side in a tuple, `(a, b)`, and `match` on the pair. One arm handles "both are there"; `_` catches every other mix. RAM as a percentage, which needs both the used and the total amount, has the same shape:

```rust
match (used, total) {
    (Some(u), Some(t)) => Some(u / t * 100.0),
    _ => None,
}
```

**At counts.** Kinetik warns at the limit, not only above it: 85 °C with a limit of 85 is already red. That is `>=`, "at or above", not `>`.

**Unknown is not alarming.** A temperature Kinetik could not read is `None`. It shows `--`, never red. A kind with no limit, like download speed, is never red either.

## Your turn

- In `is_warning`, replace `false` with a `match` on the pair of the reading's value and its kind's limit.
- Both there: warn when the value has reached the limit. Anything else: no warning.
- Save.
