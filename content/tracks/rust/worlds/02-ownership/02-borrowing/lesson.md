Moving everything into every function would be exhausting. A **reference** lets a function look at a value without taking it.

`&x` borrows `x`. The owner keeps ownership; the function gets a temporary read-only view.

```rust
struct Backpack {
    items: Vec<String>,
}

fn count(bag: &Backpack) -> usize {
    bag.items.len()
}

let bag = Backpack { items: vec!["rope".to_string()] };
let n = count(&bag);                // lend it
println!("{n} {}", bag.items.len()); // still ours
```

- The type `&Backpack` means "a borrowed Backpack".
- Borrow as many times at once as you like, as long as nobody changes it.
- No `.clone()` needed: nothing is copied.

## Your turn

- In `src/lib.rs`, fill in `count_items` and `total_chars`. Both already take `&Backpack`.
- Save: the third objective passes once both work and the bag is still usable.
