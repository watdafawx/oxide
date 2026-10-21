"Is this stack full?" is a question about a stack, and the answer lives in its own numbers. Instead of a loose function that you hand a stack to, you can teach the stack to answer. A function that belongs to a struct is called a method.

**Rules for one thing.** An `impl ItemStack { ... }` block holds the functions that belong to `ItemStack`. Inside it, a function whose first parameter is `&self` is a **method**.

**Call it with a dot.** You call a method on a value with a dot, the same way you called `.trim()`: `stack.is_full()`. The stack before the dot is the one the method works on.

**What self is.** Inside the method, `self` is that stack. `self.count` reads its field, just like `stack.count` did outside.

**Look, do not touch.** The `&` in `&self` means the method only *looks* at the stack. It is not used up and not changed. A later world explains the `&` in full; for now read it as "borrow to look".

```rust
impl Chest {
    fn is_big(&self) -> bool {
        self.slots > 27
    }
}

let chest = Chest { label: "tools".to_string(), slots: 54 };
chest.is_big()      // true
```

## Your turn

- In `is_empty`, return whether `self.count` is 0.
- In `is_full`, return whether `self.count` is 64.
- Save. Both objectives turn green.
