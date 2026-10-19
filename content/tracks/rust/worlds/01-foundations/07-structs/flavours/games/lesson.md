**Why now:** the calculator needs a real thing to count: an item and how many of it you hold. A **struct** bundles the name and the count under one name, `ItemStack`, and gives it behaviour like "take 5 out".

```rust
struct Chest {
    label: String,
    slots: u32,
}

impl Chest {
    fn new(label: &str) -> Chest {         // associated fn: no self, called Chest::new(..)
        Chest { label: label.to_string(), slots: 27 }
    }
    fn add_slots(&mut self, extra: u32) {  // &mut self: may change the chest
        self.slots += extra;
    }
    fn is_big(&self) -> bool {             // &self: only reads
        self.slots > 27
    }
}
```

- Fields are listed once in `struct`, methods live in an `impl` block.
- `Chest::new(..)` is just a convention for constructors. It is a plain function.
- `&self` reads, `&mut self` changes. Call methods with `chest.add_slots(9)`.
- `u32` cannot go negative: `5u32 - 9` panics in debug. `saturating_sub` stops at 0.

## Your turn

- Open `src/lib.rs`. The `ItemStack` struct is given.
- Fill `new`, `take` and `has_items` in the `impl` block.
