Kinetik's widget can sit on the desktop behind your windows, be a normal window, or stay always on top. It is the same window each time; only its place in the stack of windows differs. `apply_placement` turns the setting into that place.

**A stack of windows.** Windows keeps every window in an order, front to back, called the z-order. `SetWindowPos` with `HWND_BOTTOM` sends a window to the back, `HWND_TOPMOST` to the always-on-top front, and `HWND_NOTOPMOST` back among the normal ones.

**Three layers.** `Layer` names those three places: `Bottom`, `Normal` and `Top`. Desktop placement is the bottom, behind everything; Normal is among the others; On top is the top.

**Through the trait.** `WinApi` gains `set_layer(hwnd, layer)`. `RealWin` makes the `SetWindowPos` call; `FakeWin` only records it, so the test can check which layer was asked for without a real window.

**Pick, then call.** A `match` gives back a value, so it can sit on the right of a `let`: first pick the layer, then make one call with it. Here the same shape picks a text colour, then uses it:

```rust
let colour = match appearance {
    WindowAppearance::Dark | WindowAppearance::VibrantDark => rgb(0xffffff),
    _ => rgb(0x1e1e1e),
};
label.text_color(colour)
```

**One caveat.** Kinetik's README warns that Win+D (Show desktop) can hide a desktop-placed widget. Always on top is the fix there, which is one reason the setting exists.

## Your turn

- In `apply_placement`, `match` on `placement` to pick the `Layer` for each of the three placements.
- Call `api.set_layer` with `hwnd` and that layer.
