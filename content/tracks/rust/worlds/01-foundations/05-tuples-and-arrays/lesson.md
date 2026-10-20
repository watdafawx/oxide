Positions on a map, a row of inventory slots: small groups of values show up everywhere.

```rust
let pos: (i32, i32) = (3, 7);        // tuple: fixed size, mixed types OK
let (x, y) = pos;                    // destructure into two variables
let first = pos.0;                   // or access by position

let slots = [10, 20, 30];            // array: fixed size, one type
let middle = slots[1];               // indexes start at 0
let count = slots.len();             // 3
let [a, b, c] = slots;               // arrays destructure too (a = 10, b = 20, c = 30)
```

- Tuples hold **different** types: `(String, u32)`. Arrays hold the **same** type.
- Both have a **fixed size** known at compile time.
- Going past the end (`slots[5]`) **panics** at runtime.
- `&[i32]` is a *slice*: a borrowed view of any number of items. It has `.len()` too.

## Your turn

- Open `src/lib.rs`.
- `first_and_last`: index into the slice, using `.len()` for the end.
- `swap_pair`: destructure the tuple and return it flipped.
- `sum_three`: destructure the array and add.
