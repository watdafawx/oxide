A `Vec<T>` holds one type. But a drawing has circles **and** rects. A **trait object** erases the concrete type and keeps only "it implements `Shape`".

```rust
trait Shape { fn area(&self) -> f64; }
struct Sq(f64);
struct Dot;
impl Shape for Sq  { fn area(&self) -> f64 { self.0 * self.0 } }
impl Shape for Dot { fn area(&self) -> f64 { 0.0 } }

let shapes: Vec<Box<dyn Shape>> = vec![Box::new(Sq(2.0)), Box::new(Dot)];
let total: f64 = shapes.iter().map(|s| s.area()).sum();   // 4.0

fn show(s: &dyn Shape) -> f64 { s.area() }                // borrowed trait object
show(&Sq(3.0));
```

- **`Box<dyn Shape>`**: owned, lives on the heap, any `Shape` goes in.
- **`&dyn Shape`**: a borrowed trait object, no allocation.
- Methods are found through the **vtable** when you call them.

## Your turn

- Open `src/lib.rs`.
- Write `area` for `Circle` and `Rect`.
- Fill in `make_shapes`, `total_area` and `describe`.
