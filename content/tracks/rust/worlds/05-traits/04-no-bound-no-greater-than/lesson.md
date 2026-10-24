The ruler tool shows the largest size in a list: whole pixels on the grid, decimals on the vector layer, names in the font menu. One `largest` should cover all three, but first the compiler has a question for you.

**Try the obvious thing.** Write `fn largest<T>(xs: &[T]) -> &T` and compare two items with `>`. Rust refuses to build:

```text
error[E0369]: binary operation `>` cannot be applied to type `&T`
help: consider restricting type parameter `T` with trait `PartialOrd`
```

**Why it says no.** `T` could be any type at all, including one where "bigger" means nothing, like a colour or a file. Rust checks the body once, for every possible `T`, so it only allows what the bounds promise.

**Standard traits are bounds too.** The standard library names abilities as traits. **`PartialOrd`** means "can be compared with `<` and `>`", and **`Display`** means "can be printed with `{}`". With `T: PartialOrd`, the `>` is allowed.

**More than one.** Join bounds with `+`: `T: Display + PartialOrd`. When the list grows long, move it into a **`where`** clause after the signature. It means the same thing and keeps the first line readable.

```rust
use std::fmt::Display;

fn smaller<T: PartialOrd>(a: T, b: T) -> T {
    if a < b { a } else { b }
}

fn show_smaller<T>(a: T, b: T) -> String
where
    T: Display + PartialOrd,
{
    format!("smaller: {}", smaller(a, b))
}
```

**The message is the plan.** You rarely guess bounds ahead. Write the body, and the compiler's `help` line names the trait to add, as it did above.

## Your turn

- In `largest`, keep a reference to the best item and compare with `>`. Save and read the error.
- Add the bound the error asks for to `largest`.
- Write `announce_largest`: `largest: <item>`, using `largest`. Its `where` clause is already there.
