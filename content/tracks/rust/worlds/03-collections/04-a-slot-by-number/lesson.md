Customers ask "how many are on shelf 3?", and sometimes they name a shelf that does not exist. The stock tool has to answer by position without crashing on a bad number.

**The position type.** Positions in a `Vec` are a **`usize`**: an unsigned whole number, like `u32`, whose size matches the computer's memory addresses. A position is never negative, and on a 64-bit machine a list can be longer than a `u32` can count. `.len()` hands back a `usize` too.

**Reaching too far.** `counts[9]` on a three-item list panics, just like the arrays in the Foundations world. That is fine when a wrong position means a bug in your code, but here the number comes from a customer.

**Ask instead.** `counts.get(9)` never panics. It answers with an `Option`: `Some(...)` when the shelf exists, `None` when it does not. Then you decide what a missing shelf means.

```rust
let counts = vec![12, 6, 1];
counts.get(1)              // Some(&6): a reference into the list
counts.get(9)              // None
counts.get(1).copied()     // Some(6): the number itself
```

**The arrow inside.** `get` leaves the item in the list and gives you a reference, so the answer is an `Option<&u32>`. `.copied()` follows the arrow inside the Option, like `*` does on its own, and gives you an `Option<u32>`.

## Your turn

- In `count_on_shelf`, ask with `get` and turn the answer into an `Option<u32>`.
- In `count_or_zero`, do the same and fall back to 0 with `unwrap_or`.
