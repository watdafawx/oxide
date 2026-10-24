The tick loop needs **one** list of every mob in the world: zombies, creepers and villagers, side by side. A `Vec<T>` holds a single `T`, and generics pick that `T` once. This quest gets past that wall.

**Try the obvious thing.** `vec![Zombie { hp: 20 }, Creeper { hp: 15, fuse: 30 }]` stops the build:

```text
error[E0308]: mismatched types
    expected `Zombie`, found `Creeper`
```

**Why one type.** A `Vec` stores its items side by side in memory, every slot the same size. A `Zombie` is one `u32` (4 bytes), a `Creeper` is two (8 bytes), so they can't share slots.

**A handle of fixed size.** `Box::new(value)` moves a value to a separate area of memory, the **heap**, and gives you a `Box`: a small handle that says where the value is. Every `Box` is the same size whatever it holds. You met `Box<dyn Error>` in Errors; it is the same `Box`.

**Keep only the promise.** **`dyn Entity`** means "some type that implements `Entity`, never mind which". A `Box<dyn Entity>` is a **trait object**: the concrete type is forgotten, and only the trait's methods can be called on it.

```rust
trait Entity {
    fn health(&self) -> u32;
}
struct Bat;
struct Skeleton(u32);
impl Entity for Bat { fn health(&self) -> u32 { 6 } }
impl Entity for Skeleton { fn health(&self) -> u32 { self.0 } }

let cave: Vec<Box<dyn Entity>> = vec![Box::new(Bat), Box::new(Skeleton(20))];
for mob in &cave {
    println!("{}", mob.health());
}
```

**Each call finds its own body.** `mob.health()` runs the bat's `health` for the bat and the skeleton's for the skeleton. How Rust finds the right one is a short read two quests from now.

## Your turn

- In `spawn_wave`, return a zombie with 20 hp and a creeper with 15 hp and fuse 30, each boxed, in that order.
- In `total_health`, add up `health()` of every boxed mob.
