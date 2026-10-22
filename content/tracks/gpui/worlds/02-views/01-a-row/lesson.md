Press Run on the starter: CPU and RAM sit one above the other, like lines in a document. Kinetik's bar is one row on the taskbar. The outer box has to lay its children out left to right.

**Boxes stack by default.** A plain `div()` puts each child under the one before, at full width. That is right for a page of text and wrong for a bar.

**The word for it.** A **flex** box lays its children out along a line. `.flex()` switches it on, and `.flex_row()` makes that line run left to right. `.flex_col()` would run it top to bottom.

**Leave a gap.** Side by side with nothing between, `CPU 13%RAM 8.0 GB` runs together. `.gap_2()` puts a small fixed space between every pair of children:

```rust
.flex().flex_row().gap_2()
```

**Fixed numbers for now.** The pill texts are typed in by hand. A few quests from now they come from the model you built in Readings.

## Your turn

- In `render`, add `.flex().flex_row().gap_2()` to the outer `div()`.
- Save. Press Run: the pills sit side by side.
