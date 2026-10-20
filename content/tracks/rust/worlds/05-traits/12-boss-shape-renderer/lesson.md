Time to put it all together: a tiny renderer that draws shapes as ASCII.

```text
.....
..#..        <- a circle, r = 2, on a 5 x 5 canvas
.###.
#####
.###.
..#..
```

## The design

- **`trait Shape`**: `area`, `bounds -> Rect`, `name`, `contains(x, y)`.
- `Rect`, `Circle` and `Triangle` implement it. `Triangle` has a right angle at `(x, y)`, legs along `+x` and `+y`.
- **`Canvas { w, h }`** draws a `&[Box<dyn Shape>]`: for each cell, test its **centre** `(col + 0.5, row + 0.5)`. `#` if any shape contains it, else `.`. Rows are joined by `\n`.
- `total_area`, `largest`, `sort_by_area` are **generic** over `T: Shape`.
- **`Scaled<S: Shape>`** wraps any shape and scales it about its top-left corner. It is a `Shape` too.

- **Tools:** `items.iter().max_by(|a, b| a.weight.total_cmp(&b.weight))` picks the biggest by an `f64` key (`max_by_key` cannot take floats). `items.sort_by(|a, b| a.weight.total_cmp(&b.weight))` sorts with the same closure. `iter().any(|s| s.is_big())` asks whether any item matches. `col as f64` turns a `usize` into an `f64`. `std::f64::consts::PI` is π.

## Your turn

- Open `src/lib.rs`. Types and signatures are given. Every method is `todo!()`.
- Work in the order of the objectives. `Rect` first: the first tick is two lines away.
- Dynamic dispatch for the canvas, static dispatch for the helpers: that is the point.

Press Run to play it.
