Kinetik's widget remembers its position: move it to the bottom-left, restart, and it is still there. `WidgetSettings::position` keeps the spot as `Option<(i32, i32)>`, its top-left corner, and `place_widget` turns that into the rectangle the widget opens in.

**Saved, or not.** A first start has no saved position, so `saved` is `None`: open in the top-right corner, 24 pixels in from the edges. Otherwise, open where it was left.

**Still on screen.** The saved spot may be on a monitor that is unplugged now, or the resolution may have shrunk. A widget that opens off screen is lost until you edit the settings file by hand. So use the saved spot only if a good piece of the widget, 40 by 40 pixels, would be visible.

**Overlap.** `place.intersect(&work)` from the Taskbar world gives the part of the widget's rectangle that is on the work area: `Some(rect)` if they overlap, `None` if the widget would be completely off screen.

**Ask the Option.** `is_some_and(|x| ..)` asks a question of the value inside an `Option`: `true` only if there is a value and the closure says yes; `None` is always `false`. One call instead of a `match`. Here it checks a saved gap from the settings:

```rust
let usable = saved_gap.is_some_and(|gap| gap >= 2 && gap <= 32);
```

**Saved on move.** The boss stores the widget's corner in `WidgetSettings::position` after a drag, so this function always has the latest spot.

## Your turn

- In `place_widget`, if `saved` is `Some((x, y))`, build the widget's rectangle there. If its overlap with `work` is at least 40 by 40, return it.
- Otherwise give back the top-right corner, 24 px in from the right and the top.
