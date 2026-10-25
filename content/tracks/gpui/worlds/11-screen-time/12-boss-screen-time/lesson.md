Every piece of screen time is written: the program in front, the idle and lock rules, day files, pruning, the summary, bar heights, hot timers and toasts. This boss writes the two pieces that join them up: the sampler that runs every 5 seconds, and the page that shows the day.

**Date first.** `ScreenTime::sample` starts with `desk.now()`, and `.date()` gives its day. When that date is not `self.day.date`, midnight has passed: start a fresh `Day::new(..)` and `prune` the folder, so the first sample of a new day also clears out the old files.

**Then count.** `app_to_count(desk)` gives the program to count, or `None` when the PC is locked or nobody is there. With `if let Some(app) = ..`, add it to the day at `now` and `save_day` straight away.

**Errors on purpose.** A folder that cannot be pruned or a file that cannot be saved must not stop the counting; the next sample tries again. `let _ = prune(..);` says that out loud: the `Result` is thrown away on purpose, and Rust stops warning about it.

**Clone, then build.** `render` reads the counter with `self.screen.read(cx)`, which borrows `cx`. Clone the `day` out of it first, as the dashboard took its settings first, then `summarize` it and work out `bar_heights(&day.hours, CHART_HEIGHT)`.

**The page.** A column: `.size_full()` fills the window, then `.p_4()`, `.flex()`, `.flex_col()`, `.gap_2()`, the dark background and light text. A title with `.text_xl()` for bigger text, then the five `summary_line`s from the last read, then the chart.

**The chart.** A row named `chart`, `CHART_HEIGHT` tall and `.w_full()` wide, with `.items_end()` so every bar stands on the bottom edge, and `.flex_none()` so a short window cannot squash it. The bars come from `enumerate`, which hands out each height with its hour, as in this list of named rows:

```rust
.children(names.iter().enumerate().map(|(i, name)| div().debug_selector(move || format!("row-{i}")).child(name.clone())))
```

**Wired up.** `main.rs` starts the counter with a `RealDesk`, gives it to the bar so the menu's Screen time row can open the page, and starts `watch_alerts` with `show_toast`.

## Your turn

- Write `ScreenTime::sample`: a new day and a prune when the date changed, then count and save.
- Write `ScreenTimeView::render`: the title, the five summary lines, and the chart of 24 bars named `hour-0` to `hour-23`.
- Press Run, then use the PC for a while and open Screen time from the bar's menu.
