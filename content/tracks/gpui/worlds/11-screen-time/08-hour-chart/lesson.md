Below the summary, Kinetik draws the day as 24 bars, one per hour, so you see at a glance when you used the PC. The bar's menu has a new row, Screen time, which opens the page. The page draws the bars; this quest works out how tall each one is.

**Relative heights.** The busiest hour fills the chart, and every other hour gets its share of that: half the busiest hour's seconds, half the height. A light day still fills the chart and stays easy to read, which heights out of 3600 seconds would not.

**Find the busiest.** `hours.iter().copied().max()` gives the most seconds in any hour, as an `Option`, because `max` of nothing is nothing. `.unwrap_or(0)` turns that into a plain number.

**Never divide by zero.** On an empty day the busiest hour has 0 seconds, and every share would be 0 / 0, which is `NaN` for floats: not a number, and not a height. Check first, and give back 24 zeros with `[0.0; 24]`.

**Map an array.** Arrays have their own `map`, which gives back an array of the same length, so 24 numbers in become 24 numbers out with no `Vec`. Turning each hour's seconds into minutes:

```rust
hours.map(|secs| secs / 60) // [u64; 24] of minutes
```

**Floats for heights.** Heights are `f32` pixels, so cast before you divide: `tallest * secs as f32 / busiest as f32`. Dividing the `u64`s first would round every share down to 0 or 1.

**Bars at the bottom.** The page draws the 24 bars in a flex row with `.items_end()`, so each bar stands on the bottom edge, and `.flex_1()` gives all of them the same width.

## Your turn

- In `bar_heights`, find the busiest hour's seconds, and give back zeros when it is 0.
- Otherwise map each hour to its share of `tallest`. Save.
- Press Run, then open Screen time from the bar's menu.
