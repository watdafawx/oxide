The hasher also shows the average jar size of a modpack. A fresh instance with an empty `mods/` folder means dividing by zero jars, and integer division by zero **panics**. On one thread that ends the launcher. On a worker thread of its own, the damage stays inside that worker.

**One thread falls over.** When a spawned thread panics, Rust prints the panic message and ends *that thread only*. The main thread and every other worker keep running. This is why one broken jar does not have to take the whole launcher down.

**join tells you.** The `Result` from `handle.join()` is `Ok(answer)` when the worker finished normally, and `Err(..)` when it panicked. Until now you called `unwrap()` on it, which would pass the panic on to the main thread. Here you want to handle it instead.

**From Result to Option.** You met `.ok()` in the collections world: it turns `Ok(v)` into `Some(v)` and any `Err` into `None`. On the result of `join` it means "the answer, or nothing if the worker crashed". You will still see the panic message in the test output; that is the worker reporting before it ends, and the test passes anyway.

```rust
use std::thread;

let jars: Vec<u32> = Vec::new();
let first = thread::spawn(move || jars[0]).join().ok(); // index out of range
println!("{first:?}");                                  // None
```

## Your turn

- In `average_jar_size`, divide `total_bytes` by `jar_count` on a new thread.
- Join it and return `None` when that worker panicked, using `.ok()`.
