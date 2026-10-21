An inventory slot holds two facts: which item it is and how many. Two separate variables work, but you cannot return both from a function, and they drift apart. You want one value that carries both.

**Two values, one name.** `(5, 64)` is a **tuple**: a few values in round brackets, kept together. Its type in code is written the same way, `(u32, u32)`.

**Mixed types are fine.** The parts of a tuple can be different types, like `(u32, bool)`. The size is fixed: a pair stays a pair.

**Take it apart.** To get the parts out, write the names in the same shape on the left of a `let`. This is called **destructuring**. You can also ask for a part by its position, counting from 0:

```rust
let slot = (5, 64);          // (item id, count)
let (id, count) = slot;      // id is 5, count is 64
let first = slot.0;          // 5
```

**Return two things.** A function can return a tuple. Put the pair as the last line, and write the pair's type after the arrow: `-> (u32, u32)`.

## Your turn

- In `slot_count`, destructure the slot and return the count.
- In `craft_planks`, return a pair: how many logs were used and how many planks they make (four per log).
- Save. Both objectives turn green.
