`impl Trait` is a shorthand: "some type that implements this trait", without naming it. It shines for closures and iterator chains, whose real types are unwritable.

```rust
use std::fmt::Display;

fn show(x: impl Display) -> String {                 // argument: any Display type
    format!("<{x}>")
}

fn counter(limit: u32) -> impl Iterator<Item = u32> { // return: some iterator, not named
    (1..=limit).map(|n| n * 10)
}

fn multiplier(k: i32) -> impl Fn(i32) -> i32 {        // return: a closure
    move |x| x * k
}
// show(5) == "<5>",  counter(2) yields 10, 20,  multiplier(3)(4) == 12
```

- **Argument position**: same as a generic `<T: Display>`, shorter to write.
- **Return position**: you promise a trait, the caller never sees the concrete type.
- A function returning `impl Trait` must return **one** concrete type.
- `move` makes a closure own the variables it uses, needed when returning it.

## Your turn

- Open `src/lib.rs`.
- Implement `shout`, `evens`, `adder` and `compose`. The signatures are given.
