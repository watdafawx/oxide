Locked, Kinetik's widget becomes click-through: clicks on it go to whatever is underneath. A widget over your browser should not eat the click meant for a link. Windows does this with two bits in a window's extended style.

**Two bits.** A window's extended style is one `u32` where each bit is a yes-or-no switch. `WS_EX_LAYERED` (0x80000) and `WS_EX_TRANSPARENT` (0x20) together make Windows pass mouse clicks through a window. `CLICK_THROUGH` holds both bits.

**Keep the others.** The same number has many other switches: the tool-window bit, the no-activate bit, ones GPUI set when it made the window. Writing `CLICK_THROUGH` over the whole number would switch all of those off. Change only your two bits and keep the rest.

**Set with or.** `style | bits` gives a number with every bit that is on in either: your bits switch on, the others stay as they were. **Clear with and-not.** `!bits` flips every bit, so it is on everywhere except your bits; `style & !bits` keeps only the bits on in both, which switches exactly your bits off. Here both are done to the tool-window bit:

```rust
let with_tool = style | TOOL_WINDOW;      // tool-window bit on, others kept
let without_tool = style & !TOOL_WINDOW;  // tool-window bit off, others kept
```

**Read, change, write.** `RealWin::set_click_through` reads the current style with `GetWindowLongPtrW`, passes it through your function, and writes it back.

**Unlock from the bar.** A click-through widget cannot be right-clicked any more, so Kinetik unlocks it from the bar's menu. The lock setting lives in `Settings`, reachable from anywhere.

## Your turn

- In `click_through_style`, give back `style` with `CLICK_THROUGH` switched on when `on` is true, and switched off when it is false.
- Save.
