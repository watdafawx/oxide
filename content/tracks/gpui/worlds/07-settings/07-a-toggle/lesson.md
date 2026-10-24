The Stats page lists every stat with a switch: on puts it on the bar, off takes it away. A switch is a small thing drawn the same way many times, which is what a component is for.

**What a toggle is.** A rounded track, blue when on and grey when off, with a white knob at the right or the left end. `Toggle` holds its id, whether it is on, and `on_toggle`: what to do when clicked.

**The look is written.** The starter draws it: `.p(px(2.))` is padding on every side, `.size(px(16.))` sets width and height at once, `.rounded_full()` rounds the ends into a pill, and `.when(on, |el| el.justify_end())` pushes the knob to the right end when on.

**It does not decide.** The toggle knows nothing about stats or settings. The Stats page passes a closure that calls `set_stat_on` on the global's order. The same toggle could switch "two lines" or "all taskbars" with a different closure.

**Click flips.** A click hands `!on` to `on_toggle`: an off toggle asks to be on. As the last quest showed, this is a plain closure with the event, the window and `&mut App`, and it passes the last two on. A button component with an `on_press` handler would do the same:

```rust
.on_click(move |_, window, cx| on_press(window, cx))
```

**Calling the field.** `on_toggle` was taken out of `self` into a local, so it is called like a function: `on_toggle(..)`. The closure takes it, and the bool `on`, with `move`.

## Your turn

- At the end of the chain in `Toggle::render`, add `.on_click(..)` with a plain `move` closure.
- In it, call `on_toggle` with the opposite of `on`, the window and `cx`.
- Save. Press Run and switch Battery on.
