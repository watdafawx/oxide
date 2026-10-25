Most greeters say hello the same way. Writing the same `greet` body into every `impl` is copy-paste, and copies drift apart. A trait method can carry a **default body** instead.

**Required and default.** A method with only a signature is **required**: every type must write it. A method with a body in the trait is a **default**: a type that says nothing gets that body for free.

**Defaults call the others.** A default body may call the trait's other methods through `self`, even required ones whose bodies it has never seen. Each type fills in `name`, and the shared `speak` uses it.

```rust
trait Speaker {
    fn name(&self) -> String;                  // required
    fn speak(&self) -> String {                // default
        format!("{} says hi", self.name())
    }
}

struct Bob;
impl Speaker for Bob {
    fn name(&self) -> String { "Bob".to_string() }
    // no speak(): Bob gets the default, "Bob says hi"
}

struct Loud;
impl Speaker for Loud {
    fn name(&self) -> String { "Loud".to_string() }
    fn speak(&self) -> String { "HEY".to_string() }   // override
}
```

**Override.** A type that needs something different writes the method itself in its `impl`, and that version wins, for that type only.

**The type's own version.** When a default calls `self.greet()`, it gets the type's own `greet`, override included. That is why a `shout` written once still shouts a parrot's line.

## Your turn

- In `trait Greeter`, give `greet` a default body: `Hello, <name>!` using `self.name()`.
- Give `shout` a default body: `greet` in uppercase.
- In `impl Greeter for Parrot`, override `greet` with `Squawk! <name> wants a cracker!`.
