The card needs a graph. Before anything is drawn, each reading becomes a point in the graph's box. Pure maths, no window: easy to get exactly right with tests.

**Left to right.** 60 readings spread over the box's width: the oldest at the left edge, the newest at the right. Reading `i` of `n` sits at `i / (n - 1)` of the way across. `last` is `n - 1`, never below 1, so a single reading does not divide by zero.

**Up is smaller.** On screen, y grows downward. The highest value goes at the top, so its y is the smallest. Work out the value's share between `min` and `max`, from 0 to 1, and go up from the bottom by that share of the height:

```rust
let y = bounds.bottom() - bounds.size.height * share;
```

**A flat line.** If every reading is the same, `max - min` is 0 and the share would divide by zero. Put a flat line in the middle: share 0.5.

**Pixels are numbers.** `Pixels` can be multiplied by an `f32` and added to another `Pixels`, so the maths reads like plain arithmetic.

## Your turn

- In `graph_points`, inside the `map`, work out `x` from `i / last` across the width.
- Work out `share` (0.5 for a flat line) and `y` up from the bottom, and give back `point(x, y)`.
- Save.
