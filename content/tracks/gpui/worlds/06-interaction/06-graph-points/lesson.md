The card needs a graph. Before anything is drawn, each reading becomes a point in the graph's box. That is pure maths, no window, so it is easy to get exactly right with tests.

**Left to right.** 60 readings spread over the box's width: the oldest at the left edge, the newest at the right. Reading `i` of `n` sits at `i / (n - 1)` of the way across. `last` is `n - 1`, never below 1, so a single reading does not divide by zero.

**Up is smaller.** On screen, y grows downward, so the top edge has the smallest y. A value's **share** is where it sits between `min` and `max`, from 0 at the lowest to 1 at the highest. Go up from the bottom edge by that share of the height. A line at 80 % of the height, for example, sits here:

```rust
let y = bounds.bottom() - bounds.size.height * 0.8;
```

**A flat line.** If every reading is the same, `max - min` is 0 and the share would divide by zero. Put a flat line in the middle: share 0.5.

**Pixels are numbers.** `bounds.left()`, `bounds.bottom()` and `bounds.size.width` are `Pixels`. A `Pixels` can be multiplied by an `f32` and added to another `Pixels`, so the maths reads like plain arithmetic, and `point(x, y)` makes the point.

## Your turn

- In `graph_points`, inside the `map`, work out `x` from `i / last` across the width.
- Work out `share` (0.5 for a flat line) and `y` up from the bottom, and give back `point(x, y)`.
- Save.
