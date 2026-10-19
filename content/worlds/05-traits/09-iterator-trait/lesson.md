`for`, `map`, `filter` and `sum` all sit on one tiny trait. Write **one method** and every adaptor works on your type.

```rust
struct Up { cur: u32, max: u32 }

impl Iterator for Up {
    type Item = u32;                       // what each step yields

    fn next(&mut self) -> Option<u32> {    // Some(value) or None when finished
        if self.cur >= self.max { return None; }
        self.cur += 1;
        Some(self.cur)
    }
}

let v: Vec<u32> = Up { cur: 0, max: 3 }.collect();                         // [1, 2, 3]
let big: u32 = Up { cur: 0, max: 4 }.filter(|n| n % 2 == 0).sum();         // 2 + 4 = 6
```

- `next` returns `Some(item)` until the end, then `None` forever.
- **`type Item`** names the element type. It is an *associated type*.
- You never write `map`, `filter`, `collect`: the trait gives you dozens of **adaptors**.

## Your turn

- Open `src/lib.rs`.
- Write `next` for `Countdown`.
- Use adaptors on it in `evens` and `sum_of_squares`.
