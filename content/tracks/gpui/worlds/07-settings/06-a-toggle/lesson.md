The Stats page lists every stat with a switch: on puts it on the bar, off takes it away. A switch is a small thing drawn the same way many times, which is what a component is for.

**What a toggle is.** A rounded track, blue when on and grey when off, with a white knob at the right or the left end. `Toggle` holds its id, whether it is on, and `on_toggle`: what to do when clicked.

**It does not decide.** The toggle knows nothing about stats or settings. The Stats page passes a closure that calls `set_stat_on` on the global's order. The same toggle will switch "two lines" or "all taskbars" later with a different closure.

**Knob to the end.** The track is a flex row; `.justify_end()` pushes the knob to the right. `.when(on, ..)` adds it only when on:

```rust
.when(on, |el| el.justify_end())
```

**Click flips.** The click hands `!on` to `on_toggle`: an off toggle asks to be on. The global changes, and both the toggle and the bar redraw from it.

## Your turn

- In `Toggle::render`, replace the TODO with the track: id and name, size, padding, `flex`, `rounded_full`, the colour by `on`, `justify_end` when on, the knob child.
- Add `.on_click(move |_, window, cx| on_toggle(!on, window, cx))`.
- Save. Press Run and switch Battery on.
