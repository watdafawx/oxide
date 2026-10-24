A drawing has circles **and** rectangles, and the canvas wants one list of everything, in drawing order. A `Vec<T>` holds a single `T`, and generics pick that `T` once. This quest gets past that wall.

**Try the obvious thing.** `vec![Circle { r: 1.0 }, Rect { w: 2.0, h: 3.0 }]` stops the build:

```text
error[E0308]: mismatched types
    expected `Circle`, found `Rect`
```

**Why one type.** A `Vec` stores its items side by side in memory, every slot the same size. A `Circle` is one `f64` (8 bytes), a `Rect` is two (16 bytes), so they can't share slots.

**A handle of fixed size.** `Box::new(value)` moves a value to a separate area of memory, the **heap**, and gives you a `Box`: a small handle that says where the value is. Every `Box` is the same size whatever it holds. You met `Box<dyn Error>` in Errors; it is the same `Box`.

**Keep only the promise.** **`dyn Shape`** means "some type that implements `Shape`, never mind which". A `Box<dyn Shape>` is a **trait object**: the concrete type is forgotten, and only the trait's methods can be called on it.

```rust
trait Shape {
    fn area(&self) -> f64;
}
struct Dot;
struct Line(f64);
impl Shape for Dot { fn area(&self) -> f64 { 0.0 } }
impl Shape for Line { fn area(&self) -> f64 { 0.0 } }

let marks: Vec<Box<dyn Shape>> = vec![Box::new(Dot), Box::new(Line(4.0))];
for m in &marks {
    println!("{}", m.area());
}
```

**Each call finds its own body.** `m.area()` runs the dot's `area` for the dot and the line's for the line. How Rust finds the right one is a short read two quests from now.

## Your turn

- In `make_shapes`, return a circle with `r = 1.0` and a 2 by 3 rect, each boxed, in that order.
- In `total_area`, add up `area()` of every boxed shape.
