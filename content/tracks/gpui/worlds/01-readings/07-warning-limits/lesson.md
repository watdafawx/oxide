Kinetik paints a value in the warning colour when it crosses a limit you set: CPU temperature at 85 °C, GPU usage at 95 %, ping at 100 ms. To know whether a pill turns red, compare its reading with its kind's limit.

**The limits.** `Limits` holds one number per kind that has a limit, with Kinetik's defaults. `limit_for(kind)` gives `Some(85.0)` for `CpuTemp`, and `None` for kinds with no limit, like `NetDown`.

**Two maybes.** A warning needs both a value and a limit, and each one is an `Option`. Put them side by side in a pair and match on the pair. One arm handles "both are there"; everything else is not a warning:

```rust
(Some(v), Some(limit)) => v >= limit,
```

**At counts.** Kinetik warns at the limit, not only above it: 85 °C with a limit of 85 is already red. That is `>=`, not `>`.

**Unknown is not alarming.** A temperature Kinetik could not read is `None`. It shows `--`, never red: the `_` arm covers it.

## Your turn

- In `is_warning`, replace `false` with a `match` on `(reading.value, limits.limit_for(reading.kind))`.
- Give it the `(Some(v), Some(limit))` arm and a `_ => false` arm.
- Save.
