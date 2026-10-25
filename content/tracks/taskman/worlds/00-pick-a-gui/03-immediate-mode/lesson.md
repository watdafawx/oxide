egui is the first route for the task manager. It works differently from most toolkits: your code draws the whole window, right now, every time.

**One function draws everything.** You write a function that takes a `ui` and calls things on it: a label here, a button there, a row of three labels. egui runs that function and draws what it said. egui keeps a little bookkeeping of its own between runs (scroll position, which box has the keyboard), but no widget objects for you to manage. Your data lives in your own struct (for the task manager, the list of processes).

**A click is an answer.** `ui.button("CPU")` draws a button and hands back what happened to it. `.clicked()` on that answer is `true` during the run that handles the click. So a click is an `if`, right where the button is drawn.

```rust
if ui.button("Refresh").clicked() {
    self.refresh();
}
```

**This style's name.** Drawing from scratch on every run, with no widget objects kept, is called **immediate mode**. Each run of your function is called a **frame**. Your struct is the only truth: change a field, and the next run draws the new value.

**Only when needed.** egui runs your function when something happens: the mouse moves, a key goes down, or your code asks for a frame with `ctx.request_repaint_after(Duration::from_secs(1))`. A task manager at rest runs it about once a second.

**Long lists.** egui's companion crate `egui_extras` has a table that draws only the rows you can see. 400 processes cost about as much as the 30 on screen.

**Where egui shines.** Tools, debug panels, settings screens, overlays inside games. Testing is easy too: a test runs your function without a window, finds a widget by its text and clicks it.
