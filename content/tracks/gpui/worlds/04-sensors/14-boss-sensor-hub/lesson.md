Every sensor exists: CPU, RAM, network, disk, NVIDIA GPU, ACPI temperature, battery and ping. The bar reads them in the background and knows which ones it wants. This boss writes the three pieces that join it all up: reading, storing, and explaining.

**What is given.** The sensors and `live_sensors`, `Missing` and `missing_text`, `wanted_kinds`, and `bar.rs` with the struct, `new` and the background `start`. `start` calls your `read_sensors` on a worker thread and your `apply` on the UI thread.

**Read, with reasons.** `read_sensors` locks the sensor list, keeps the wanted ones, and reads each. A value needs no explanation; a `None` asks the sensor `missing()` right after the read.

**Store them.** `apply` puts each value in the snapshot and its history, and each reason in `self.missing`. A value clears an old reason: a ping that answers again is no longer timed out. Then the compare-then-notify you wrote in Live:

```rust
Some(why) => self.missing.insert(kind, why),
```

**Explain gaps.** `hover_text(kind)` is what the hover card will say: the reason in words if the stat is missing, else min, average and max from the history.

## Your turn

- Write `read_sensors`: the first three objectives come from it.
- Write `apply` and `hover_text`.
- Press Run: every number on the bar is this PC's own, or `--` with a reason.
