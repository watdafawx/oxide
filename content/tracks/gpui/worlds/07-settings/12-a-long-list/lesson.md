The Arrange list is every slot on the bar. Kinetik's real list has all 44 stats plus dividers, and each row is a clickable, draggable element. Building rows that are scrolled out of sight wastes work on every frame.

**The word for it.** A **virtual list** builds only the rows you can see. GPUI's `uniform_list` does that for rows of one height: it measures the first row, works out which ones fit, and asks for just those. `uniform_list(id, count, ..)` takes a name, the number of rows, and a closure.

**Ranges, not lists.** The closure gets a `Range<usize>` of row numbers, say `0..5`, and gives back one element per number. `cx.processor(..)` makes it, the way `cx.listener` makes a click handler: the first input is `this`, the `SettingsWindow`, then the range, the window, and `cx`.

**`collect` picks the type.** `range.map(|i| ..)` gives the rows one by one; `.collect()` gathers them into a collection. Which collection is decided by what the closure must return, a `Vec` of rows, so you do not write the type. The 15 swatches for CPU as a virtual list would be:

```rust
uniform_list("swatches", 15, cx.processor(|_, range: Range<usize>, _, cx| {
    range.map(|i| swatch(i, SWATCHES[i], StatKind::CpuUsage, cx)).collect()
}))
```

**Give it a height.** A virtual list needs a fixed height to know how many rows fit. 140 pixels shows five 28-pixel rows; the rest scroll.

**Same rows.** `arrange_row` is unchanged: each row is still clickable, and in the next quest draggable.

## Your turn

- In `arrange_list`, replace the plain list with `uniform_list("arrange-list", count, cx.processor(..))`, where `count` is the length of the global's order.
- In the closure, clone the order, map the range to `arrange_row(i, order[i], i == this.selected, cx)`, and `collect()`.
- Give the list `.h(px(140.))`. Save.
