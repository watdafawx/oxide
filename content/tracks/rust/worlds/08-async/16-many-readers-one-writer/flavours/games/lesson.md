The launcher remembers which version of each mod is installed so it can skip jars that are up to date. Every download task looks versions up all the time, and only a finished install ever changes one. A lock that lets one task in at a time would make the readers queue for no reason.

**A Mutex lets one in.** With a `Mutex`, even two tasks that only read take turns, although reading at the same time can't hurt anyone. For a value that is read far more often than it is written, that waiting is wasted.

**Read or write.** An `RwLock` (read-write lock) has two doors. `read().await` lets any number of readers in together; `write().await` waits until nobody else is inside and then lets in exactly one writer. It is the borrow rule from world 02, "many `&` or one `&mut`", checked while the program runs.

```rust
use std::collections::HashMap;
use tokio::sync::RwLock;

let sizes: RwLock<HashMap<String, u64>> = RwLock::new(HashMap::new());
sizes.write().await.insert("sodium.jar".to_string(), 4_200_000); // one writer
let known = sizes.read().await.contains_key("sodium.jar");        // many readers
```

**Owned out of a guard.** The guard from `read()` lends the map, so `map.get(key)` gives an `Option<&String>` that dies with the guard. `.cloned()` turns it into an `Option<String>` you can return.

**Shared by Arc.** `insert` takes `&self`, not `&mut self`: the lock inside guards the changes, so many tasks can hold the cache through one `Arc<VersionCache>`, the interior-mutability idea of world 06 made safe for threads.

## Your turn

- In `insert`, take the write lock and insert owned copies of `mod_id` and `version`.
- In `get`, take the read lock and return a copy of the version.
- In `count`, take the read lock and return the number of entries.
