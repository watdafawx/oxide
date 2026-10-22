Kinetik lets you pick the bar's font size. A size of 4 is unreadable and 40 does not fit in a 32-pixel taskbar, and a settings file edited by hand can hold anything. The bar takes the setting and keeps it in a sensible range before using it.

**Set the size.** `.text_size(..)` sets the text size of a box and everything inside it, unless a child sets its own. The bar now calls it with `bar_text_size(self.font_size)`.

**Keep it in range.** `f32::clamp(min, max)` gives back the number itself when it is in range, `min` when it is below, `max` when it is above. One call, no `if`:

```rust
px(setting.clamp(8.0, 24.0))
```

**Fix it where it is read.** The setting stays what the person typed; the bar uses the clamped size. If the range ever changes, it changes in this one function.

## Your turn

- In `bar_text_size`, clamp `setting` to 8.0 and 24.0 before `px(..)`.
- Save. Press Run, then try `font_size: 40.0` in `from_snapshot`.
