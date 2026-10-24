Every piece of the overlay exists: parsing, FPS, lows, the ten-second window, spikes, base FPS, the live trace, what counts as a game, the strip window, hotkeys, auto-show, session lines and benchmarks. Two pieces join them up, and this boss writes both.

**Frame stats.** `FrameStats::compute` turns the frames in the window into everything the strip shows: FPS, base FPS, both lows, the last 3 seconds of frame times for the graph, and which of those are spikes. `OverlayView` calls it twice a second. Every number comes from a function you already wrote; the new work is picking the right frames for each.

**The newest frame.** "The last 3 seconds" counts back from the newest frame's time. `frames.last()` gives an `Option`, `None` for no frames, so `.map(|f| f.time).unwrap_or(0.0)` reads its time with a safe fallback. The recent frame times are the ones whose time is at least the newest minus 3.

**The controller.** `OverlayController::tick` runs once a second in `main.rs`. In order: keep the peak GPU temperature while a game is remembered, ask `AutoShow` what to do with `self.auto.step(front, running)`, remember when a session started on Show, and on Close write the summary line into `sessions`. It gives back the step it got.

**The higher of two maybes.** Both the peak so far and the new reading may be missing. A `match` on the pair covers all four cases in three arms: both there, take the higher; only the old one, keep it; otherwise the new one, which may be `None` too. Here the same match keeps the lowest ping:

```rust
let best = match (best, ping) {
    (Some(a), Some(b)) => Some(a.min(b)),
    (Some(a), None) => Some(a),
    (None, b) => b,
};
```

**No windows inside.** The controller only decides; `main.rs` opens and closes the window by dispatching `ToggleOverlay`. That keeps it testable with plain numbers.

**Live on your PC.** `main.rs` starts the DXGI trace and feeds its frames, from ETW's thread, into the overlay's `Arc<Mutex<FrameWindow>>`, the lock from the overlay-window quest. Without administrator the overlay says so; run Oxide as administrator to see live FPS.

## Your turn

- Write `FrameStats::compute`: the first three objectives, then the hotkey one, turn green.
- Write `OverlayController::tick`.
- Press Run, then start a game in borderless mode, or press Ctrl+Shift+F10.
