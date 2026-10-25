The outliner tidies every line it loads: tabs become spaces. Almost no line has a tab, so returning a fresh `String` for each one copies thousands of lines for nothing. The result type should be able to say either "the same text you gave me" or "new text", and only the second should cost anything.

**What a copy costs.** A `String` owns its text on the heap, so making one means asking the heap for memory and copying the bytes in. That request is an **allocation**, and it is the slow part. A `&str` borrows text that already exists and costs nothing.

**One type, two cases.** `Cow<str>` is an enum with two variants: `Cow::Borrowed(&str)` for text you hand back untouched, and `Cow::Owned(String)` for text you had to build. The name means "clone on write": you pay for a copy only when you change something.

```rust
use std::borrow::Cow;

fn no_commas(text: &str) -> Cow<'_, str> {
    if text.contains(',') {
        Cow::Owned(text.replace(',', ""))   // had to change: allocate
    } else {
        Cow::Borrowed(text)                 // untouched: free
    }
}
```

**Use it like text.** `Cow` implements `Deref` with `Target = str`, so callers print it, compare it with `==` and call `.len()` like any string. `.into_owned()` turns either case into a `String` when you need to keep one.

**The lifetime.** The `Borrowed` case points into the input, so the result cannot outlive it, the same promise as world 02's borrowing structs. With one reference coming in, `'_` asks Rust to tie the result to it.

**Which one you got.** `matches!(value, Cow::Borrowed(_))` is `true` when the value fits the pattern, without writing a whole `match`. The tests use it to check that a clean line was not copied.

## Your turn

- In `normalise`, check whether `input` contains a tab, `'\t'`.
- With a tab, return `Cow::Owned` with each tab replaced by one space. Without one, return `Cow::Borrowed(input)`.
