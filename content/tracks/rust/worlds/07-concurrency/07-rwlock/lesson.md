**Why now:** the hasher keeps a table of already-seen hashes that every worker reads for each file and only sometimes updates; `RwLock` lets all the readers in at once.

A `Mutex` lets one thread in at a time, even if all of them only read. **`RwLock<T>`** is smarter: any number of **readers** together, or exactly **one writer** alone. Good for config that is read constantly and changed rarely.

```rust
use std::sync::RwLock;

let seen = RwLock::new(vec![0xa1, 0xb2]);

{
    let a = seen.read().unwrap();      // shared access
    let b = seen.read().unwrap();      // a second reader is fine
    println!("{} {}", a.len(), b.len());
}                                        // both guards dropped

seen.write().unwrap().push(0xc3);        // exclusive access
```

- `read()` / `write()` block until allowed. They return guards, same poisoning story as `Mutex`.
- A writer waits for all readers to leave. Asking for `write()` while *you* hold a read guard is a **deadlock**.
- `try_read()` and `try_write()` return immediately with `Err(WouldBlock)` instead of waiting. Handy for checks and tests.
- `.is_ok()` / `.is_err()` on a `Result` give a plain `bool`: `lock.try_read().is_ok()` asks "did I get in?".
- Use `Mutex` when writes are common. `RwLock` pays off when reads dominate.

## Your turn

In `src/lib.rs`:

- `entry_count` reads, `add_entry` writes.
- `readers_share`: hold one read guard, try for a second, report if you got it.
- `writer_is_refused`: hold a read guard, report whether `try_write` failed.
