The Arrange list is every slot on the bar. Kinetik's real list has all 44 stats plus dividers, and each row is a clickable, draggable element. Building rows that are scrolled out of sight wastes work on every frame.

**The word for it.** A **virtual list** builds only the rows you can see. GPUI's `uniform_list` does that for rows of one height: it measures the first row, works out which ones fit, and asks for just those.

**Ranges, not lists.** The closure gets a `Range<usize>` of row numbers and gives back their elements. `cx.processor(..)` wraps it so it also gets the window view, like `cx.listener` does for events:

```rust
cx.processor(|this, range: Range<usize>, _, cx| {
```

**Give it a height.** A virtual list needs a fixed height to know how many rows fit. 140 pixels shows five 28-pixel rows; the rest scroll.

**Same rows.** `arrange_row` is unchanged: each row is still clickable, and in the next quest draggable.

## Your turn

- In `arrange_list`, replace the plain list with `uniform_list("arrange-list", count, cx.processor(..))`.
- In the closure, map the range to `arrange_row(i, order[i], i == this.selected, cx)` and `collect()`.
- Give the list `.h(px(140.))`. Save.
