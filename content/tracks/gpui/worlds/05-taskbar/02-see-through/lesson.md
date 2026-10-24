The bar's window is a rectangle, but Kinetik's bar is a rounded pill with the taskbar visible around it. An opaque window paints its whole rectangle, corners included, and hides the taskbar under them. The window itself has to be see-through.

**Opaque by default.** A window's background starts as `WindowBackgroundAppearance::Opaque`: Windows fills the whole rectangle before GPUI draws, so the corners around the pill stay solid. Written out, the default looks like this:

```rust
window_background: WindowBackgroundAppearance::Opaque,
```

**Make it transparent.** The enum has a `Transparent` variant too. With it, only what the view draws is drawn, and every pixel the view leaves alone shows what is behind the window. Windows mixes the two for you, a step called **compositing**.

**Measured, not hoped.** When this track was tested, the window was put over white, black and red backdrops: the empty pixels matched the backdrop exactly, and the pill drew its own colour. Nothing leaks.

**The pill remains.** The bar's `pill_background(opacity)` from Views is now the only background there is: a rounded, slightly see-through grey on top of your taskbar.

## Your turn

- In `bar_window_options`, change the window background so the window is see-through.
- Save. Press Run: only the rounded pill shows, with whatever is behind the window around it.
