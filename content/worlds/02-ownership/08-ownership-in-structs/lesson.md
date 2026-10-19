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

If the lifetime syntax feels like too much, use `String` and move on. The next quest shows the one place you must write it by hand.

## Your turn

- In `src/lib.rs` fill in `Owned::new`, `Borrowed::new` and `Owned::as_borrowed`.
- The lifetimes are already written. Notice `Borrowed<'_>` in `as_borrowed`: it ties the view to `&self`.
