Every piece of the overlay exists: parsing, FPS, lows, the ten-second window, spikes, base FPS, the live trace, what counts as a game, the strip window, hotkeys, auto-show, session lines and benchmarks. Two pieces join them up.

**Frame stats.** `FrameStats::compute` turns the frames in the window into everything the strip shows: FPS, base FPS, both lows, the last 3 seconds of frame times for the graph, and which of those are spikes. `OverlayView` calls it twice a second.

**The controller.** `OverlayController::tick` runs once a second in `main.rs`. It keeps the peak GPU temperature while a game is in front, asks `AutoShow` what to do, remembers when a session started, and on Close writes the summary line:

```rust
let step = self.auto.step(front, running);
```

**No windows inside.** The controller only decides; `main.rs` opens and closes the window by dispatching `ToggleOverlay`. That keeps it testable with plain numbers.

**Live on this PC.** `main.rs` starts the DXGI trace, follows the process in front, and feeds frames to the overlay. Without administrator the overlay says so; run Oxide as administrator to see live FPS.

## Your turn

- Write `FrameStats::compute`: the first three objectives, then the hotkey one, turn green.
- Write `OverlayController::tick`.
- Press Run, then start a game in borderless mode, or press Ctrl+Shift+F10.
