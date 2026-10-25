The owner asks "how many different products did we sell today?", and a list with every sale in it has repeats. Sorting and `dedup` would work, but there is a collection built for "each value once": the **set**.

**Each value once.** A **`HashSet`** holds values, not key-value pairs, and holds each value at most once: inserting `pen` a second time changes nothing. It lives next to `HashMap`, so it needs `use std::collections::HashSet;`. Like a `HashMap`, it keeps no order.

**Collect into one.** `collect` can build a set too. `sales.iter().collect::<HashSet<_>>()` drops every repeat on the way, and `.len()` then counts the different values. The turbofish says "a HashSet, you work out of what".

```rust
use std::collections::HashSet;

let staff: HashSet<&str> = ["ann", "bo", "ann"].iter().copied().collect();
staff.len()               // 2
staff.contains("bo")      // true, and fast, however big the set is
```

**What two sets share.** `a.intersection(&b)` is an iterator over the values in both sets. It lends them, so map each to an owned `String` before you collect. A set has no order, so sort the result when the order matters.

**For the curious.** `union` (in either) and `difference` (in `a` but not `b`) work the same way, and `contains` on a set is fast where `contains` on a `Vec` checks every item.

## Your turn

- In `distinct_products`, collect the sales into a `HashSet` and give its length.
- In `in_both`, make a set of each list, intersect them, turn each into a `String`, collect and sort.
