Boxes and text cannot draw a line graph. For that GPUI has `canvas`: an element whose box is laid out like any other, but whose inside you paint yourself.

**Two closures.** `canvas(prepaint, paint)`: the first can prepare data once the box's size is known; the second gets the final `bounds` and the `window` and paints. The graph needs only the second.

**A path.** `PathBuilder::stroke(px(1.5))` starts a line 1.5 pixels wide. `move_to` the first point, `line_to` each next one, `build()`, and `window.paint_path(path, colour)` draws it. `graph` below does all of that with your `graph_points`.

**Give it a box.** The canvas fills a `div` 48 pixels tall and as wide as the card. The card's `render` adds it between the title and the summary:

```rust
.child(graph(self.points.clone()))
```

**Drawn every frame.** Painting runs on every frame the card is shown. That is cheap here: 60 points, one path.

## Your turn

- In `HoverCard`'s `render`, add `.child(graph(self.points.clone()))` between the title and the summary.
- Save. Press Run and rest the mouse on CPU: the last minute, as a line.
