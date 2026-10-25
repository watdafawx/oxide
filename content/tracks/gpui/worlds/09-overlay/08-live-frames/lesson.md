The capture taught the maths; live frames come from event tracing. `start_present_trace` in `live.rs` uses the `ferrisetw` crate to listen to Microsoft-Windows-DXGI, the provider every Direct3D present goes through, and calls back with the process and the time of each present.

**Admin or not.** Starting a real-time trace session without administrator fails with Windows error 5, "access denied". That is not a bug; it is the normal case for most users. The overlay must say "needs administrator", not show an error code.

**Another thread.** ETW calls `on_present` from a thread of its own, many times a second, while GPUI keeps drawing on the UI thread. That is why its type says `Fn(u32, f64) + Send + Sync + 'static`: safe to hand to another thread (`Send`), safe to call from there (`Sync`), and holding nothing borrowed that could disappear (`'static`). The Sensors world met `Send` when it moved reading off the UI thread; Rust world 07 (Concurrency) covers threads in full.

**Nested errors.** ferrisetw wraps the reason in layers: a `TraceError::EtwNativeError` holding an `EvntraceNativeError::IoError` holding an `io::Error`. A `match` pattern can reach through all the layers at once by writing them inside each other, the way the values were built.

**A guard.** An `if` after a pattern, a **match guard**, adds a condition: the arm matches only when the pattern fits *and* the condition is true. Otherwise the next arm is tried. `raw_os_error()` gives the Windows error number as an `Option<i32>`. Here a guard picks out error 2, "file not found", from an `io::Error` result:

```rust
match result {
    Err(e) if e.raw_os_error() == Some(2) => Missing::NoFile,
    other => Missing::Failed(format!("{other:?}")),
}
```

**A live test.** The second objective really starts a trace on your PC. Run Oxide as administrator and it starts; otherwise it must come back as `NeedsAdmin`.

## Your turn

- In `live_error`, `match` on `error`: the `IoError` inside an `EtwNativeError` whose OS error is 5 is `LiveError::NeedsAdmin`.
- Every other error is `LiveError::Failed` with its `{:?}` text.
- Save.
