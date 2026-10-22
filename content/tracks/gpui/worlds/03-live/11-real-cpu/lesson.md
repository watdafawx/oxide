The bar ticks on scripted numbers. Time for the first real one: this PC's CPU usage. Windows reports it through performance counters, and the `sysinfo` crate reads them for you on Windows, Linux and macOS alike.

**A new dependency.** `Cargo.toml` now lists `sysinfo` with only its `system` feature, which keeps the build small. `System::new()` makes an empty reader; nothing is measured until you refresh.

**Refresh, then ask.** `refresh_cpu_usage()` takes a fresh measurement. `global_cpu_usage()` gives the usage across all cores as an `f32` percentage. The bar wants `f64`, so cast it:

```rust
Some(self.system.global_cpu_usage() as f64)
```

**First read: 0.** CPU usage is "busy time since the last refresh", so the first refresh has nothing to compare with. On the bar this is one tick of `0%`; the test waits sysinfo's minimum interval and reads twice.

**A live test.** This test runs on your real PC, so it only checks the range: any number from 0 to 100 passes.

## Your turn

- In `CpuSensor::read` (in `lib.rs`), refresh the CPU usage, then give back `Some` of the global usage as `f64`.
- Save. Press Run: CPU now shows this PC's real usage; the rest is still scripted.
