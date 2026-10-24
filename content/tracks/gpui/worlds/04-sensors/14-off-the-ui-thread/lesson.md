`bar.rs` now does the hop from the last quest: the sensors sit in an `Arc<Mutex<..>>`, and every interval `start` hands them to the background executor, which runs `read_sensors` on a worker thread. `apply` stores the result on the UI thread. Read `start` once: it is the three steps from the map. Then build, because it does not.

**The compiler says no.** The error points at the background `spawn` and names the culprit, `dyn Sensor`:

```text
error: future cannot be sent between threads safely
  = help: the trait `std::marker::Send` is not implemented for `dyn Sensor`
```

**Why it asks.** The future you give the background `spawn` carries the sensors to a worker thread, so everything in it must be **Send**, safe to move to another thread. A plain struct is Send when all its fields are, and the compiler works that out by itself. Not every type is: an `Rc` is not, because two threads could change its count at once.

**Why it cannot tell.** `dyn Sensor` means "some type that implements `Sensor`", decided while the program runs. The compiler cannot look inside, and any sensor someone writes later could hold an `Rc`. So it refuses, though `CpuSensor`, `NvidiaGpu` and the ping are all Send already: nobody promised it for the trait.

**Promise it once.** `trait Sensor: Send` makes Send part of being a sensor. Send is a **supertrait** here, a trait that every implementer must also have. Now every `impl Sensor for ..` is checked for Send where it is written, and `dyn Sensor` is known to be Send everywhere it is used. The same promise on another trait:

```rust
pub trait Launcher: Send {
    fn launch(&mut self, path: &str);
}
```

**Not on the UI thread.** `cx.spawn` needed no such promise: its task stays on the UI thread, so nothing in it moves. Only the hop to a worker asks for Send.

## Your turn

- Read the error in the build output.
- In `sensor.rs`, make Send part of the `Sensor` trait.
- Save. Press Run: ping no longer stalls the bar.
