The hasher also shows the average file size of a folder. An empty folder means dividing by zero files, and integer division by zero **panics**. On one thread that ends the program. On a thread of its own, the damage stays inside that thread.

**One thread falls over.** When a spawned thread panics, Rust prints the panic message and ends *that thread only*. The main thread and every other thread keep running. This is why a crash in one background job does not have to take the whole tool down.

**join tells you.** The `Result` from `handle.join()` is `Ok(answer)` when the thread finished normally, and `Err(..)` when it panicked. Until now you called `unwrap()` on it, which would pass the panic on to the main thread. Here you want to handle it instead.

**From Result to Option.** You met `.ok()` in the collections world: it turns `Ok(v)` into `Some(v)` and any `Err` into `None`. On the result of `join` it means "the answer, or nothing if the thread crashed". You will still see the panic message in the test output; that is the thread reporting before it ends, and the test passes anyway.

```rust
use std::thread;

let parts: Vec<u32> = Vec::new();
let first = thread::spawn(move || parts[0]).join().ok(); // index out of range
println!("{first:?}");                                   // None
```

## Your turn

- In `average_size`, divide `total_bytes` by `file_count` on a new thread.
- Join it and return `None` when that thread panicked, using `.ok()`.
