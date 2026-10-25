The end-of-round scoreboard shows the top value of each column: kill counts are whole numbers, damage is a decimal, and the name column is sorted A to Z. One `top` should cover all three, but first the compiler has a question for you.

**Try the obvious thing.** Write `fn top<T>(xs: &[T]) -> &T` and compare two items with `>`. Rust refuses to build:

```text
error[E0369]: binary operation `>` cannot be applied to type `&T`
help: consider restricting type parameter `T` with trait `PartialOrd`
```

**Why it says no.** `T` could be any type at all, including one where "bigger" means nothing, like a block colour or a sound. Rust checks the body once, for every possible `T`, so it only allows what the bounds promise.

**Standard traits are bounds too.** The standard library names abilities as traits. **`PartialOrd`** means "can be compared with `<` and `>`", and **`Display`** means "can be printed with `{}`". With `T: PartialOrd`, the `>` is allowed.

**More than one.** Join bounds with `+`: `T: Display + PartialOrd`. When the list grows long, move it into a **`where`** clause after the signature. It means the same thing and keeps the first line readable.

```rust
use std::fmt::Display;

fn lower<T: PartialOrd>(a: T, b: T) -> T {
    if a < b { a } else { b }
}

fn show_lower<T>(a: T, b: T) -> String
where
    T: Display + PartialOrd,
{
    format!("lower: {}", lower(a, b))
}
```

**The message is the plan.** You rarely guess bounds ahead. Write the body, and the compiler's `help` line names the trait to add, as it did above.

## Your turn

- In `top`, keep a reference to the best item and compare with `>`. Save and read the error.
- Add the bound the error asks for to `top`.
- Write `announce_top`: `top: <item>`, using `top`. Its `where` clause is already there.
