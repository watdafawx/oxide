Your meter said pushing 100 chunk ids into `Vec::new()` took 6 allocations, while `Vec::with_capacity(100)` took 1. Both chunk-indexer helpers in this quest give the right answer and grow step by step; the test in `tests/allocations.rs` counts the steps.

**Why it grows in steps.** A `Vec` has a **capacity**, the room its heap block holds, and a length, the items in use. When a push finds no room, the `Vec` asks for a new block twice as big, copies every item across, and frees the old one. For 100 pushes the capacity goes 4, 8, 16, 32, 64, 128: six allocations, and 124 items copied on the way.

**Doubling is not the bug.** Doubling keeps the total copying small, so pushing stays cheap on average. But when you know the size in advance, all of it is wasted work.

**Ask once.** `Vec::with_capacity(n)` makes an empty `Vec` (length 0) whose block already holds `n` items. Pushing up to `n` items never allocates again. `String::with_capacity` works the same way for text.

```rust
let mut hotbar: Vec<&str> = Vec::with_capacity(3);
println!("{} {}", hotbar.len(), hotbar.capacity());   // 0 3
hotbar.push("pickaxe");
hotbar.push("torch");
hotbar.push("bread");
println!("{} {}", hotbar.len(), hotbar.capacity());   // 3 3, still the first block
```

**Let the library add it up.** `join(" ")` on a slice of `&str` first adds up the lengths of all the pieces and separators, reserves that much once, then copies each piece in. One call, one allocation, and shorter than the loop.

## Your turn

- In `chunk_areas`, start from a `Vec` with room for `n` items.
- In `join_block_names`, replace the loop with `join` on the slice.
