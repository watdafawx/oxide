Boxes and text cannot draw a line graph. For that GPUI has `canvas`: an element whose box is laid out like any other, but whose inside you paint yourself. The card already places a `graph` 48 pixels tall between its title and summary; its paint step is still empty.

**Two closures.** `canvas(prepaint, paint)`: the first can prepare data once the box's size is known; the second gets the final `bounds` and the `window`, and paints. The graph needs only the second. Inside it, `graph_points` from the last quest already turns the values into `points` in that box.

**A pen.** `PathBuilder::stroke(px(1.5))` starts a line 1.5 pixels wide, like picking up a pen. `move_to(point)` puts the pen down at a point without drawing; `line_to(point)` draws a straight piece from where the pen is to the next point.

**Build, then paint.** `build()` turns the builder into a path. It gives a `Result`, because some paths cannot be built (one with no points, say), so use it only when it is `Ok`. `window.paint_path(path, colour)` draws it. A grey line straight across the middle of the box would be:

```rust
let mut line = PathBuilder::stroke(px(1.));
line.move_to(point(bounds.left(), bounds.center().y));
line.line_to(point(bounds.right(), bounds.center().y));
if let Ok(path) = line.build() {
    window.paint_path(path, rgb(0x555555));
}
```

**Many points.** The graph has up to 60 points instead of two. Loop over them with `enumerate()`: the first one (`i == 0`) is a `move_to`, every other one a `line_to`.

**Drawn every frame.** Painting runs on every frame the card is shown. That is cheap here: 60 points, one path.

## Your turn

- In `graph`, inside the paint closure, replace the TODO and its `todo!()`.
- Start a 1.5-pixel line, move to the first point, and draw a line to each next one.
- Build it, and when that works, paint it in `rgb(0x5ab0ff)`. Save, press Run and rest the mouse on CPU.
