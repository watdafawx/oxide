Kinetik's widget shows CPU cores as a per-thread bar chart: one little bar per thread, filled by its usage. A 16-thread CPU makes 16 bars; a Threadripper 32. They have to fit whatever width the widget has.

**Per thread.** `real_cores` asks sysinfo for every thread's usage. The widget refreshes that list each time the bar reads its sensors.

**Wrap like text.** A flex row with `.flex_wrap()` puts children side by side until the row is full, then starts a new row below, like words in a paragraph. No maths about how many fit.

**Fill from the bottom.** Each bar is a dark 28 by 20 box. `.items_end()` sits its child at the bottom, and the child's height is the usage, as a share:

```rust
.child(div().w_full().h(relative(usage / 100.0)).bg(rgb(0x5ab0ff)))
```

**Fresh each read.** The widget asks `real_cores` again each time the bar reads its sensors, so the bars move once per interval, in step with the numbers.

## Your turn

- In `core_bars`, make the box a wrapping flex row with a small gap.
- Add one child per core: a 28 by 20 box, filled from the bottom by its usage, named `core-<i>`.
- Save. Press Run: your CPU's threads under the stats.
