The launcher keeps a table of jar hashes it already knows, so an unchanged jar is skipped. Every worker reads the table for every jar, and only a new or updated mod adds an entry. With a `Mutex`, readers queue up behind each other even though reading together is harmless: quest 08's data race needs at least one writer.

**Readers together, writers alone.** `RwLock<T>`, a **read-write lock**, has two ways in. `read()` gives a reader's guard, and any number of threads may hold one at the same time. `write()` gives a writer's guard, and only when nobody else holds any guard at all. Both return a `Result`, poisoned the same way a `Mutex` is.

**Writers wait.** A writer waits until every reader has left, and while it writes, new readers wait for it. So asking for `write()` while *you* still hold a read guard is a deadlock: you wait for a reader to leave, and that reader is you.

**Ask without waiting.** `try_read()` and `try_write()` never wait. If the lock is free they give `Ok(guard)`; if not they give an `Err` right away. `.is_err()` then answers "was I turned away?".

**Which one to pick.** `RwLock` does more bookkeeping than `Mutex`, so it pays off only when reads are common and writes are rare, like the known-hash table. When most visits change the value, as with a counter, a `Mutex` is simpler and faster.

```rust
use std::sync::RwLock;

let settings = RwLock::new(vec![String::from("render_distance=12")]);
{
    let a = settings.read().unwrap();             // a reader
    let b = settings.read().unwrap();             // a second reader is fine
    println!("{} {}", a.len(), b.len());
}                                                 // both readers leave
settings.write().unwrap().push(String::from("fov=90")); // one writer
```

## Your turn

- In `known_count`, take a read lock and return the length of the table.
- In `remember`, take a write lock and push the hash.
- In `writer_is_refused`, hold a read guard and return whether `try_write` was turned away.
