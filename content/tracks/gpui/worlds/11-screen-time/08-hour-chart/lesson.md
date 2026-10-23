Below the summary, Kinetik draws the day as 24 bars, one per hour, so you see at a glance when you used the PC. The bar menu has a new row, Screen time, which opens the page.

**Relative heights.** The busiest hour fills the chart; every other hour gets its share of that. Half the busiest hour's seconds, half the height.

**Never divide by zero.** On an empty day the busiest hour has 0 seconds. Check first and give back 24 zeros.

**Map an array.** Arrays have their own `map`, which gives back an array of the same length, so 24 seconds become 24 heights with no `Vec`:

```rust
hours.map(|secs| tallest * secs as f32 / busiest as f32)
```

**Bars at the bottom.** The page draws a flex row with `.items_end()`, so each bar stands on the bottom edge, and `.flex_1()` gives all 24 the same width.

## Your turn

- Write `bar_heights`: the busiest hour's seconds, zeros for an empty day, then each hour's share of `tallest`.
- Save. Press Run: the screen time page opens with today's chart.
