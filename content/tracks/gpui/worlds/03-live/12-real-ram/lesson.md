`CpuSensor` moved into `real.rs`. RAM is next, and it is the same shape: refresh, then ask. Only the question changes.

**Same shape, other numbers.** `refresh_memory()` measures memory; `used_memory()` gives the bytes in use as a `u64`. Unlike CPU usage, memory needs no second sample: the first read is already right.

**Bytes, not GB.** `RamUsed` is in `Unit::Bytes`, and `format_bytes` writes 8589934592 as `8.0 GB`. So the sensor gives raw bytes and the formatting stays where it is:

```rust
Some(self.system.used_memory() as f64)
```

**Two real, six scripted.** `live_sensors` now uses real CPU and real RAM. GPU, network and ping stay scripted until the Sensors world.

**Read once, fresh.** Each sensor keeps its own `System`, so refreshing memory never refreshes CPU by accident, and each read costs only what it asks for.

## Your turn

- In `RamSensor::read`, refresh the memory, then give back `Some` of the used memory as `f64`.
- Save. Press Run: CPU and RAM are this PC's own.
