A crafting grid has rows of three slots, and your hotbar is a row of nine. The calculator needs the total of a crafting row, and it needs to grab one slot by its place in the row. A row of numbers is called an array.

**A row of one type.** `[10, 20, 30]` is an **array**: items of the same type in a fixed row, in square brackets. Its type in code carries the length: `[u32; 3]` is three `u32` numbers.

**Ask by place.** `row[0]` is the first item. Places count from 0, so a row of three ends at `row[2]`. The number in the brackets is called the **index**.

**How long is it.** `.len()` tells you how many items a row has. The last item is always at `len() - 1`, which is handy when you do not want to count by hand.

```rust
let row = [10, 20, 30];       // [u32; 3]
let first = row[0];           // 10
let count = row.len();        // 3
```

**Past the end.** An index that does not exist is a bug, so Rust stops the program on the spot:

```text
index out of bounds: the len is 3 but the index is 5
```

**If this is new.** Programming 101 explains lists and indexes from zero: [A list](/quest/basics/06-collections/01-a-list).

## Your turn

- In `row_total`, add the three slots by index.
- In `last_slot`, return the final slot of the hotbar, using `.len()`.
- Save. Both objectives turn green.
