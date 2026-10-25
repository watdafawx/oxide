The menu appears whenever `menu_at` holds a point. Now a right click on a pill has to put the point there.

**Right button only.** `.on_click` is for the left button. `.on_mouse_down(MouseButton::Right, ..)` runs its listener when the right button goes down over the element, and ignores the other buttons. Menus open on the press, not on the release, so `on_mouse_down` fits.

**The event, used.** This time the listener needs its event input. A `MouseDownEvent` carries `position`, where the mouse was in the window, which is exactly where the menu should open. Name the input with its type so you can use it: `event: &MouseDownEvent`. A listener that only reported middle-button presses would read:

```rust
.on_mouse_down(MouseButton::Middle, cx.listener(|_, event: &MouseDownEvent, _, _| {
    println!("middle press at {:?}", event.position);
}))
```

**Closing is written.** `menu` uses `on_mouse_down_out`: a press anywhere outside the menu sets `menu_at` back to `None`. That is how every desktop menu closes.

**No `move` needed.** This closure uses only its inputs, nothing from `interactive_pill`, so it has nothing to take along.

## Your turn

- In `interactive_pill`, add `.on_mouse_down(MouseButton::Right, ..)` with a listener that names its event.
- In it, store `Some(event.position)` in `this.ui.menu_at` and notify.
- Save. Press Run and right-click a pill.
