On the taskbar Kinetik's stats sit on a rounded, see-through "pill" so the taskbar shows through a little. The Appearance page has a slider for how see-through. The bar now has an `opacity` setting, from 0.0 for invisible to 1.0 for solid.

**Four parts, not three.** An `Rgba` colour has a red, green and blue part and a fourth, `a`, the **alpha**: how much of what is behind shows through. `rgb(..)` always gives alpha 1.0, solid.

**Change one field.** `Rgba` is a struct with public fields `r`, `g`, `b` and `a`, each from 0.0 to 1.0. Make the colour `mut`, set `a` to the slider's value, and give it back:

```rust
colour.a = opacity;
```

**Alpha, not colour.** The grey stays the same at every setting; only how much shows through changes, so the slider never turns the bar a different shade.

**Rounded corners.** The starter's bar already calls `.rounded(px(8.))` for the pill shape, and `.px_2()` for room inside it. In this window you see the pill against the window's background; on the taskbar, in a later world, you see the taskbar through it.

## Your turn

- In `pill_background`, make `colour` `mut` and set `colour.a = opacity` before giving it back.
- Save. Press Run, then try `opacity: 0.3` in `demo`.
