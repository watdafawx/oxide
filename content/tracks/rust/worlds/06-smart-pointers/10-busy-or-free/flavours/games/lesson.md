An inserter wants to drop its item into a chest, but another machine may be reading the chest right now. If the inserter calls `borrow_mut()` then, the program panics. It would rather wait a tick, so it needs to ask first: "is this chest busy right now?"

**Try, don't demand.** `try_borrow_mut()` does what `borrow_mut()` does, but instead of panicking it returns a `Result`: `Ok` with the writer when the cell is free, `Err` when some borrow is alive.

**Only yes or no.** `result.is_err()` is `true` for an `Err` and `false` for an `Ok`; `is_ok()` is the opposite. If the answer was an `Ok`, the writer inside it is dropped at the end of the statement, so asking leaves the chest free again.

```rust
let can_read = belt.try_borrow().is_ok();   // try_borrow: the reader version
```

**Readers count too.** A live `borrow()` also blocks a writer, because the rule is many readers *or* one writer. So "busy" means any borrow at all is alive, which is exactly when `try_borrow_mut` fails.

## Your turn

- In `chest_busy`, return whether a `try_borrow_mut()` on the chest would fail.
