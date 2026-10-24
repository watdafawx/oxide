GPUI has no slider widget, so the Appearance page builds one: a grey track, a blue knob at the current gap, and two mouse listeners that turn a press or a drag into a gap.

**Find the track.** `slider_value` needs the track's left edge and width in window pixels, and only layout knows them. A `canvas` inside the slider uses its first closure, the prepaint one, to record its own bounds into `gap_track` on every frame; `set_gap_at` reads them back and does the maths.

**A shared cell.** `gap_track` is an `Rc<Cell<Bounds<Pixels>>>`. The canvas closure and the window both hold an `Rc` to the same cell, and `Cell` lets either `set` or `get` the bounds through a shared handle. Both come from [Rust world 06](/quest/rust/06-smart-pointers/08-interior-mutability); here the starter uses them for you.

**Press to jump.** A left-button press anywhere on the track jumps the gap there. It is the right-click listener from the menu, with the left button and a different body; that one read:

```rust
.on_mouse_down(MouseButton::Right, cx.listener(|this, event: &MouseDownEvent, _, cx| {
    this.ui.menu_at = Some(event.position);
    cx.notify();
}))
```

**Drag to follow.** `on_mouse_move` fires for every move over the slider, with a `MouseMoveEvent`. Its `pressed_button` says which button is held, if any: only `Some(MouseButton::Left)` means dragging. Act then, and ignore plain moves.

**Live on the bar.** `set_gap_at` takes `x` (the event's `position.x`) and `cx`, and updates the `Settings` global; the bar watches it, so the gap on the bar changes while you drag.

## Your turn

- In `gap_slider`, add a left-button `.on_mouse_down` whose listener calls `this.set_gap_at(e.position.x, cx)`.
- Add `.on_mouse_move` that does the same, but only while the left button is held.
- Save. Press Run, open Appearance and drag the Gap slider.
