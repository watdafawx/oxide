A hundred mob types, and ninety of them are peaceful. Writing `is_hostile() -> false` into every `impl` is copy-paste, and copies drift apart. A trait method can carry a **default body** instead: every mob gets the normal behaviour, and only the zombie says otherwise.

**Required and default.** A method with only a signature is **required**: every mob must write it. A method with a body in the trait is a **default**: a mob that says nothing gets that body for free.

**Defaults call the others.** A default body may call the trait's other methods through `self`, even required ones whose bodies it has never seen. Each mob fills in `name`, and the shared `label` uses it.

```rust
trait Entity {
    fn name(&self) -> String;                  // required
    fn is_hostile(&self) -> bool { false }     // default
    fn label(&self) -> String {                // default that calls name()
        format!("[{}]", self.name())
    }
}

struct Sheep;
impl Entity for Sheep {
    fn name(&self) -> String { "Sheep".to_string() }
    // no is_hostile(), no label(): Sheep gets the defaults
}

struct Creeper;
impl Entity for Creeper {
    fn name(&self) -> String { "Creeper".to_string() }
    fn is_hostile(&self) -> bool { true }      // override
}
```

**Override.** A mob that needs something different writes the method itself in its `impl`, and that version wins, for that mob only.

**The mob's own version.** When a default calls `self.announce()`, it gets the mob's own `announce`, override included. That is why an `alert` written once still shouts a zombie's line.

## Your turn

- In `trait Entity`, write the defaults: `is_hostile` is `false`, `announce` is `A wild <name> appears!`, `alert` is `announce` in uppercase.
- In `impl Entity for Zombie`, override `is_hostile` (zombies are hostile) and `announce` with `<name> groans at you!`.
