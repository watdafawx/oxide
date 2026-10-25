A product sells well, so the owner wants it at the front of the shelf where customers see it first. Moving one item means taking it out of the list and putting it back somewhere else.

**Take it out.** `shelf.remove(2)` takes the item at position 2 out of the list and hands it back to you, so you can keep it in a `let`. Everything after it slides one place to the left to close the gap.

**Put it in.** `shelf.insert(0, item)` puts an item at position 0 and slides everything else one place to the right. `push` is still the way to add at the end.

```rust
let mut shelf = vec!["pen".to_string(), "mug".to_string(), "lamp".to_string()];
let mug = shelf.remove(1);     // shelf is now [pen, lamp]
shelf.insert(0, mug);          // [mug, pen, lamp]
```

**Changing someone else's list.** The functions take `shelf: &mut Vec<String>`: a mutable borrow, as in the ownership world. They change the caller's list in place and return nothing.

**For the curious.** Sliding is real work: removing from the front of a list of a million items moves the other 999,999. For small lists it does not matter; for big ones, Rust has other containers.

## Your turn

- In `move_to_front`, take the product out at `index` and insert it at 0.
- In `move_to_back`, take it out and push it on the end.
