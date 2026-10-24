A player's hands hold swords, stack counts or block names, and "two hands, swap them" never changes. `Vec<T>` and `Option<T>` hold any type, and your own struct can too: `Hands<T>`.

**A type parameter on a struct.** Write `struct Hands<T> { main_hand: T, off_hand: T }`. `Hands<u32>` and `Hands<&str>` are two real types built from one definition, the same way `Vec<u32>` and `Vec<String>` are.

**Declare it on the impl.** Methods go in `impl<T> Hands<T>`. The first `<T>` declares the placeholder and the second uses it: "for every `T`, here are the methods of `Hands<T>`".

**Self is the type.** Inside an `impl`, **`Self`** is short for the type the block is for, here `Hands<T>`. `fn new(..) -> Self` reads "returns a Hands", and keeps working if you rename the struct.

```rust
struct Chest<T> {
    item: T,
}

impl<T> Chest<T> {
    fn new(item: T) -> Self {
        Chest { item }
    }

    fn take(self) -> T {            // self, not &self
        self.item
    }
}
```

**self, not &self.** A method that takes plain `self` takes the value itself, not a borrow. Calling it **moves** the hands into the method, as moves did in Ownership, so the caller can't use the old value afterwards.

**Why that fits a swap.** After `let h = h.swap_hands();` nobody needs the old hands. Because `swap_hands` owns them, it can move both items across without copying or cloning them.

## Your turn

- Write `Hands::new`: build `Hands` from the two items.
- Write `swap_hands`: new `Hands` with `main_hand` and `off_hand` crossed over.
