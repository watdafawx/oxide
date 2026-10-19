Two ways to say "any type that implements `Shape`". They look similar and behave very differently.

## Generics: static dispatch

```rust
fn area_of<T: Shape>(s: &T) -> f64 { s.area() }
```

The compiler **monomorphises**: it writes a separate `area_of` for every `T` you use. Calls are direct, can be inlined, and cost nothing at runtime. The price is more compiled code, and every `T` in one collection must be the **same** type.

## `dyn Trait`: dynamic dispatch

```rust
fn area_of(s: &dyn Shape) -> f64 { s.area() }
let shapes: Vec<Box<dyn Shape>> = vec![Box::new(Circle), Box::new(Rect)];
```

A `&dyn Shape` is a **fat pointer**: the data plus a pointer to a **vtable** (a table of that type's methods). Each call looks the method up at runtime. A tiny cost, but one function, one `Vec`, many types.

## Rule of thumb

- Default to **generics**. Fast, simple.
- Reach for **`dyn`** when types are mixed in one collection or only known at runtime (plugins, UI widgets, enemies in a game).
- `dyn` needs a pointer (`&`, `Box`, `Rc`) because its size is unknown.

## Your turn

Answer the two questions below.
