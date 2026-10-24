Finishing a quest that raises your level shows a box over everything: "Level up!", "You reached level 3", and a Continue button. Until you press it, the screen behind is dimmed and can't be clicked. That box is a **modal**, and egui has one.

**When to show it.** The state says whether a level-up is waiting: `level_up: Option<u32>` is `Some(3)` until you dismiss it. Each frame, `ui` draws the screen and then, only when `level_up` holds a level, the modal. Clearing it to `None` is all it takes to make the box disappear next frame.

**A box with a closure.** `egui::Modal::new(egui::Id::new("level_up"))` builds the modal; the `Id` is a name egui uses to remember it between frames. `.show(ctx, |ui| { .. })` draws it over everything, and the closure fills it, just like `ui.horizontal`. `ui.heading(text)` is a label in big letters.

**Decide inside, act after.** The Continue button lives inside the closure, so record the click in a local `close` flag and clear `self.level_up` after `show` returns: the same "click now, act after" shape as last quest. `show` also hands back a response, and `response.should_close()` is true when the player pressed Esc or clicked outside the box, which should close it too.

```rust
let mut quit = false;
let response = egui::Modal::new(egui::Id::new("confirm_quit")).show(ctx, |ui| {
    ui.heading("Quit?");
    if ui.button("Yes").clicked() {
        quit = true;
    }
});
if quit || response.should_close() {
    self.confirming = false;
}
```

**The context.** A modal belongs to the whole window, not to one area of it, so it is shown on the **context**, egui's handle to the window, instead of on `ui`. `ui.ctx().clone()` gets it; the starter does that for you and passes `ctx` in.

## Your turn

- In `level_up_modal`, show a modal with the heading `Level up!`, the label `You reached level N` and a `Continue` button.
- Record a Continue click in a local flag.
- After the modal, clear `self.level_up` when Continue was clicked or the modal should close.
