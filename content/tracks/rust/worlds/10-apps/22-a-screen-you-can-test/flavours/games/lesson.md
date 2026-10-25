The mod manager's settings screen sets the render distance for the pack: a `-` button, the text "Render distance: 12" and a `+` button in a row, with Reset underneath. The rules are already written as methods on `RenderDistance`. What is missing is the screen, drawn with egui, and a way to click it from a test.

**Your struct is the state.** `RenderDistance` holds the number and the rules (`farther`, `closer`, `reset`, `label`), and none of them know about windows. That struct is the **view model**: the screen only reads it and calls its methods. Rules stay testable as plain Rust.

**One function draws it all.** egui calls `fn ui(&mut self, ui: &mut egui::Ui)` once per frame. Inside, each call adds one widget: `ui.label(text)` draws text (a `String` is fine), `ui.button("+")` draws a button. Next frame the function runs again, so the label always shows the current number.

**A button reports its click.** `ui.button("+")` draws the button and hands back a `Response` about it, and `.clicked()` on that response is `true` on the frame the button was clicked. So the whole click handler is an `if`: `if ui.button("+").clicked() { self.farther(); }`. No callbacks to register.

**Side by side.** Widgets stack downwards by default. `ui.horizontal(|ui| { .. })` takes a closure (world 03) and hands it an inner `ui`; widgets added inside sit in a row. When the closure ends, the next widget goes below the row again.

```rust
ui.horizontal(|ui| {
    ui.label(format!("FOV: {}", self.fov));
    if ui.button("Wider").clicked() {
        self.fov += 5;
    }
});
if ui.button("Launch").clicked() {
    self.launching = true;
}
```

**Clicks from a test.** The **egui_kittest** crate runs your `ui` function with no window: the tests' `Harness` finds a widget by its text, `get_by_label("+").click()` clicks it, and `harness.run()` lets egui run the frames that process the click. `get_by_label` fails the test if no widget shows that text, so a typo in a label is caught at once.

**First build.** This quest compiles egui and egui_kittest, about two to four minutes the first time. Later runs take seconds.

## Your turn

- In `ui`, put a `-` button, the label from `self.label()` and a `+` button in one row.
- Under the row, add a `Reset` button.
- Each button calls the matching method when clicked.
