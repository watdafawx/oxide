Time to ship the drawing app's renderer: shapes drawn as ASCII on a canvas. Everything from this world goes in: one trait, three shapes, a list of mixed shapes, generic helpers, and a wrapper that is itself a shape.

```text
..#..
.###.        <- a circle, r = 2, on a 5 x 5 canvas
#####
.###.
..#..
```

**One trait.** `Shape` asks for `area`, `bounds` (the smallest `Rect` around the shape), `name` and `contains(x, y)`. The formula for each shape is in the doc comment above its struct: it is the spec, not the skill.

**Three shapes.** `Rect`, `Circle` and `Triangle` implement it. Start with `Rect`: the first objective is two lines away, and `contains` is four comparisons.

**A canvas of mixed shapes.** `Canvas::render` takes `&[Box<dyn Shape>]`, the trait objects from the mixed-list quest. For each cell it tests the cell's centre: `#` if any shape contains it, `.` if none does. Rows are joined with a newline and there is none at the end.

**Generic helpers.** `total_area`, `largest` and `sort_by_area` take `&[T]` with `T: Shape`: static dispatch, one copy per shape type. `largest` and `sort_by_area` compare `f64` areas the way the sorting quest did.

**A shape made of a shape.** `Scaled<S: Shape>` wraps any shape and grows it around its top-left corner. `impl<S: Shape> Shape for Scaled<S>` makes every scaled shape a shape too, so it fits in the canvas list next to the others.

**The design in one line.** Dynamic dispatch where the types mix, in the canvas; static dispatch where one type repeats, in the helpers. That is the choice from the dispatch read, made twice.

## Your turn

- Work through the objectives top to bottom; save after each one.
- Press Run to see the picture.
