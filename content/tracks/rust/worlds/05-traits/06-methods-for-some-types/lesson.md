A pair of sizes can say which side is larger. A pair of colours can't: there is no "larger" colour. You want `larger` on `Pair<u32>` without demanding that every `Pair` be comparable.

**A bound on the block.** `impl<T: PartialOrd> Pair<T> { ... }` adds methods only for pairs whose `T` can be compared. The plain `impl<T>` methods, like `new`, still work for every pair.

**Missing, not broken.** `Pair<Colour>` simply has no `larger`. Call it and the error says the method exists but its trait bounds were not satisfied, so you see why at the call, not in a crash later.

```rust
use std::fmt::Display;

struct Wrapper<T> {
    value: T,
}

impl<T: Display> Wrapper<T> {       // only when T can be printed
    fn show(&self) -> String {
        format!("[{}]", self.value)
    }
}
// Wrapper { value: 5 }.show() is "[5]"
// Wrapper { value: vec![1] }.show() does not build: Vec has no Display
```

**Stack them.** A second block, `impl<T: Display + PartialOrd>`, adds methods that need both abilities. Inside it you may call `larger`, because its bound is part of this one.

**For the curious: blanket impls.** The same trick works for traits: `impl<T: Display> Describe for T { .. }` implements `Describe` for every printable type at once. The standard library does exactly this for `ToString`, which is why `.to_string()` works on anything you can print.

## Your turn

- In the `PartialOrd` block, write `larger`: a reference to the bigger field. Ties return `first`.
- In the `Display + PartialOrd` block, write `describe_larger`: `The larger is <value>`, using `larger`.
