**Why now:** time to ship the debug overlay of your entity system: every mob has a **hitbox**, and you want to see them as ASCII, F3+B style. One trait, many shapes, one renderer.

```text
..#..
.###.        <- a cylinder hitbox, r = 2, seen from above on a 5 x 5 canvas
#####
.###.
..#..
```

## The design

- **`trait Hitbox`**: `area`, `bounds -> Aabb`, `name`, `contains(x, y)`.
- `Aabb` (a box), `Cylinder` (a circle from above) and `Wedge` (a ramp) implement it. `Wedge` has a right angle at `(x, y)`, legs along `+x` and `+y`.
- **`Canvas { w, h }`** draws a `&[Box<dyn Hitbox>]`: for each cell, test its **centre** `(col + 0.5, row + 0.5)`. `#` if any hitbox contains it, else `.`. Rows are joined by `\n`.
- `total_area`, `biggest`, `sort_by_area` are **generic** over `T: Hitbox`.
- **`Scaled<H: Hitbox>`** wraps any hitbox and scales it about its top-left corner (a baby zombie, a giant). It is a `Hitbox` too.

- **Tools:** `items.iter().max_by(|a, b| a.weight.total_cmp(&b.weight))` picks the biggest by an `f64` key (`max_by_key` cannot take floats). `items.sort_by(|a, b| a.weight.total_cmp(&b.weight))` sorts with the same closure. `iter().any(|s| s.is_big())` asks whether any item matches. `col as f64` turns a `usize` into an `f64`. `std::f64::consts::PI` is π.

## Your turn

- Open `src/lib.rs`. Types and signatures are given. Every method is a `TODO`.
- Work in the order of the objectives. `Aabb` first: the first tick is two lines away.
- Dynamic dispatch for the canvas, static dispatch for the helpers: that is the point.

Press Run to play it.
