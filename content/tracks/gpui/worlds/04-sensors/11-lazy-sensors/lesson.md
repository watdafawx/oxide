`live_sensors` now reads ten sensors every second, including a ping that can take a second and a WMI query that starts COM each time. Kinetik's README is clear: sensors "only run while their stats are switched on". A stat that is not on the bar should cost nothing at all.

**Which stats are shown.** `wanted_kinds(&self.order)` collects every `Slot::Stat` of the bar's order into a `HashSet<StatKind>`. A set answers "is this one in?" quickly, whatever its size.

**Skip the rest.** In `tick`, build the set once, then skip every sensor whose kind is not in it. `continue` jumps straight to the next sensor:

```rust
if !wanted.contains(&kind) { continue; }
```

**Not even one read.** A skipped sensor's `read` is never called. For NVML, WMI or ping that is real work saved, every second, for as long as Kinetik runs.

**Turned on later.** Add GPU temperature back to the order and the next tick reads it, with no restart. The Settings world builds the switch.

## Your turn

- In `tick` in `bar.rs`, before the loop, build `wanted` with `wanted_kinds(&self.order)`.
- In the loop, after `let kind = sensor.kind();`, `continue` when `wanted` does not contain `kind`.
- Save.
