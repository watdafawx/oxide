The stock sheet is too long for one screen, so the tool shows it ten lines per page. Page 0 is lines 0 to 9, page 1 is lines 10 to 19. Two small adaptors cut a list into pages.

**Take the first few.** `.take(n)` lets the first `n` items through and then stops the chain. If there are fewer than `n`, you simply get them all; it never panics. Because iterators are lazy, the items after the first `n` are never even looked at.

**Skip some first.** `.skip(k)` passes over the first `k` items and lets the rest through. Skipping more items than there are gives an empty chain, again with no panic.

```rust
let staff = ["ann", "bo", "cy", "dee", "ed"];
staff.iter().skip(2).take(2)    // cy, dee
staff.iter().skip(9).take(2)    // nothing
```

**Pages from the two.** Page `p` of size `s` skips the `p * s` lines before it and takes the next `s`. The last page can be short, and a page past the end is empty, both for free.

**Left to right.** A chain runs in the order you write it. `skip(1).take(2)` passes over one item and then takes two; `take(2).skip(1)` takes two and then drops the first of those, leaving one.

## Your turn

- In `first_lines`, take the first `n` names and clone each into the new `Vec`.
- In `page`, skip the earlier pages and take one page's worth.
