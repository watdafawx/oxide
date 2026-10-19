**Why now:** a hundred mob types, and 90 of them are peaceful. Writing `is_hostile() -> false` into every impl is silly. A **default method** gives every mob the normal behaviour for free; only the zombie overrides it.

```rust
trait Entity {
    fn name(&self) -> String;                  // required
    fn is_hostile(&self) -> bool { false }     // default: free for everyone
    fn label(&self) -> String {
        format!("[{}]", self.name())           // defaults can call other methods
    }
}

struct Sheep;
impl Entity for Sheep {
    fn name(&self) -> String { "Sheep".to_string() }
    // is_hostile() and label() not written: Sheep gets the defaults
}

struct Creeper;
impl Entity for Creeper {
    fn name(&self) -> String { "Creeper".to_string() }
    fn is_hostile(&self) -> bool { true }      // override
}
```

- A default body can call the **other** methods of the trait.
- A type may **override** any default by writing the method itself.

## Your turn

- Open `src/lib.rs`.
- In `trait Entity`, write the defaults for `is_hostile`, `announce` and `alert`.
- In `impl Entity for Zombie`, override `is_hostile` and `announce`.
