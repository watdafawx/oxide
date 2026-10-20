**Why now:** the hasher hands each worker a list of jars, and the worker must own that list because the function that built it returns first.

`thread::spawn` cannot know when your function returns, so the worker might still be running after its locals are gone. Rust refuses to let a thread **borrow** them. Use a **`move` closure** and the worker owns what it needs.

```rust
use std::thread;

let jar_name = String::from("sodium.jar");

let handle = thread::spawn(move || {          // jar_name moves into the worker
    format!("hashing {jar_name}")
});
// println!("{jar_name}");                    // error: moved
println!("{}", handle.join().unwrap());
```

- Without `move` the error says the closure "may outlive the current function".
- After the move the original variable is gone. Clone first if you still need a copy.
- To give two workers each their own part, split the data first and move one part into each. **`split_off(mid)`** does the cut: `let right = v.split_off(2);` leaves the first 2 items in `v` and returns the rest as a new `Vec`.

## Your turn

In `src/lib.rs`:

- `label_in_thread`: build `"Hashing <jar_name>"` on a worker that owns the name.
- `total_bytes_in_thread`: move the whole `Vec` into a worker and sum it there.
- `split_folder`: `split_off` the jar list in two and total each half on its own worker.
