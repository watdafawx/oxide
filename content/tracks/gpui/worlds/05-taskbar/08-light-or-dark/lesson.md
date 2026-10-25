White text on a light taskbar is invisible. Kinetik's Colours page has "Match the taskbar theme": text turns dark when Windows is in light mode. GPUI tells every window which mode it is in.

**Ask the window.** `render` gets a `window: &mut Window` as well as `cx`; until now the bar never needed it. `window.appearance()` gives a `WindowAppearance`: `Light`, `Dark`, or their "vibrant" variants, which macOS uses and Windows reports as the plain ones. The bar's `render` now asks on every frame.

**One arm for two.** In a `match`, `|` lets one arm cover several variants: "this one or that one". Dark and vibrant dark get white; light and vibrant light get near-black, `0x1a1a1a`, which is softer than pure black. The same move, grouping two stats into one label:

```rust
StatKind::NetDown | StatKind::NetUp => "network",
```

**Every variant covered.** `match` must handle every variant, so a new appearance in a later GPUI would stop the build until you choose its colour. That is why the arms name all four rather than ending in `_ =>`.

**Picked up on redraw.** When Windows switches mode, GPUI updates the window's appearance, and the bar's next redraw uses the new colour. `cx.observe_window_appearance` lets a view react the moment it changes. Warnings stay red: `value_colour` still paints them red in either mode.

## Your turn

- In `view.rs`, fill in `text_colour_for`: white for the two dark appearances, `rgb(0x1a1a1a)` for the two light ones.
- Save. Press Run, then switch Windows between light and dark mode.
