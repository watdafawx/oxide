Right-click Kinetik's bar and a menu opens where you clicked, with two rows: Settings… and Exit. `menu(cx)` already builds it. This quest draws it while `ui.menu_at` holds a point; the next one sets that point on a right click.

**Only while open.** `ui.menu_at` is an `Option<Point<Pixels>>`: `Some(point)` means the menu is open there, `None` means closed. `.when(..)` from Views adds something for a `bool`. Its sibling `.when_some(option, |el, value| ..)` runs the closure only for `Some`, and hands it the value inside. With `None` the element comes back unchanged.

**At a point.** Normal children are placed by the layout, one after another. `anchored().position(at)` takes its child out of the layout and puts the child's top-left corner at a point in the window.

**On top.** GPUI paints elements in order, and what is painted later covers what came before, so a later pill would cover the menu. `deferred(..)` waits to paint its child until everything else in the window is done, so nothing covers it.

**The same shape.** A label that appears only while a stat is hovered, showing which one, would be:

```rust
.when_some(this.ui.hovered, |el, kind| el.child(format!("pointing at {}", kind.label())))
```

**Inside the window.** The menu lives inside the bar's own window. This world runs the bar in a normal window; docked on the taskbar, a window only as tall as the bar would cut the menu off. A separate pop-up window would fix that, but it is out of scope here.

## Your turn

- In `bar_layer`, add `.when_some(this.ui.menu_at, |el, at| ..)` that adds one child.
- That child is `menu(cx)`, placed with `anchored().position(at)`, inside `deferred(..)`.
- Save.
