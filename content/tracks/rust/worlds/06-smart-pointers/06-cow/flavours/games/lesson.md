Your machine graph handles thousands of item names, and nearly all of them already come as `mod:item` (`base:iron-plate`). A few arrive bare (`gear-wheel`) and need the mod id in front. Returning a fresh `String` for every name copies thousands of names for nothing; only the bare ones should cost anything.

**What a copy costs.** A `String` owns its text on the heap, so making one means asking the heap for memory and copying the bytes in. That request is an **allocation**, and it is the slow part. A `&str` borrows text that already exists and costs nothing.

**One type, two cases.** `Cow<str>` is an enum with two variants: `Cow::Borrowed(&str)` for text you hand back untouched, and `Cow::Owned(String)` for text you had to build. The name means "clone on write": you pay for a copy only when you change something.

```rust
use std::borrow::Cow;

fn with_base(name: &str) -> Cow<'_, str> {
    if name.starts_with("base:") {
        Cow::Borrowed(name)                  // fine as it is: free
    } else {
        Cow::Owned(format!("base:{name}"))   // had to change: allocate
    }
}
```

**Use it like text.** `Cow` implements `Deref` with `Target = str`, so callers print it, compare it with `==` and call `.len()` like any string. `.into_owned()` turns either case into a `String` when you need to keep one.

**The lifetime.** The `Borrowed` case points into the name, so the result cannot outlive it, the same promise as world 02's borrowing structs. Your function takes two references, so its signature says which one the result borrows from: `Cow<'a, str>` ties it to `name: &'a str`.

**Which one you got.** `matches!(value, Cow::Borrowed(_))` is `true` when the value fits the pattern, without writing a whole `match`. The tests use it to check that a ready name was not copied.

## Your turn

- In `qualified_name`, check whether `name` already contains a `:`.
- If it does, return `Cow::Borrowed(name)`. If not, return `Cow::Owned` with `"<mod_id>:<name>"`.
