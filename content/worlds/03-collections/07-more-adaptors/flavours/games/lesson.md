**Why now:** the hotbar is nine numbered slots, a chest has pages, and an upgrade path compares each tool with the next. All of that used to be index math. A handful of small adaptors replace it.

```rust
let hotbar = ["sword", "pickaxe", "torch"];

for (i, item) in hotbar.iter().enumerate() {   // (0, "sword"), (1, "pickaxe"), ...
    println!("slot {}: {item}", i + 1);
}

let counts = [1, 64, 16];
let pairs: Vec<_> = hotbar.iter().zip(counts).collect();   // [("sword",1), ...]

let first_two: Vec<_> = hotbar.iter().take(2).collect();   // sword, pickaxe
let after_one: Vec<_> = hotbar.iter().skip(1).collect();   // pickaxe, torch

let tiers = [1, 3, 2, 4];
let pairs: Vec<&[i32]> = tiers.windows(2).collect();       // [1,3] [3,2] [2,4]
```

- **`enumerate()`**: gives `(index, item)`. **`zip(other)`**: pairs two iterators, stops at the shorter.
- **`take(n)` / `skip(n)`**: first n / all but the first n. Together they make pages.
- **`windows(n)`**: slides a window of n neighbours over a slice. Perfect for "compare each with the next".
- **`rev()`** reverses; the same chain style works on `chars()` later.

## Your turn

- Open `src/lib.rs`.
- Four functions, one chain each: `hotbar_labels`, `loot_value`, `chest_page`, `count_upgrades`.
