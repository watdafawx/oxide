The hasher's first feature is small: hash one jar on a thread of its own, so the launcher's main thread stays free to draw its window. Rust's standard library starts threads with `std::thread`, and getting the hash back is half the job.

**Start a thread.** `thread::spawn` takes a closure, the `|| ...` you know from the collections world, and runs it on a brand new thread. A closure with nothing between the bars takes no parameters. Whatever the closure hands back at its end is the thread's answer.

**A ticket, not the answer.** `spawn` returns at once, while the new thread is still working. What you get is a `JoinHandle<T>`, a ticket for an answer of type `T` that is not ready yet. The launcher can keep drawing its progress bar in the meantime.

**Wait for it.** `handle.join()` waits until that thread has finished and then hands over its answer. It is a `Result`, because the thread might have crashed instead; quest 05 deals with that, so `unwrap()` it for now. Waiting matters for another reason too: when the main thread ends, the whole process ends, and any thread still working is stopped mid-step.

```rust
use std::thread;

let handle = thread::spawn(|| {
    "iris.jar".len() * 2       // the thread's answer
});
// the launcher could draw a frame here
let answer = handle.join().unwrap(); // 16
```

## Your turn

- In `start_hash`, spawn a thread that computes `quick_hash("sodium.jar")` and return its handle without waiting.
- In `hash_and_wait`, start the hash, wait for it, and return the answer.
