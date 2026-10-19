**Why now:** the tick loop needs **one** list holding every mob in the world: zombies, creepers, villagers. A `Vec<Zombie>` only holds zombies. A **trait object** erases the concrete type and keeps only "it is an `Entity`".

```rust
trait Entity { fn health(&self) -> u32; }
struct Skeleton(u32);
struct Bat;
impl Entity for Skeleton { fn health(&self) -> u32 { self.0 } }
impl Entity for Bat      { fn health(&self) -> u32 { 6 } }

let world: Vec<Box<dyn Entity>> = vec![Box::new(Skeleton(20)), Box::new(Bat)];
let total: u32 = world.iter().map(|m| m.health()).sum();   // 26

fn show(m: &dyn Entity) -> u32 { m.health() }              // borrowed trait object
show(&Skeleton(3));
```

- **`Box<dyn Entity>`**: owned, lives on the heap, any `Entity` goes in.
- **`&dyn Entity`**: a borrowed trait object, no allocation.
- Methods are found through the **vtable** when you call them.

## Your turn

- Open `src/lib.rs`.
- Write `health` for `Zombie` and `Creeper`.
- Fill in `spawn_wave`, `total_health` and `describe`.
