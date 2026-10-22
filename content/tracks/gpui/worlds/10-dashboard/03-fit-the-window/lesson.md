Kinetik's dashboard grid "fits the window and scales its text, gauges and graphs to match". On a 4K monitor you want six columns; in a narrow window, one. The number of columns comes from the window's width.

**Ask the window.** `window.viewport_size()` gives the drawable size. `render` gets `window`, so every redraw, including after a resize, works it out again.

**How many fit.** A card looks right at least 320 pixels wide. The columns that fit are the width over 320, as a whole number:

```rust
((f32::from(width) / CARD_MIN_WIDTH) as usize).clamp(1, 6)
```

**Keep it sane.** Under 320 pixels that is 0 columns, which would draw nothing; at 5000 pixels it is 15 tiny columns. Clamp to 1..=6.

**`Pixels` to a number.** `f32::from(width)` takes the plain number out of a `Pixels`, for maths that is not about lengths.

## Your turn

- In `columns_for`, divide the width by `CARD_MIN_WIDTH`, make it a `usize`, and clamp to 1..=6.
- Save. Press Run and resize the dashboard.
