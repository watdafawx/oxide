Every piece of screen time is written: the program in front, idle and lock rules, day files, pruning, the summary, bar heights, hot timers and toasts. This boss writes the two pieces that join them up.

**The sampler.** `ScreenTime::sample` runs every 5 seconds. It checks the date first, so the first sample after midnight starts a new day and prunes old files. Then it counts the program in front, if it counts, and saves the day.

**The page.** `ScreenTimeView::render` clones today's `Day` out of the counter, summarizes it, and draws five summary lines and 24 bars. Clone first: the borrow of `cx` ends before the page is built.

**The bars.** A row with `.items_end()` stands every bar on the bottom edge; `.flex_none()` keeps the chart its full height when the window is short:

```rust
div().debug_selector(move || format!("hour-{hour}")).flex_1().h(px(height))
```

**Wired up.** `main.rs` starts the counter with a `RealDesk`, gives it to the bar so the menu's Screen time row can open the page, and starts `watch_alerts` with `show_toast`.

## Your turn

- Write `ScreenTime::sample`: new day and prune, then count and save.
- Write `ScreenTimeView::render`: the summary lines and the chart.
- Press Run, then use the PC for a while and open Screen time from the bar's menu.
