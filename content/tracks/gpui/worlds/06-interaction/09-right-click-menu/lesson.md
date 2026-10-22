Right-click Kinetik's bar and a menu opens where you clicked. Yours has two rows: Settings… and Exit. The menu is part of the bar's own window, drawn over the pills. Two GPUI pieces put it in the right place and on top.

**Where you clicked.** `.on_mouse_down(MouseButton::Right, ..)` runs on the right button only. The `MouseDownEvent` carries `position`; store it in `ui.menu_at`. `Some` means the menu is open there.

**Anchored, on top.** `anchored().position(at)` places its child at a point instead of in the layout. `deferred(..)` draws it after everything else in the window, so no pill can cover it:

```rust
.when_some(this.ui.menu_at, |el, at| el.child(deferred(anchored().position(at).child(menu(cx)))))
```

**Close by clicking away.** `menu` (written below) uses `on_mouse_down_out`: a mouse-down anywhere outside the menu sets `menu_at` back to `None`. That is how every desktop menu behaves.

**Inside the window.** The menu is drawn inside the bar's own window. Docked on the taskbar that window is only as tall as the bar, so the menu would be cut off there; this world runs the bar in a normal window, and a pop-up window for the menu is on the plan's list for later.

## Your turn

- In `interactive_pill`, add the right-button `.on_mouse_down` that stores `event.position` in `this.ui.menu_at`.
- In `bar_layer`, add `.when_some(this.ui.menu_at, ..)` with the `deferred(anchored()..)` child.
- Save. Press Run and right-click a pill.
