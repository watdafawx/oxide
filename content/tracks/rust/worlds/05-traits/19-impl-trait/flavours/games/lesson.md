You have written `impl Fn(i32) -> i32` since Collections. That `impl` was this world's idea all along: "some type that implements this trait". It works for any trait, in two places, and the server's chat banner and spawn timer need one each.

**In an argument.** `fn banner(text: impl Display)` is a shorter way to write `fn banner<T: Display>(text: T)`. Same generic, same copy per type, just without naming `T`.

**In a return.** `fn spawn_ticks(..) -> impl Iterator<Item = u32>` promises "some iterator of `u32`" without naming it. An iterator chain has a type like `Filter<Range<u32>, {closure}>` that you can't even write down, so this is often the only way to return one.

```rust
use std::fmt::Display;

fn tag(x: impl Display) -> String {                       // any printable value
    format!("<{x}>")
}

fn night_ticks(limit: u32) -> impl Iterator<Item = u32> { // some iterator, unnamed
    (1..=limit).map(|n| n * 10)
}
// tag(5) is "<5>", night_ticks(2) yields 10, 20
```

**Exactly one type.** The function picks one hidden type, the same on every path. Return a filtered range from one branch and a reversed range from the other, and Rust refuses:

```text
error[E0308]: `if` and `else` have incompatible types
help: you could change the return type to be a boxed trait object
```

That help is the trait object from this world: `Box<dyn Iterator<Item = u32>>` can hold a different type each call.

**The caller sees only the trait.** Code that calls `spawn_ticks` can use every `Iterator` method but can't rely on the concrete type, so you can change the chain later without breaking anyone.

## Your turn

- Write `banner`: the text in uppercase with a `!` at the end.
- Write `spawn_ticks`: every tick below `limit` that is a multiple of 5, as a lazy iterator.
