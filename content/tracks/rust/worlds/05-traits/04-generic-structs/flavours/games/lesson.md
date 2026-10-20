**Why now:** a player inventory holds swords, stack counts or block names, but the code for "two hands, swap them, pick the best" never changes. Put a type parameter on the **struct**: `Inventory<T>`.

```rust
use std::fmt::Display;

struct Chest<T> {
    item: T,
}

impl<T> Chest<T> {                         // methods for every T
    fn new(item: T) -> Self { Chest { item } }
}

impl<T: Display> Chest<T> {                // only when T can be printed
    fn show(&self) -> String { format!("[{}]", self.item) }
}
```

- `impl<T>` declares `T`, then `Chest<T>` uses it.
- A **bounded impl block** (`impl<T: Display>`) gives methods only to types that qualify.
- **`PartialOrd`** means "can be compared with `<` and `>=`". **`Display`** means "can be printed with `{}`". They are bounds like any other.
- `Chest<u32>` has `show`. `Chest<Vec<u32>>` does not: no `Display`.

## Your turn

- Open `src/lib.rs`.
- Write `new` and `swap_hands` (any `T`), `best` (needs `PartialOrd`) and `describe_best` (needs `Display` too).
