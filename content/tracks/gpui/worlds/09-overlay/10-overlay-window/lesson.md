Kinetik's overlay is a slim strip on top of the game. It must never steal focus (the game would pause or lose input), be see-through around its text, and sit where it does not cover the action: top centre.

**The window.** `toggle_overlay_window` opens a `PopUp`, transparent, unfocused, unmovable window, the same recipe as the bar. `OverlayView` draws FPS, base FPS, the lows and a frame-time graph from a `FrameWindow`, every half second.

**Top centre.** The left edge is the screen's left plus half the space the strip does not use. `Pixels` divide by a number like any length:

```rust
let x = screen.origin.x + (screen.size.width - width) / 2.0;
```

**A replay, for now.** `main.rs` fills the overlay with the capture, so Run shows real-looking numbers. The boss and the live trace from quest 8 feed it from games.

**Exclusive fullscreen.** No overlay can draw over exclusive fullscreen; only borderless and windowed games. Kinetik's README says the same.

## Your turn

- In `overlay_bounds`, work out `x` for the strip centred on the screen.
- Give back bounds at that `x`, 8 px below the screen's top, of the given size.
- Save. Press Run: the overlay strip appears at the top of your screen.
