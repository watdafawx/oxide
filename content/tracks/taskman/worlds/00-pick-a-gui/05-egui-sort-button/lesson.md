The list now shows three processes, in the order Windows gave them. A task manager sorts: click the CPU column and the busiest process jumps to the top. In egui that is three lines.

**The list.** `TaskList` holds a `Vec<Process>`, and its `ui` draws a row for each one with your `row_ui`. `sort_by_cpu` is already written: it puts the highest CPU first.

**A button answers.** `ui.button("CPU")` draws a button and hands back a **response**: what happened to that button on this run. `.clicked()` on it is `true` once, in the frame that handles the click.

**So a click is an if.** Draw the button and ask, in one line. If it was clicked, sort. The rows are drawn a few lines later in the same run, so they already show the new order.

```rust
if ui.button("Name").clicked() {
    self.sort_by_name();
}
```

**Nothing to wire up.** No listener, no callback, no "tell the screen it changed". You changed `self.processes`, and egui draws whatever `self` holds. The test clicks the button with `click()`, runs one more frame, and reads the order.

## Your turn

1. At the top of `TaskList::ui`, draw a `CPU` button.
2. When it is clicked, call `self.sort_by_cpu()`.
3. Save.
