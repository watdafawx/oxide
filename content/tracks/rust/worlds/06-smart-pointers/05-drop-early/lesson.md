Saving has an order: write the temporary file, delete it, then log "saved". But the file only dies at the function's closing brace, after the log line. And while it lives, it holds the log borrowed.

**Try the obvious thing.** Make the file, then push "saved" straight onto the log. Rust refuses with `error[E0499]: cannot borrow *log as mutable more than once at a time`. The `TempFile` holds the one `&mut` to the log, and it holds it until it is dropped, which is still to come at the brace.

**End it now.** `drop(file)` ends the value on that line. It is a plain function from the standard library that takes the value by move, so the value is gone when it returns and its `Drop` code runs on the way out. After that line, the borrow it held is over.

```rust
let lamp = Lamp::new("desk", log);
drop(lamp);                          // "desk off" is logged here
log.push("door locked".to_string()); // the log is free again
```

**Why it is safe.** After `drop(file)`, using `file` again is the "use of moved value" error from world 02. The compiler makes sure nothing touches a value whose cleanup has already run.

## Your turn

- In `save`, make a `TempFile` named `"save.tmp"` with `TempFile::new("save.tmp", log)`.
- End it with `drop`, then push `"saved"` onto `log`.
