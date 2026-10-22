White text on a light taskbar is invisible. Kinetik's Colours page has "Match the taskbar theme": text turns dark when Windows is in light mode. GPUI tells every window which mode it is in.

**Ask the window.** `window.appearance()` gives a `WindowAppearance`: `Light`, `Dark`, or their "vibrant" variants, which macOS uses and Windows reports as the plain ones. The bar's `render` now asks on every frame.

**One arm for two.** In a `match`, `|` lets one arm cover several variants. Dark and vibrant dark get white; light and vibrant light get near-black, `0x1a1a1a`, which is softer than pure black:

```rust
WindowAppearance::Dark | WindowAppearance::VibrantDark => rgb(0xffffff),
```

**Picked up on redraw.** When Windows switches mode, GPUI updates the window's appearance, and the bar's next redraw uses the new colour. `cx.observe_window_appearance` lets a view react the moment it changes.

**Warnings stay red.** Only the normal text follows the mode. `value_colour` still paints warnings red on either.

## Your turn

- In `view.rs`, fill in `text_colour_for`: white for the two dark appearances, `rgb(0x1a1a1a)` for the two light ones.
- Save. Press Run, then switch Windows between light and dark mode.
