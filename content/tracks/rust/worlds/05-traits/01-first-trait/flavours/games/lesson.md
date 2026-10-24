Your server has zombies, creepers and villagers, and the debug overlay must label every one of them. Different structs need the same verb, `describe`. In Rust, a shared verb is a **trait**, and the whole entity system of this world grows from one: `Entity`.

**Try the obvious thing.** You could write `describe_zombie(z: &Zombie)` and `describe_creeper(c: &Creeper)`. That works until the tenth mob, and the overlay, which wants "the label of any mob", still has no type to ask for.

**No parent class.** Many languages solve this with inheritance: a base class `Mob` that `Zombie` and `Creeper` extend. Rust has no classes and no inheritance. A struct holds only its own data, and shared behaviour is a separate promise each type signs up to.

**The promise.** A **trait** is that promise: a list of method signatures (name, parameters, return type) with no bodies. It says "every type that implements me has these methods".

**Keeping it.** `impl Entity for Skeleton { ... }` is where a type keeps the promise: you write the bodies there, and then `skeleton.describe()` works like any method. If you implemented `Display` in Errors, you have done this once already: `Display` is a trait too.

```rust
trait Entity {
    fn describe(&self) -> String;      // signature only: no body
}

struct Skeleton {
    arrows: u32,
}

impl Entity for Skeleton {             // Skeleton keeps the promise
    fn describe(&self) -> String {
        format!("skeleton with {} arrows", self.arrows)
    }
}

let s = Skeleton { arrows: 12 };
let text = s.describe();               // "skeleton with 12 arrows"
```

**Why it beats a family tree.** You can add a trait to a type at any time, without touching the struct, and one mob can sign up for as many traits as it needs. Nothing forces a creeper to be "a kind of" zombie.

## Your turn

- In `impl Entity for Zombie`, return text like `Steve the zombie`.
- In `impl Entity for Creeper`, return text like `creeper (fuse 30)`.
