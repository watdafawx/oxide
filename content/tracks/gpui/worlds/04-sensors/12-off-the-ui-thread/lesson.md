Run the previous quest on a slow network and the bar stutters: every second, the UI thread waits up to a second for ping. Nothing can draw or answer the mouse meanwhile. Sensor reads belong on a background thread, and the UI thread only stores the results.

**Already rewired.** `bar.rs` now keeps the sensors in an `Arc<Mutex<..>>` and, every interval, hands them to `cx.background_executor().spawn(..)`. That runs `read_sensors` on a worker thread. The result comes back to the UI thread, where `apply` stores it and redraws.

**The compiler says no.** It does not build. The error points at the background `spawn` and names the culprit, `dyn Sensor`:

```text
error: future cannot be sent between threads safely
  = help: the trait `std::marker::Send` is not implemented for `dyn Sensor`
```

**The word for it.** A type is **Send** when it is safe to move to another thread. The compiler cannot know that every future sensor will be, so it refuses. `CpuSensor`, `NvidiaGpu`, the ping: all of them are Send already; nobody promised it for the trait.

**Promise it once.** `pub trait Sensor: Send` makes Send part of being a sensor. Every sensor must now be Send, the compiler checks each one, and the background read builds.

## Your turn

- Read the error in the build output.
- In `sensor.rs`, change `pub trait Sensor {` to `pub trait Sensor: Send {`.
- Save. Press Run: ping no longer stalls the bar.
