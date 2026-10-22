Watch a bar update every second: CPU goes from `9%` to `13%`, the value gets one character wider, and every stat to its right shifts. Kinetik's README suggests a monospace font so "numbers don't jump around". A monospace font fixes the width of each character, not the number of characters.

**The obvious fix, half done.** The bar already uses `.font_family("Consolas")`, a monospace font: every digit is equally wide, so `12%` and `13%` match. `9%` is still one character shorter than `13%`, and the bar still twitches.

**A floor for the width.** `.min_w(..)` gives a box a minimum width. Shorter content leaves the box at the minimum; longer content still grows it. 48 pixels fits `100%` at the bar's size:

```rust
.min_w(px(48.))
```

**Only the value.** Labels never change, so they keep their natural width. The number is the part that moves, so the floor goes on the value's box.

## Your turn

- In `PillView::render`, add `.min_w(px(48.))` to the value's box.
- Save. In tests both widths now match; on screen, the bar holds still as numbers change.
