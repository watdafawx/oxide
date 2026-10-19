**Why now:** a recipe lives in a small grid and a hotbar is a row of slots. Both are fixed-size groups, which is exactly what tuples and arrays are, and your calculator will read them constantly.

```rust
let slot: (u32, u32) = (3, 64);      // tuple: (item id, count). Mixed types OK
let (id, count) = slot;              // destructure into two variables
let first = slot.0;                  // or access by position

let row = [10, 20, 30];              // array: fixed size, one type
let middle = row[1];                 // indexes start at 0
let width = row.len();               // 3
```

- Tuples hold **different** types: `(String, u32)`. Arrays hold the **same** type.
- Both have a **fixed size** known at compile time.
- Going past the end (`row[5]`) **panics** at runtime.
- `&[u32]` is a *slice*: a borrowed view of any number of items. It has `.len()` too.

## Your turn

- Open `src/lib.rs`.
- `hotbar_ends`: index into the slice, using `.len()` for the end.
- `swap_slots`: destructure the tuple and return it flipped.
- `row_total`: destructure the array and add.
