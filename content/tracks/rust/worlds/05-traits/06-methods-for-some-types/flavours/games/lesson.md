Two stack counts can say which hand holds more. Two potion effects can't: there is no "bigger" effect. You want `best` on `Hands<u32>` without demanding that every item type be comparable.

**A bound on the block.** `impl<T: PartialOrd> Hands<T> { ... }` adds methods only for hands whose `T` can be compared. The plain `impl<T>` methods, like `new`, still work for every item.

**Missing, not broken.** `Hands<Effect>` simply has no `best`. Call it and the error says the method exists but its trait bounds were not satisfied, so you see why at the call, not in a crash later.

```rust
use std::fmt::Display;

struct Chest<T> {
    item: T,
}

impl<T: Display> Chest<T> {         // only when T can be printed
    fn show(&self) -> String {
        format!("[{}]", self.item)
    }
}
// Chest { item: 5 }.show() is "[5]"
// Chest { item: vec![1] }.show() does not build: Vec has no Display
```

**Stack them.** A second block, `impl<T: Display + PartialOrd>`, adds methods that need both abilities. Inside it you may call `best`, because its bound is part of this one.

**For the curious: blanket impls.** The same trick works for traits: `impl<T: Display> Describe for T { .. }` implements `Describe` for every printable type at once. The standard library does exactly this for `ToString`, which is why `.to_string()` works on anything you can print.

## Your turn

- In the `PartialOrd` block, write `best`: a reference to the better item. Ties return `main_hand`.
- In the `Display + PartialOrd` block, write `describe_best`: `Best item: <item>`, using `best`.
