The row model from the last quest knows what to say. Now egui draws it: the process name, its CPU and its memory, on one line, the way every row of the task manager will look.

**A label.** `ui.label("chrome.exe")` draws a line of text at the `ui`'s current position. Each call goes under the one before, like lines on a page.

**One line.** `ui.horizontal(|ui| { .. })` takes a closure, and everything drawn inside it goes left to right instead. The closure gets its own `ui`; draw on that one.

```rust
ui.horizontal(|ui| {
    ui.label(row.name.as_str());
    ui.label(row.cpu.as_str());
});
```

**Text from a String.** A label wants text it can read. `row.name.as_str()` lends it the `String` as a `&str`, and the row keeps its text.

**A test without a window.** The tests use **egui_kittest**: `Harness::new_ui` runs `row_ui` the way egui would, but with no window. `get_by_label("512 MB")` finds the widget showing that text, and fails the test if there is none. Each widget it finds knows where it was drawn, so a test can check left and right too. A missing label fails with `No nodes found matching the query`, then a long list of every widget it did find: read the first line, skip the rest.

**The first build.** This quest builds egui and egui_kittest: about a minute the first time, then a few seconds per save.

## Your turn

1. In `row_ui`, inside the `horizontal` closure, add a label for `row.cpu` after the name.
2. Add a label for `row.memory` after it.
3. Save.
