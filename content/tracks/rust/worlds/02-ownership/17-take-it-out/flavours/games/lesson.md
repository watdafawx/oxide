`give` hands an item from the admin stash to a player. Taking the diamond out should hand you the diamond itself, an owned `String`, and leave the stash one item shorter. A copy would be a duplication glitch; a borrow would leave the diamond in the stash.

**Try the obvious thing.** `let diamond = stash[i];` tries to move a `String` out of the middle of the list. That would leave a hole the `Vec` still thinks it owns, and it would drop that hole later. Rust refuses:

```text
error[E0507]: cannot move out of index of `Vec<String>`
```

**Ask the owner.** `stash.remove(i)` takes item `i` out, shifts the rest down to close the gap, and hands you the owned value. The list stays whole, and you are the new owner.

**Find the position.** Walk the positions with `for i in 0..stash.len()` and compare `stash[i] == item`. A `String` compares with a `&str` directly.

```rust
for i in 0..banned.len() {
    if banned[i] == "Alex" {
        return Some(banned.remove(i));
    }
}
None
```

**Leave right away.** The `return` inside the loop matters: after `remove`, the positions shift and the list is shorter than the range you started with. Hand the item over and stop.

## Your turn

- In `take_from_stash`, find the first item equal to `item`, remove it and give it back in `Some`.
- Give back `None` when it is not there.
