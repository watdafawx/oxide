Most implementors want the same behaviour. A trait method with a **default body** saves them from copy-pasting it.

```rust
trait Speaker {
    fn name(&self) -> String;                  // required
    fn speak(&self) -> String {                // default: free for everyone
        format!("{} says hi", self.name())
    }
}

struct Bob;
impl Speaker for Bob {
    fn name(&self) -> String { "Bob".to_string() }
    // speak() not written: Bob gets the default
}

struct Loud;
impl Speaker for Loud {
    fn name(&self) -> String { "Loud".to_string() }
    fn speak(&self) -> String { "HEY".to_string() }   // override
}
```

- A default body can call the trait's **other** methods.
- A type may **override** any default by writing the method itself.

## Your turn

- Open `src/lib.rs`.
- In `trait Greeter`, write the default `greet` and `shout`.
- In `impl Greeter for Parrot`, override `greet`.
