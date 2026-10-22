Kinetik's widget "remembers its position": move it to the bottom-left, restart, and it is still there. `WidgetSettings::position` keeps the spot, and `place_widget` turns it into the place the widget opens.

**Saved, or not.** A first start has no saved position: open in the top-right corner, 24 pixels in from the edges. The rest of the time, use where it was left.

**Still on screen.** The saved spot may be on a monitor that is gone, or the resolution may have shrunk. A widget opening off screen is lost until you edit the settings file. Use the saved spot only if a good piece of the widget, 40 by 40 pixels, would be visible:

```rust
place.intersect(&work).is_some_and(|seen| seen.width() >= 40 && seen.height() >= 40)
```

**Asking an Option.** `is_some_and` asks a question of the value inside an `Option` and gives `false` for `None`. No match needed.

**Saved on move.** The boss stores the widget's corner in `WidgetSettings::position` after a drag, so this function always has the latest spot.

## Your turn

- In `place_widget`, if `saved` is `Some((x, y))` and enough of the widget there is on screen, return that rectangle.
- Otherwise give back the top-right corner, 24 px in.
- Save.
