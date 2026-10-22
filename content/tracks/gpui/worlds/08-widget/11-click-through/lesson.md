Locked, Kinetik's widget becomes click-through: clicks on it go to whatever is underneath. A widget over your browser should not eat the click meant for a link. Windows does this with two extended-style bits.

**Two bits.** `WS_EX_LAYERED` (0x80000) and `WS_EX_TRANSPARENT` (0x20) together make Windows pass mouse clicks through a window. `CLICK_THROUGH` holds both.

**Keep the others.** A window's extended style has many other bits: the tool-window bit, the no-activate bit, ones GPUI set. Overwriting the whole number would break those. Change only the two bits: `|` sets them, `& !` clears them:

```rust
if on { style | CLICK_THROUGH } else { style & !CLICK_THROUGH }
```

**Read, change, write.** `RealWin::set_click_through` reads the current style with `GetWindowLongPtrW`, passes it through your function, and writes it back.

**Unlock from the bar.** A click-through widget cannot be right-clicked any more, so Kinetik unlocks it from the bar's menu. The lock setting lives in `Settings`, reachable from anywhere.

## Your turn

- In `click_through_style`, give back `style` with `CLICK_THROUGH` added when `on`, removed when not.
- Save.
