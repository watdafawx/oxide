Time to ship the debug overlay of your entity system: every mob has a **hitbox**, and you want to see them as ASCII, F3+B style. Everything from this world goes in: one trait, three hitbox shapes, a list of mixed hitboxes, generic helpers, and a wrapper that is itself a hitbox.

```text
..#..
.###.        <- a cylinder hitbox, r = 2, seen from above on a 5 x 5 canvas
#####
.###.
..#..
```

**One trait.** `Hitbox` asks for `area`, `bounds` (the smallest `Aabb` around it), `name` and `contains(x, y)`. The formula for each shape is in the doc comment above its struct: it is the spec, not the skill.

**Three shapes.** `Aabb` (a box), `Cylinder` (a circle from above) and `Wedge` (a ramp) implement it. Start with `Aabb`: the first objective is two lines away, and `contains` is four comparisons.

**A canvas of mixed hitboxes.** `Canvas::render` takes `&[Box<dyn Hitbox>]`, the trait objects from the mob-list quest. For each cell it tests the cell's centre: `#` if any hitbox contains it, `.` if none does. Rows are joined with a newline and there is none at the end.

**Generic helpers.** `total_area`, `biggest` and `sort_by_area` take `&[T]` with `T: Hitbox`: static dispatch, one copy per hitbox type. `biggest` and `sort_by_area` compare `f64` areas the way the sorting quest did.

**A hitbox made of a hitbox.** `Scaled<H: Hitbox>` wraps any hitbox and grows it around its top-left corner, for baby zombies and giants. `impl<H: Hitbox> Hitbox for Scaled<H>` makes every scaled hitbox a hitbox too, so it fits in the canvas list next to the others.

**The design in one line.** Dynamic dispatch where the types mix, in the canvas; static dispatch where one type repeats, in the helpers. That is the choice from the dispatch read, made twice.

## Your turn

- Work through the objectives top to bottom; save after each one.
- Press Run to see the picture.
