Taking the lamp out of the backpack should hand you the lamp itself, an owned `String`, and leave the backpack one item shorter. A copy would leave two lamps; a borrow would leave the lamp in the bag.

**Try the obvious thing.** `let lamp = items[i];` tries to move a `String` out of the middle of the list. That would leave a hole the `Vec` still thinks it owns, and it would drop that hole later. Rust refuses:

```text
error[E0507]: cannot move out of index of `Vec<String>`
```

**Ask the owner.** `items.remove(i)` takes item `i` out, shifts the rest down to close the gap, and hands you the owned value. The list stays whole, and you are the new owner.

**Find the position.** Walk the positions with `for i in 0..items.len()` and compare `items[i] == wanted`. A `String` compares with a `&str` directly.

```rust
for i in 0..jars.len() {
    if jars[i] == "jam" {
        return Some(jars.remove(i));
    }
}
None
```

**Leave right away.** The `return` inside the loop matters: after `remove`, the positions shift and the list is shorter than the range you started with. Hand the item over and stop.

## Your turn

- In `take`, find the first item equal to `wanted`, remove it and give it back in `Some`.
- Give back `None` when it is not there.
