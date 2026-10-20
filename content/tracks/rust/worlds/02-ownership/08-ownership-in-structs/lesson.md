A struct field can own its data (`String`) or borrow it (`&str`). The choice decides how long everything lives.

```rust
struct Owned {
    name: String,          // owns the text
}

struct Borrowed<'a> {
    name: &'a str,         // points at text owned elsewhere
}
```

- **Owned** is simple: the struct carries its text wherever it goes. Pick this by default.
- **Borrowed** is cheap (no copy) but the struct can't outlive the text it points at. Rust needs a name for "how long", so a **lifetime parameter** `'a` appears. Read `Borrowed<'a>` as "a Borrowed that is valid for as long as `'a`".
- The `'a` doesn't change how long anything lives. It only lets the compiler check the promise.

You do not write any lifetimes in this quest: both structs and the `new` functions are given. Your job is to produce a **view** of an `Owned`. The next quest shows why a function that returns a borrow needs a lifetime in its signature (it is given there too; you write the body).

## Your turn

- In `src/lib.rs` fill in `Owned::as_borrowed`. Everything else is written.
- Notice `Borrowed<'_>` in its signature: `'_` means "fill in the lifetime for me", and here it ties the view to `&self`.
- `&self.name` is a `&String`; Rust turns it into the `&str` the field wants automatically.
