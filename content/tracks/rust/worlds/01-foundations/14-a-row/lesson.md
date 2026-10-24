A shelf has three slots, each holding a number of boxes. The planner needs the total, and it needs to grab one slot by its place on the shelf. A row of numbers is called an array.

**A row of one type.** `[10, 20, 30]` is an **array**: items of the same type in a fixed row, in square brackets. Its type in code carries the length: `[u32; 3]` is three `u32` numbers.

**Ask by place.** `shelf[0]` is the first item. Places count from 0, so a row of three ends at `shelf[2]`. The number in the brackets is called the **index**.

**How long is it.** `.len()` tells you how many items a row has. The last item is always at `len() - 1`, which is handy when you do not want to count by hand.

```rust
let shelf = [10, 20, 30];     // [u32; 3]
let first = shelf[0];         // 10
let count = shelf.len();      // 3
```

**Past the end.** An index that does not exist is a bug, so Rust stops the program on the spot:

```text
index out of bounds: the len is 3 but the index is 5
```

**If this is new.** Programming 101 explains lists and indexes from zero: [A list](/quest/basics/06-collections/01-a-list).

## Your turn

- In `shelf_total`, add the three slots by index.
- In `last_box`, return the final slot of nine, using `.len()`.
- Save. Both objectives turn green.
