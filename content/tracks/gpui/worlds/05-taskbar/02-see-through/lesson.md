The bar's window is a rectangle, but Kinetik's bar is a rounded pill with the taskbar visible around it. An opaque window paints its whole rectangle, corners included, and hides the taskbar under them. The window itself has to be see-through.

**Opaque by default.** A window's background is `WindowBackgroundAppearance::Opaque`: Windows fills it before GPUI draws, and the corners around the pill stay solid.

**Make it transparent.** With `WindowBackgroundAppearance::Transparent`, only what the view draws is drawn. Every pixel the view leaves alone shows what is behind the window:

```rust
window_background: WindowBackgroundAppearance::Transparent,
```

**Measured, not hoped.** A test put this window over white, black and red backdrops: the empty pixels matched the backdrop exactly, and the pill drew its own colour. Nothing leaks.

**The pill remains.** The bar's `pill_background(opacity)` from Views is now the only background there is: a rounded, slightly see-through grey on top of your taskbar.

## Your turn

- In `bar_window_options`, add `window_background: WindowBackgroundAppearance::Transparent`.
- Save. Press Run: only the rounded pill shows, with whatever is behind the window around it.
