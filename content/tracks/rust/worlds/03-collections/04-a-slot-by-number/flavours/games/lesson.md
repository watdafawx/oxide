A chat command like `/peek 3` asks what is in hotbar slot 3, and players type any number they like. Your plugin has to answer by position without crashing the server on a bad slot.

**The slot type.** Positions in a `Vec` are a **`usize`**: an unsigned whole number, like `u32`, whose size matches the computer's memory addresses. A slot is never negative, and on a 64-bit machine a list can be longer than a `u32` can count. `.len()` hands back a `usize` too.

**Reaching too far.** `hotbar[9]` on a three-slot hotbar panics, just like the arrays in the Foundations world. That is fine when a wrong slot means a bug in your code, but here the number comes from a player.

**Ask instead.** `hotbar.get(9)` never panics. It answers with an `Option`: `Some(...)` when the slot exists, `None` when it does not. Then you decide what a missing slot means.

```rust
let hotbar = vec![1, 16, 8];
hotbar.get(1)              // Some(&16): a reference into the list
hotbar.get(9)              // None
hotbar.get(1).copied()     // Some(16): the number itself
```

**The arrow inside.** `get` leaves the item in the list and gives you a reference, so the answer is an `Option<&u32>`. `.copied()` follows the arrow inside the Option, like `*` does on its own, and gives you an `Option<u32>`.

## Your turn

- In `stack_in_slot`, ask with `get` and turn the answer into an `Option<u32>`.
- In `stack_or_empty`, do the same and fall back to 0 with `unwrap_or`.
