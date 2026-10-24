`live_sensors` now reads ten sensors every second, including a ping that can take a second and a WMI query that starts a connection to WMI each time. Kinetik's README is clear: sensors "only run while their stats are switched on". A stat that is not on the bar should cost nothing at all.

**Which stats are shown.** `wanted_kinds(&self.order)` collects every `Slot::Stat` of the bar's order into a `HashSet<StatKind>`. A set holds each value at most once and answers "is this one in?" quickly, however big it gets, with `contains`.

**Build it once.** The order does not change in the middle of one tick, so build the set before the loop, not once per sensor.

**Skip the rest.** Inside a loop, `continue` jumps straight to the next round and skips the rest of this one. Put it first in the loop body, before the `read`, and a skipped sensor is never read. The same move, skipping dividers while counting stats:

```rust
for slot in &order {
    if let Slot::Divider(_) = slot { continue; }
    count += 1;
}
```

**Not even one read.** For NVML, WMI or ping that is real work saved, every second, for as long as Kinetik runs. Add GPU temperature back to the order and the next tick reads it, with no restart. The Settings world builds the switch.

## Your turn

- In `tick` in `bar.rs`, before the loop, build the set of wanted kinds from `self.order`.
- In the loop, after `let kind = sensor.kind();`, skip to the next sensor when the set does not contain `kind`.
- Save.
