**Why now:** the server loads every chunk around a player, and you want `for chunk in ChunksInRadius::new(4)`, plus `filter` and `map` on top. `for`, `map`, `filter` and `sum` all sit on one tiny trait. Write **one method** and every adaptor works on your type.

```rust
struct Row { cur: i32, last: i32 }

impl Iterator for Row {
    type Item = i32;                       // what each step yields

    fn next(&mut self) -> Option<i32> {    // Some(value) or None when finished
        if self.cur > self.last { return None; }
        let v = self.cur;
        self.cur += 1;
        Some(v)
    }
}

let v: Vec<i32> = Row { cur: 1, last: 3 }.collect();                         // [1, 2, 3]
let big: i32 = Row { cur: 1, last: 4 }.filter(|n| n % 2 == 0).sum();         // 2 + 4 = 6
```

- `next` returns `Some(item)` until the end, then `None` forever.
- **`type Item`** names the element type. It is an *associated type*.
- You never write `map`, `filter`, `collect`: the trait gives you dozens of **adaptors**.

## Your turn

- Open `src/lib.rs`.
- Write `next` for `ChunksInRadius`: x is the outer loop, z the inner loop.
- Use adaptors on it in `ore_chunks` and `total_distance_sq`.
