**Why now:** while the hasher runs, a UI thread, a log thread and a size-checker all want to look at the mod list, but only now and then does the installer change it. A `Mutex` would make the lookers queue up. **`RwLock`** lets any number of readers in together, and a writer waits for its turn alone.

```rust
use std::sync::RwLock;

let mods = RwLock::new(vec![String::from("sodium.jar")]);

{
    let a = mods.read().unwrap();   // reader 1
    let b = mods.read().unwrap();   // reader 2: fine at the same time
    println!("{} {}", a.len(), b.len());
}                                   // both released

mods.write().unwrap().push(String::from("iris.jar")); // writer: alone
```

- `read()` gives a shared guard. Many can exist at once.
- `write()` gives an exclusive guard. It waits until every reader is gone.
- Asking for `write()` **while you still hold a read guard** on the same thread deadlocks. `try_write()` returns `Err` at once instead.
- Use `RwLock` when reads far outnumber writes. For short or mixed access a `Mutex` is simpler.

## Your turn

In `src/lib.rs`:

- `mod_count`: read lock, return the length.
- `add_mod`: write lock, push the name.
- `two_scanners_share`: hold one read guard, then see whether a second read lock is granted.
- `installer_is_refused`: hold a read guard, then see whether `try_write` is refused.
