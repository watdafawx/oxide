A game has players, items, monsters. A **struct** bundles related values under one name.

```rust
struct Dog {
    name: String,
    age: u32,
}

impl Dog {
    fn new(name: &str) -> Dog {            // associated fn: no self, called Dog::new(..)
        Dog { name: name.to_string(), age: 0 }
    }
    fn birthday(&mut self) {               // &mut self: may change the dog
        self.age += 1;
    }
    fn is_puppy(&self) -> bool {           // &self: only reads
        self.age < 2
    }
}
```

- Fields are listed once in `struct`, methods live in an `impl` block.
- `Dog::new(..)` is just a convention for constructors. It is a plain function.
- `&self` reads, `&mut self` changes. Call methods with `dog.birthday()`.
- `u32` can't go negative: `5u32 - 9` panics in debug. `saturating_sub` stops at 0.

## Your turn

- Open `src/lib.rs`. The `Player` struct is given.
- Fill `new`, `take_hit` and `is_alive` in the `impl` block.
