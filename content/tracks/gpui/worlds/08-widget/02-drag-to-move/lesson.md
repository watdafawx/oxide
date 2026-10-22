A widget without a title bar has nothing to grab. Kinetik's widget moves when you drag anywhere on it. Here the header strip does that job, and Windows does the actual moving.

**Let Windows drag.** `window.start_window_move()` tells Windows "the user is now dragging this window by its title". From the press on, Windows follows the mouse and moves the window itself, smoothly, until the button comes up.

**Only on a press.** Call it from `on_mouse_down` with the left button. Calling it on a move would start a new drag on every mouse movement.

**Locked stays put.** The widget's lock setting stops dragging; the next quests make it click-through too. A locked widget returns at once:

```rust
if cx.global::<Settings>().widget.locked { return; }
```

**A swappable call.** GPUI's test platform has no real windows to move, and `start_window_move` panics there. So the view calls it through `start_move`, a function field the tests replace, and counts `moves_started`.

## Your turn

- In `widget_header`, add `.on_mouse_down(MouseButton::Left, cx.listener(..))`.
- Inside: return when locked; otherwise add 1 to `this.moves_started` and call `(this.start_move)(window)`.
- Save. Press Run and drag the widget by its header.
