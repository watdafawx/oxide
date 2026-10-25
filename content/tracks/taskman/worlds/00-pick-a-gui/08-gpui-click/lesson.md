In egui the CPU button was an `if`. GPUI wants a closure instead, and one more call that egui never needed: `cx.notify()`. The sort is the same as before; only the wiring changes.

**An id first.** A click is a mouse-down and a mouse-up on the same box, so GPUI must recognise the box from one frame to the next. `.id("header-cpu")` gives it that name. The header in the starter already has it; without an id, a box has no `on_click`.

**The callback.** `.on_click(..)` takes the closure GPUI runs when the click lands. `cx.listener(..)` wraps it so the closure gets the view back as its first argument, `list`, ready to change. Its four arguments are the view, the click event, the window and `cx`; this closure needs only the first and the last, so the middle two are `_`. The example sorts by name; yours sorts by CPU.

```rust
.on_click(cx.listener(|list, _, _, cx| {
    list.sort_by_name();
    cx.notify();
}))
```

**Why notify.** GPUI only calls `render` again when it has a reason. `cx.notify()` is the reason: "this view changed". After a click GPUI happens to redraw the window anyway, so a missing notify there can hide. Write it every time you change the view.

**Where it bites.** New numbers arrive once a second with nobody touching the mouse. `set_processes` stores them, and without `cx.notify()` the window keeps showing the old list until something else, like a click, makes GPUI redraw. That is why this bug comes and goes. The test counts the times `render` ran (`renders`) and checks that new numbers made it go up.

## Your turn

1. On the `header` box in `render`, add `.on_click(cx.listener(..))` that calls `list.sort_by_cpu()` and then `cx.notify()`.
2. In `set_processes`, call `cx.notify()` after storing the new list.
3. Save. The `unused variable: cx` warnings go away when you are done.
