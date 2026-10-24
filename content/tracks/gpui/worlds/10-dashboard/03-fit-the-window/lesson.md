On a 4K monitor the dashboard should show six columns of cards; in a narrow window, one. Four is right only for one window size, so the number of columns has to come from the window's width.

**Ask the window.** `window.viewport_size()` gives the size you can draw in, a `Size<Pixels>` with a `.width` and a `.height`. `render` gets the `window` as its second input, so every redraw, including the one after a resize, asks again.

**Lengths are not numbers.** A `Pixels` is a length. GPUI keeps lengths apart from plain numbers, so you cannot add a width to a count by mistake. `f32::from(width)` takes the plain number out, for maths that is not about lengths.

**How many fit.** A card looks right at 320 pixels or more, which is `CARD_MIN_WIDTH`. The columns that fit are the width over 320, and `as usize` drops the fraction: 1400 / 320 is 4.375, so 4 columns. A list works the same way with rows:

```rust
let rows = (f32::from(height) / ROW_HEIGHT) as usize; // 400 px / 40 px = 10 rows
```

**Keep it sane.** Under 320 pixels the sum is 0 columns, and the dashboard would draw nothing. At 5000 pixels it is 15 tiny columns. `.clamp(1, 6)` keeps the answer between 1 and 6; it works on a `usize` just like the `f32::clamp` from the Views world.

**Tested by resizing.** The test calls `simulate_resize` to change the window's size without a screen, then checks which cards share a row.

## Your turn

- In `columns_for`, divide the width by `CARD_MIN_WIDTH` as plain numbers and make it a `usize`.
- Clamp it to 1..=6. Save, press Run and drag the dashboard's edge.
