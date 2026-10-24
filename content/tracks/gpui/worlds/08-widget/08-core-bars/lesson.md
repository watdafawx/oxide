Kinetik's widget shows the CPU as a per-thread bar chart: one little bar per thread, filled by its usage. A 16-thread CPU makes 16 bars; a Threadripper makes 64. They have to fit whatever width the widget has.

**Per thread.** `cores` holds one usage per thread, 0 to 100. The widget asks sysinfo again each time the bar reads its sensors, so the bars move once per interval, in step with the numbers.

**Wrap like text.** A flex row with `.flex_wrap()` puts its children side by side until the row is full, then starts a new row below, like words in a paragraph. You do no maths about how many fit; a narrower widget simply gets more rows.

**One child per core.** `.children(..)` takes one element per item, as the bar's pills did in the Views world. `cores.iter().enumerate()` gives each usage with its position `i`, which names the bar `core-<i>` for the tests.

**Fill from the bottom.** Each bar is a dark 28 by 20 box. In a flex row, a child shorter than the row sits at the top; `.items_end()` lines the children up along the bottom instead. Inside, the fill is `.w_full()`, the whole width, and as tall as the usage, as a share: `relative(usage / 100.0)`.

**The same tools, on a row of tags.** Here a row of process names wraps, one small box each; your bars are boxes with a fill instead of text:

```rust
div()
    .flex()
    .flex_wrap()
    .gap_1()
    .children(names.iter().map(|name| div().px_2().bg(rgb(0x2a2a2a)).child(name.clone())))
```

**For the curious.** `real_cores` in `widget.rs` keeps its sysinfo `System` in a `thread_local!`: one per thread, kept between calls, because CPU usage is measured as the change since the last refresh.

## Your turn

- In `core_bars`, make the box a flex row that wraps, with `.gap_1()`.
- Add one child per core: a 28 by 20 box named `core-<i>`, filled from the bottom by a full-width child as tall as its usage.
- Save. Press Run: your CPU's threads appear under the stats.
