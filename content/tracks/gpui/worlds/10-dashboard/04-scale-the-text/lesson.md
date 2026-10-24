A dashboard on a second monitor is read from across the room. The big number on each card should grow with the card, and never shrink so far that nobody can read it.

**Where the width comes from.** `render` already works out each card's width: `(width - px(32.)) / columns as f32`. The 32 pixels are the outer `div`'s `p_4` padding, 16 on the left and 16 on the right; what is left is shared between the columns.

**Share of the width.** A tenth of the card's width works well: a 320-pixel card gets 32-pixel numbers, a 200-pixel card 20.

**Readable both ways.** Below 14 pixels nobody reads it from a chair; above 32 the number crowds the card. That is the same `clamp` you used for the bar's font size, on a computed value. The gauge's line gets the same treatment with other numbers:

```rust
px((f32::from(card_width) / 40.0).clamp(4.0, 10.0)) // a 320 px card: 8 px thick
```

**Only the big number.** Titles and the avg/peak line keep small fixed sizes. They are labels, not the thing you glance at.

**Recomputed on resize.** `render` runs again when the window changes size, so the numbers grow and shrink as you drag its edge.

## Your turn

- In `card_text_size`, give back a tenth of `card_width`, kept between 14 and 32 pixels.
- Save.
