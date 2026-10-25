The player shift-clicks a pickaxe deep in the inventory, and it should jump to the first hotbar slot. Moving one item means taking it out of the list and putting it back somewhere else.

**Take it out.** `inventory.remove(2)` takes the item in slot 2 out of the list and hands it back to you, so you can keep it in a `let`. Everything after it slides one slot to the left to close the gap.

**Put it in.** `inventory.insert(0, item)` puts an item in slot 0 and slides everything else one slot to the right. `push` is still the way to add at the end.

```rust
let mut inventory = vec!["sword".to_string(), "bow".to_string(), "torch".to_string()];
let bow = inventory.remove(1);     // [sword, torch]
inventory.insert(0, bow);          // [bow, sword, torch]
```

**Changing someone else's list.** The functions take `inventory: &mut Vec<String>`: a mutable borrow, as in the ownership world. They change the caller's list in place and return nothing.

**For the curious.** Sliding is real work: removing from the front of a list of a million items moves the other 999,999. For a 36-slot inventory it does not matter; for big lists, Rust has other containers.

## Your turn

- In `move_to_hotbar`, take the item out of `slot` and insert it at 0.
- In `move_to_end`, take it out and push it on the end.
