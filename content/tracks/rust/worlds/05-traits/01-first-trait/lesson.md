A game has dogs, robots and a hundred other things that all need a text label. A **trait** is the promise "every type that implements me has this method".

```rust
trait Describe {
    fn describe(&self) -> String;     // signature only, no body
}

struct Cat { lives: u8 }

impl Describe for Cat {               // "Cat keeps the Describe promise"
    fn describe(&self) -> String {
        format!("cat with {} lives", self.lives)
    }
}

let c = Cat { lives: 9 };
println!("{}", c.describe());         // cat with 9 lives
```

- `trait Name { ... }` lists method **signatures**.
- `impl Trait for Type { ... }` fills in the bodies.
- Then you call it like any method: `value.describe()`.

## Your turn

- Open `src/lib.rs`. The `Describe` trait is already there.
- In `impl Describe for Dog`, return `"<name> the dog"`.
- In `impl Describe for Robot`, return `"robot #<id>"`.
