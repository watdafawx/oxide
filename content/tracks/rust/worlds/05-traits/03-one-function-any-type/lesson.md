The drawing has a row of circles, and on another layer a row of squares. "Add up their areas" is the same loop for both rows. A **generic** function lets you write it once.

**Two copies of one loop.** `total_area_circles(&[Circle])` and `total_area_squares(&[Square])` would have the same body, word for word. Only the type in the signature changes, and every new shape would need a third copy.

**A type as a parameter.** A **generic** function takes a type as a parameter, written in angle brackets after its name: `fn total_area<T>(...)`. `T` is a placeholder that each call fills in: `Circle` for one row, `Square` for the other.

**Say what T can do.** The body calls `area()` on a `T`. Rust only allows that if every possible `T` has it, so you add a **bound**: `T: Shape`, which reads "any type, as long as it implements `Shape`".

```rust
trait Shape {
    fn area(&self) -> f64;
}

fn smallest_area<T: Shape>(shapes: &[T]) -> f64 {
    let mut low = shapes[0].area();
    for s in shapes {
        if s.area() < low {
            low = s.area();
        }
    }
    low
}
```

**Checked once.** The compiler checks the body against the bound, not against each caller. Pass a slice of something that is not a `Shape` and the error points at your call, not into the function.

**What it costs.** Nothing while the program runs: the compiler quietly writes one copy of the function per type you use, as if you had written both by hand. You will meet the other way, one copy for every type, in a few quests.

## Your turn

- Write `total_area`: add up `area()` of every shape in the slice.
- Write `biggest`: return a reference to the shape with the largest area. Ties go to the first.
