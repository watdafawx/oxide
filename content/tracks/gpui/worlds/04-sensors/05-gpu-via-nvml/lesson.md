Kinetik reads NVIDIA cards through `nvml.dll`, which the NVIDIA driver installs. No administrator needed. The `nvml-wrapper` crate loads the DLL and wraps its functions. On a PC without an NVIDIA card the DLL is missing, and the bar must say `--`, not crash.

**A library loaded late.** A **DLL** is a library Windows loads while the program runs, not one built into it. That is why it can be missing: the program starts fine, and only the load fails. `NvidiaGpu::new()` calls `Nvml::init()` and keeps the result as an `Option`: `Some` when the DLL loaded, `None` when it did not. `NvidiaGpu::none()` makes the empty kind on purpose, for tests.

**Three things can fail.** No library; no card at index 0; the card did not answer. Each step gives an `Option` or a `Result`, and you want one rule for all of them: the first gap ends the function with `None`.

**Result to Option.** `?` in a function that returns `Option` only works on an `Option`. `.ok()` turns a `Result` into one, dropping the error: `Ok(card)` becomes `Some(card)`, any `Err` becomes `None`. Then `?` passes the `None` on.

**Already written.** `temperature`, just below `usage`, is the same chain for a different number. Read it before you write:

```rust
let device = self.nvml.as_ref()?.device_by_index(0).ok()?;
Some(device.temperature(TemperatureSensor::Gpu).ok()? as f64)
```

**The number you need.** `utilization_rates()` gives a struct whose `gpu` field is the usage, a `u32` from 0 to 100. The second objective runs on your real card; on a PC without NVIDIA it passes by skipping, and the first objective still checks the empty case.

## Your turn

- In `usage`, get the card the same way `temperature` does.
- Ask it for its utilization rates, ending with `None` when it does not answer, and give back the `gpu` field as `f64`.
- Save. Press Run: GPU usage and temperature are your PC's own.
