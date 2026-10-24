Your drawing app has circles and squares, and the layers panel needs a label for each one. Two different structs need the same verb, `describe`. In Rust, a shared verb is a **trait**.

**Try the obvious thing.** You could write `describe_circle(c: &Circle)` and `describe_square(s: &Square)`. That works until the tenth shape, and code that wants "the label of any shape" still has no type to ask for.

**No parent class.** Many languages solve this with inheritance: a base class `Shape` that `Circle` and `Square` extend. Rust has no classes and no inheritance. A struct holds only its own data, and shared behaviour is a separate promise each type signs up to.

**The promise.** A **trait** is that promise: a list of method signatures (name, parameters, return type) with no bodies. It says "every type that implements me has these methods".

**Keeping it.** `impl Describe for Line { ... }` is where a type keeps the promise: you write the bodies there, and then `line.describe()` works like any method. If you implemented `Display` in Errors, you have done this once already: `Display` is a trait too.

```rust
trait Describe {
    fn describe(&self) -> String;      // signature only: no body
}

struct Line {
    length: u32,
}

impl Describe for Line {               // Line keeps the promise
    fn describe(&self) -> String {
        format!("line {} long", self.length)
    }
}

let l = Line { length: 5 };
let text = l.describe();               // "line 5 long"
```

**Why it beats a family tree.** You can add a trait to a type at any time, without touching the struct, and one type can sign up for as many traits as it needs. Nothing forces a circle to be "a kind of" anything.

## Your turn

- In `impl Describe for Circle`, return text like `circle with radius 2`.
- In `impl Describe for Square`, return text like `square with side 3`.
