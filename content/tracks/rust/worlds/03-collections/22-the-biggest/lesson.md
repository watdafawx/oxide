The shelf labels have a fixed width, so the owner needs the longest product name to size them. And the reorder screen should open on the product with the fewest units left. Both want one item from the list: the one that wins a comparison.

**Pick by a key.** `.max_by_key(closure)` is a consumer. The closure turns each item into a **key**, a number to compare by, and `max_by_key` hands back the item whose key is biggest. You get the item itself, not the key: the name, not its length.

**Maybe nothing.** An empty list has no biggest item, so the answer is an `Option`: `Some(item)` or `None`. After `.iter()` the item is a reference into the list, so the type is `Option<&String>` or `Option<&StockLine>`.

```rust
let staff = ["ann", "marisol", "bo"];
staff.iter().max_by_key(|n| n.len())    // Some("marisol")
staff.iter().min_by_key(|n| n.len())    // Some("bo")
```

**The smallest.** `.min_by_key` is the same with the smallest key. Neither needs a `*` in the closure: `n.len()` and `l.qty` reach through the references for you.

**Ties.** When two items share the biggest key, `max_by_key` gives back the last of them, and `min_by_key` the first. If a tie must go a certain way, put that in the key or sort first.

## Your turn

- In `longest_name`, pick the name with the biggest length.
- In `fewest_left`, pick the stock line with the smallest `qty`.
