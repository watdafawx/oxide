Every new stack you pick up starts full, with 64 items. Spelling out the name and the number every time you make one is a good way to typo the number once, in one place, and spend an hour finding it. Give the rule one home.

**A function with no self.** Inside an `impl` block you can put a function that has no `self` at all. It does not work on an existing stack; it makes a new one. That type is called an **associated function**.

**Call it with two colons.** Because there is no stack to put a dot after, you call it through the type's name: `Chest::new("tools")`. Two colons, the type, then the function.

**The name `new`.** There is nothing special about the word `new`. It is just the name everyone uses for "make me one", so people know where to look.

**What it returns.** The last line is the finished struct, without a semicolon, and the arrow names the struct as the type it hands back:

```rust
impl Chest {
    fn new(label: &str) -> Chest {
        Chest { label: label.to_string(), slots: 27 }
    }
}

let chest = Chest::new("tools");
```

## Your turn

- In `ItemStack::new`, return an `ItemStack` with the given name and a count of 64.
- Save. Both objectives turn green.
