Press Run on the starter: CPU and RAM sit one above the other, like lines in a document. Kinetik's bar is one row on the taskbar, so the outer box has to lay its children out left to right.

**Boxes stack by default.** A plain `div()` puts each child under the one before, at full width. That is right for a page of text and wrong for a bar.

**The word for it.** A **flex** box lays its children out along a line. `.flex()` switches it on, `.flex_row()` makes the line run left to right, and `.flex_col()` makes it run top to bottom. Each is one more link in the builder chain on the box.

**Leave a gap.** Side by side with nothing between, `CPU 13%RAM 8.0 GB` runs together. `.gap_N()` puts the same space between every pair of children. The `N` is a step on GPUI's spacing scale, not a number of pixels: `gap_1` is about 4 pixels, `gap_2` about 8. The quest after next explains the scale.

**The same calls, on their side.** A column with a tight gap looks like this. The bar wants the other direction and the next step up:

```rust
div().flex().flex_col().gap_1()
```

**For the curious.** GPUI lays elements out with Taffy, a flexbox engine, so `flex`, `gap` and `items_center` mean what they mean in CSS flexbox, and the style names follow Tailwind CSS.

**Fixed numbers for now.** The pill texts are typed in by hand. A few quests from now they come from the model you built in Readings.

## Your turn

- In `render`, make the outer `div()` a flex row with a `gap_2` between its children.
- Save. Press Run: the pills sit side by side.
