**Why now:** spawn rules are tiny functions you build at runtime: "add 5 damage", "then triple it". Their real closure types are unwritable. `impl Trait` is the shorthand: "some type that implements this trait", without naming it.

```rust
use std::fmt::Display;

fn tag(x: impl Display) -> String {                  // argument: any Display type
    format!("<{x}>")
}

fn night_ticks(limit: u32) -> impl Iterator<Item = u32> { // return: some iterator, not named
    (1..=limit).map(|n| n * 10)
}

fn armor(k: u32) -> impl Fn(u32) -> u32 {            // return: a closure
    move |dmg| dmg.saturating_sub(k)
}
// tag(5) == "<5>",  night_ticks(2) yields 10, 20,  armor(3)(4) == 1
```

- **Argument position**: same as a generic `<T: Display>`, shorter to write.
- **Return position**: you promise a trait, the caller never sees the concrete type.
- A function returning `impl Trait` must return **one** concrete type.
- `move` makes a closure own the variables it uses, needed when returning it.

## Your turn

- Open `src/lib.rs`.
- Implement `banner`, `spawn_ticks`, `damage_bonus` and `chain`. The signatures are given.
