Every sensor exists: CPU, RAM, network, disk, NVIDIA GPU, ACPI temperature, battery and ping. The bar reads them on a worker, knows which ones it wants, and each sensor can say why it has no number. This boss writes the three pieces that join it all up: reading, storing, and explaining.

**What is given.** The sensors and `live_sensors`, `Missing` and `missing_text`, `wanted_kinds`, `hover_summary`, and `bar.rs` with the struct, `new` and the hop in `start`. `start` calls your `read_sensors` on a worker thread and your `apply` on the UI thread. A `SensorRead` now has three parts: the kind, the value, and the reason.

**Read, with reasons.** `read_sensors` is the one from the Send quest, plus a reason. Lock the list, poison and all, as that version did; keep the wanted sensors with `filter`; turn each into a `(kind, value, reason)` with `map`; `collect`. Read first, then ask `missing()`: it describes the read that just happened. A value needs no reason, so the reason is `None` whenever the value is `Some`.

**Store them.** `apply` walks the reads. A value goes into the snapshot and its history, and clears any old reason: a ping that answers again is no longer timed out. A gap removes the stat from the snapshot, then stores its reason or removes an old one. Then count the tick and the compare-then-notify you wrote in Live. The shape, with the bodies left to you:

```rust
for (kind, value, missing) in reads {
    match value {
        Some(v) => { /* snapshot, history, forget the reason */ }
        None => { /* out of the snapshot; remember or forget the reason */ }
    }
}
```

**Two maybes.** Inside the `None` arm, `missing` is an `Option` too, so it gets its own `match`. `HashMap::insert` and `remove` both give back the old value, an `Option<Missing>`, so the two arms have the same type, and a `;` after the inner `match` throws that value away.

**Explain gaps.** `hover_text(kind)` is what the hover card will say: the reason in words if the stat is missing, else min, average and max from its history, else `no data yet`. `self.missing.get(&kind)` gives a `&Missing`; `*why` copies the value out, because `missing_text` takes a `Missing`.

## Your turn

- Write `read_sensors`: the first three objectives come from it.
- Write `apply`, then `hover_text`.
- Press Run: every number on the bar is your PC's own, or `--` with a reason.
