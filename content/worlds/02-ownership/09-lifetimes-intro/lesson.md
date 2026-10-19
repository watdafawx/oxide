Usually the compiler works out how long a borrow lasts on its own. When a function takes **two** references and returns one, it can't guess which input the answer came from, so you tell it.

```rust
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}
```

Read it as: "the result lives no longer than **both** inputs". That's the whole feature.

- `'a` is just a label (like `T` for types). Any name works; `'a` is the habit.
- It's a **promise the compiler checks**, not something that makes data live longer.
- With one reference in and one out, Rust fills it in for you. Only ambiguous cases need `'a`.

```rust
let x = String::from("hello");
let result;
{
    let y = String::from("hi");
    result = longest(&x, &y); // fine here...
    println!("{result}");
}
// println!("{result}");      // ...error here: y is gone
```

## Your turn

- In `src/lib.rs`, write the body of `longest`. The `'a` signature is already there.
- On equal lengths return the first one.
