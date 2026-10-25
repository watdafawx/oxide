A widget without a title bar has nothing to grab. Kinetik's widget moves when you drag it, so here the header strip at its top takes over the title bar's job. Windows does the actual moving; your code only says when a drag starts.

**Let Windows drag.** `window.start_window_move()` tells Windows "the user is now dragging this window by its title". From that press on, Windows follows the mouse and moves the window itself, smoothly, until the button comes up.

**Only on a press.** So call it from `on_mouse_down(MouseButton::Left, ..)`, which fires once when the left button goes down. Calling it on every mouse move would start a new drag each time the mouse twitched.

**The listener, again.** The handler is a `cx.listener(|this, event, window, cx| ..)`, the four-input shape from the Interaction world: `this` is the `WidgetView`, the event is the `MouseDownEvent` (not needed here, so `_`), then the `window` to move and `cx` to read settings with `cx.global::<Settings>()`.

**Locked stays put.** The widget's lock setting, `widget.locked` in `Settings`, stops dragging; later quests make a locked widget click-through too. A locked widget returns from the handler at once, before anything moves.

**A swappable call.** GPUI's test platform has no real windows to move, and `start_window_move` panics there. So `WidgetView` keeps the call in a field, `start_move: fn(&mut Window)`, which is a **function pointer**: a field whose value is a function. The app fills it with `|window| window.start_window_move()`; the tests put in one that does nothing, and count `moves_started` instead.

**Calling a field.** `this.start_move(window)` would look for a *method* named `start_move` and fail to compile, because there is none. To call the function stored in the field, wrap the field in parentheses first, then add the arguments. The same rule works for any field holding a function:

```rust
struct Alarm { ring: fn(&str) }

let alarm = Alarm { ring: |text| println!("{text}") };
(alarm.ring)("GPU at 90 °C");   // call the function in the field
```

## Your turn

- In `widget_header`, add a left-button `on_mouse_down` with a `cx.listener`.
- Inside: return when the widget is locked. Otherwise add 1 to `this.moves_started` and call the function in `this.start_move` with the window.
- Save. Press Run and drag the widget by its header.
