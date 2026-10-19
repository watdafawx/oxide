**Why now:** your boss fight spawns a wave of zombies, then a wave of creepers. "Which one has the most health?" is the same code for both. A **generic** function takes a type parameter `T` and works for every mob that meets its **bounds**: `T: Entity`.

```rust
trait Entity { fn health(&self) -> u32; }

fn weakest<T: Entity>(mobs: &[T]) -> &T {      // any T that is an Entity
    let mut low = &mobs[0];
    for m in mobs {
        if m.health() < low.health() { low = m; }
    }
    low
}

fn report<A, B>(a: &A, b: &B) -> String        // `where` moves long bounds out of the way
where
    A: Entity,
    B: Entity,
{
    format!("{} vs {}", a.health(), b.health())
}
```

- `<T: Bound>` says "any type `T`, as long as it implements `Bound`".
- Combine bounds with `+`: `T: Entity + Clone`.
- Without the bound the compiler refuses `m.health()`: it cannot know `T` has it.
- Prefer a **`where`** clause when the bounds get long.

## Your turn

- Open `src/lib.rs`. The mobs and the `Entity` trait are given.
- Fill in `strongest`, `roster` and `total_health`. The signatures and bounds are given.
