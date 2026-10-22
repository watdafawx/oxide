The capture taught the maths; live frames come from event tracing. `start_present_trace` in `live.rs` uses the `ferrisetw` crate to listen to Microsoft-Windows-DXGI, the provider every Direct3D present goes through, and calls back with the process and the time of each present.

**Admin or not.** Starting a real-time trace session without administrator fails with Windows error 5, "access denied". That is not a bug; it is the normal case for most users. The overlay must say "needs administrator", not show an error code.

**Nested errors.** ferrisetw wraps the reason in layers: a `TraceError` holding an `EvntraceNativeError` holding an `io::Error`. A `match` pattern can reach through all three at once.

**A guard.** `if` after a pattern adds a condition: the arm matches only when the pattern fits and the condition is true. Error 5 means admin; anything else keeps its text:

```rust
TraceError::EtwNativeError(EvntraceNativeError::IoError(e)) if e.raw_os_error() == Some(5) => LiveError::NeedsAdmin,
```

**A live test.** The second objective really starts a trace. Run Oxide as administrator and it starts; otherwise it must come back as `NeedsAdmin`.

## Your turn

- In `live_error`, `match` on `error`: the IoError with OS error 5 is `NeedsAdmin`.
- Every other error is `Failed` with its `{:?}` text.
- Save.
