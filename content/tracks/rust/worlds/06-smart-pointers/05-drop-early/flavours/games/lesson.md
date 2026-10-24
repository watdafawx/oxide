A hot swap has an order: the old inserter powers down, then the new one is placed. But the old inserter only dies at the function's closing brace, after the "place" line. And while it lives, it holds the log borrowed.

**Try the obvious thing.** Make the inserter, then push "place new inserter" straight onto the log. Rust refuses with `error[E0499]: cannot borrow *log as mutable more than once at a time`. The `Inserter` holds the one `&mut` to the log, and it holds it until it is dropped, which is still to come at the brace.

**End it now.** `drop(old)` ends the value on that line. It is a plain function from the standard library that takes the value by move, so the value is gone when it returns and its `Drop` code runs on the way out. After that line, the borrow it held is over.

```rust
let lamp = Lamp::new("lamp-3", log);
drop(lamp);                              // "lamp-3 off" is logged here
log.push("place new lamp".to_string()); // the log is free again
```

**Why it is safe.** After `drop(old)`, using `old` again is the "use of moved value" error from world 02. The compiler makes sure nothing touches a machine whose power-down has already run.

## Your turn

- In `hot_swap`, make an `Inserter` named `"inserter-1"` with `Inserter::new("inserter-1", log)`.
- End it with `drop`, then push `"place new inserter"` onto `log`.
