The boss fight spawns a wave of zombies, then a wave of creepers. "How much health is left in this wave?" is the same loop for both. A **generic** function lets you write it once.

**Two copies of one loop.** `total_health_zombies(&[Zombie])` and `total_health_creepers(&[Creeper])` would have the same body, word for word. Only the type in the signature changes, and every new mob would need a third copy.

**A type as a parameter.** A **generic** function takes a type as a parameter, written in angle brackets after its name: `fn total_health<T>(...)`. `T` is a placeholder that each call fills in: `Zombie` for one wave, `Creeper` for the other.

**Say what T can do.** The body calls `health()` on a `T`. Rust only allows that if every possible `T` has it, so you add a **bound**: `T: Entity`, which reads "any type, as long as it implements `Entity`".

```rust
trait Entity {
    fn health(&self) -> u32;
}

fn lowest_health<T: Entity>(mobs: &[T]) -> u32 {
    let mut low = mobs[0].health();
    for m in mobs {
        if m.health() < low {
            low = m.health();
        }
    }
    low
}
```

**Checked once.** The compiler checks the body against the bound, not against each caller. Pass a slice of something that is not an `Entity` and the error points at your call, not into the function.

**What it costs.** Nothing while the game runs: the compiler quietly writes one copy of the function per mob type you use, as if you had written both by hand. You will meet the other way, one copy for every type, in a few quests.

## Your turn

- Write `total_health`: add up `health()` of every mob in the slice.
- Write `strongest`: return a reference to the mob with the most health. Ties go to the first.
