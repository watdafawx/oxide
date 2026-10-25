The chest screen highlights the fullest stack, and the compact view needs the shortest item name to fit a tooltip. Both want one item from the list: the one that wins a comparison.

**Pick by a key.** `.max_by_key(closure)` is a consumer. The closure turns each item into a **key**, a number to compare by, and `max_by_key` hands back the item whose key is biggest. You get the item itself, not the key: the stack, not its count.

**Maybe nothing.** An empty inventory has no biggest stack, so the answer is an `Option`: `Some(item)` or `None`. After `.iter()` the item is a reference into the list, so the type is `Option<&ItemStack>` or `Option<&String>`.

```rust
let mobs = ["zombie", "enderman", "bat"];
mobs.iter().max_by_key(|m| m.len())    // Some("enderman")
mobs.iter().min_by_key(|m| m.len())    // Some("bat")
```

**The smallest.** `.min_by_key` is the same with the smallest key. Neither needs a `*` in the closure: `s.count` and `n.len()` reach through the references for you.

**Ties.** When two stacks share the biggest count, `max_by_key` gives back the last of them, and `min_by_key` the first. If a tie must go a certain way, put that in the key or sort first.

## Your turn

- In `biggest_stack`, pick the stack with the biggest `count`.
- In `shortest_name`, pick the name with the smallest length.
