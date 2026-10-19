**Why now:** your console parses a typed line into a `Command` with a name and an argument. Copying both pieces of text would be wasteful, so `Command<'a>` just **borrows** from the line. Now comparing two commands raises a question: the answer points into *which* line, and for how long is it valid?

Usually the compiler works out how long a borrow lasts on its own. When a function takes **two** references and returns one, it can't guess which input the answer came from, so you tell it.

```rust
struct Command<'a> {
    name: &'a str,
    arg: &'a str,
}

fn longer<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}
```

Read it as: "the result lives no longer than **both** inputs". That's the whole feature.

- `'a` is just a label (like `T` for types). It's a **promise the compiler checks**, not something that makes data live longer.
- With one reference in and one out, Rust fills it in for you. Only ambiguous cases need `'a`.

```rust
let x = String::from("kick Rustacean");
let result;
{
    let y = String::from("kick Bo");
    result = longer(&x, &y);  // fine here...
    println!("{result}");
}
// println!("{result}");      // ...error here: y is gone
```

## Your turn

- In `src/lib.rs`, write the body of `longest_arg`. `Command<'a>` and `parse` are already there; the `'a` signature too.
- On equal lengths return the first one's argument.
