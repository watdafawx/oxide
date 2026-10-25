Kinetik's overlay is a slim strip on top of the game. It must never steal focus (the game would pause or lose input), be see-through around its text, and sit where it does not cover the action: top centre.

**The window.** `toggle_overlay_window` opens a `PopUp`, transparent, unfocused, unmovable window, the same recipe as the bar. `OverlayView` draws FPS, base FPS, the lows and a frame-time graph from a `FrameWindow`, every half second.

**Shared with a lock.** The frames reach the overlay through a `FrameFeed`, an `Arc<Mutex<FrameWindow>>`. Later the ETW thread pushes frames into it while the overlay reads them on the UI thread. `Arc` lets both threads own the same window of frames; `Mutex` lets only one of them in at a time, through `lock()`.

**A poisoned lock.** If a thread panics while it holds the lock, the `Mutex` is marked **poisoned**, and `lock()` gives an error instead of the data, in case the panic left it half-changed. For a window of frame times a half-changed list is harmless, so `OverlayView` takes the data anyway: `lock().unwrap_or_else(|p| p.into_inner())`, where `into_inner` takes the data out of the error. Rust world 07 (Concurrency) covers `Arc`, `Mutex` and threads in full.

**Top centre.** The left edge is the screen's left plus half the space the strip does not use. `Pixels` add, subtract and divide by a number like any length. Here the same sum centres a box vertically instead:

```rust
let y = screen.origin.y + (screen.size.height - height) / 2.0;
```

**A replay, for now.** `main.rs` fills the overlay with the capture, so Run shows real-looking numbers. The boss and the live trace from quest 8 feed it from games.

**Exclusive fullscreen.** No overlay can draw over exclusive fullscreen; only borderless and windowed games. Kinetik's README says the same.

## Your turn

- In `overlay_bounds`, work out `x` for the strip centred on the screen.
- Give back bounds at that `x`, 8 px below the screen's top, of the given size.
- Save. Press Run: the overlay strip appears at the top of your screen.
