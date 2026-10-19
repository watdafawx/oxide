**Why now:** your server has zombies, creepers and villagers, and the debug overlay must label every one of them. The **Entity** trait is the promise "every mob has a `describe()`". Everything else in this world grows from it.

```rust
trait Entity {
    fn describe(&self) -> String;     // signature only, no body
}

struct Skeleton { arrows: u32 }

impl Entity for Skeleton {            // "Skeleton keeps the Entity promise"
    fn describe(&self) -> String {
        format!("skeleton with {} arrows", self.arrows)
    }
}

let s = Skeleton { arrows: 12 };
println!("{}", s.describe());         // skeleton with 12 arrows
```

- `trait Name { ... }` lists method **signatures**.
- `impl Trait for Type { ... }` fills in the bodies.
- Then you call it like any method: `mob.describe()`.

## Your turn

- Open `src/lib.rs`. The `Entity` trait is already there.
- In `impl Entity for Zombie`, return `"<name> the zombie"`.
- In `impl Entity for Creeper`, return `"creeper (fuse <fuse>)"`.
