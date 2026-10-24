Every hash worker needs to read the same list of jar names. Copying a list of 200 names once per worker wastes memory and time. In the smart-pointers world forty assemblers shared one recipe with `Rc`, so try that first.

**Try the obvious thing.** The starter puts the jar names in an `Rc`, makes one `Rc::clone` per worker and moves each clone in. The compiler refuses:

```text
error[E0277]: `Rc<Vec<String>>` cannot be sent between threads safely
```

**Why Rc is refused.** An `Rc` keeps a count of its owners in a plain number. `Rc::clone` does `count += 1`, and dropping one does `count -= 1`. If two threads do that at the same moment, that is exactly the data race from the last quest: a lost update leaves the count too low, and the list is freed while a worker is still reading it.

**Arc: the thread-safe Rc.** `Arc<T>`, short for **atomically reference counted**, works like `Rc`: `Arc::new`, `Arc::clone`, `Arc::strong_count`, and the value is freed when the last owner is dropped. The difference is the counter. Arc changes it with an **atomic** operation, a special processor instruction that does read, add and write as one step no other thread can get between. That is a little slower, which is why Rust keeps both: `Rc` on one thread, `Arc` across threads.

**Still read-only.** Like `Rc`, an `Arc` gives every owner shared access only: each worker may read the jar list, none may change it. Changing shared data needs a lock, and that is quest 13.

```rust
use std::sync::Arc;
use std::thread;

let jars = Arc::new(vec![120, 4_000, 75]);  // jar sizes in KB
let mine = Arc::clone(&jars);               // count 1 -> 2, no copy of the Vec
let handle = thread::spawn(move || mine[1]);
println!("{}", handle.join().unwrap());     // 4000
```

## Your turn

- Save and read the E0277 error.
- Change every `Rc` in the file to `Arc` (including the `use` line). The first objective turns green.
- In `share_jars`, return another owner of the same jar list.
