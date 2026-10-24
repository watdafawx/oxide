A storage network holds hundreds of items, but the chest screen shows 27 slots at a time. Page 0 is items 0 to 26, page 1 is items 27 to 53. Two small adaptors cut a list into pages.

**Take the first few.** `.take(n)` lets the first `n` items through and then stops the chain. If there are fewer than `n`, you simply get them all; it never panics. Because iterators are lazy, the items after the first `n` are never even looked at.

**Skip some first.** `.skip(k)` passes over the first `k` items and lets the rest through. Skipping more items than there are gives an empty chain, again with no panic.

```rust
let online = ["Alex", "Steve", "Kai", "Noor", "Ed"];
online.iter().skip(2).take(2)    // Kai, Noor
online.iter().skip(9).take(2)    // nothing
```

**Pages from the two.** Page `p` of size `s` skips the `p * s` items before it and takes the next `s`. The last page can be short, and a page past the end is empty, both for free.

**Left to right.** A chain runs in the order you write it. `skip(1).take(2)` passes over one item and then takes two; `take(2).skip(1)` takes two and then drops the first of those, leaving one.

## Your turn

- In `first_items`, take the first `n` items and clone each into the new `Vec`.
- In `chest_page`, skip the earlier pages and take one page's worth.
