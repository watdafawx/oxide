A dashboard on a second monitor is read from across the room. The big number on each card should grow with the card, and never shrink until it cannot be read.

**Share of the width.** A tenth of the card's width works well: a 320-pixel card gets 32-pixel numbers. `render` works out each card's width from the window and the columns.

**Readable both ways.** Below 14 pixels nobody reads it; above 32 the number crowds the card. The same clamp you used for the bar's font size, on a computed value:

```rust
px((f32::from(card_width) / 10.0).clamp(14.0, 32.0))
```

**Only the big number.** Titles and the avg/peak line keep small fixed sizes; they are labels, not the thing you glance at.

**Recomputed on resize.** `render` runs again when the window changes size, so the numbers grow and shrink as you drag its edge.

## Your turn

- In `card_text_size`, give back a tenth of `card_width`, clamped to 14..=32, as `px`.
- Save.
