Kinetik's widget can sit on the desktop behind your windows, be a normal window, or stay always on top. It is the same window each time; only its place in the stack of windows differs.

**A stack of windows.** Windows keeps every window in an order, front to back. `SetWindowPos` with `HWND_BOTTOM` sends one to the back, `HWND_TOPMOST` to the always-on-top front, `HWND_NOTOPMOST` back among normal ones.

**Through the trait.** `WinApi` gains `set_layer(hwnd, Layer)`; `RealWin` makes the call, `FakeWin` records it. `apply_placement` only picks the layer:

```rust
Placement::Desktop => Layer::Bottom,
```

**One caveat.** Kinetik's README warns that Win+D (Show desktop) can hide a desktop-placed widget. Always on top is the fix there, which is why the setting exists.

**Applied on change.** The boss applies the placement when the widget opens; switching it on the Widget page would call the same function again.

## Your turn

- In `apply_placement`, `match` on `placement` to pick the `Layer`.
- Call `api.set_layer(hwnd, layer)`.
- Save.
