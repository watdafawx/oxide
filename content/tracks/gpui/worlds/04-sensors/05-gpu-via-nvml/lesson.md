Kinetik reads NVIDIA cards through `nvml.dll`, which the NVIDIA driver installs. No administrator needed. The `nvml-wrapper` crate loads the DLL and wraps its functions. On a PC without an NVIDIA card the DLL is missing, and the bar must say `--`, not crash.

**Maybe a library.** `NvidiaGpu::new()` calls `Nvml::init()` and keeps the result as an `Option`: `Some` when the DLL loaded, `None` when it did not. `NvidiaGpu::none()` makes the empty kind on purpose, for tests.

**Three things can fail.** No library; no card at index 0; the card did not answer. Each step gives an `Option` or a `Result`. `.ok()` turns a `Result` into an `Option`, and `?` ends the function with `None` at the first gap:

```rust
let device = nvml.device_by_index(0).ok()?;
```

**The number.** `utilization_rates()` gives a struct whose `gpu` field is the usage, a `u32` from 0 to 100. `temperature` underneath works the same way and is already written.

**This PC.** The second objective runs on your real card. On a PC without NVIDIA it passes by skipping, and the first objective still checks the empty case.

## Your turn

- In `usage`, chain the three steps with `?`: `self.nvml.as_ref()?`, `device_by_index(0).ok()?`, `utilization_rates().ok()?`.
- Give back `Some` of the `gpu` field as `f64`.
- Save. Press Run: GPU usage and temperature are this PC's own.
