The hasher's first feature is small: hash one file on a thread of its own, so the main thread stays free. Rust's standard library starts threads with `std::thread`, and getting the answer back is half the job.

**Start a thread.** `thread::spawn` takes a closure, the `|| ...` you know from the collections world, and runs it on a brand new thread. A closure with nothing between the bars takes no parameters. Whatever the closure hands back at its end is the thread's answer.

**A ticket, not the answer.** `spawn` returns at once, while the new thread is still working. What you get is a `JoinHandle<T>`, a ticket for an answer of type `T` that is not ready yet. The main thread can carry on with other work in the meantime.

**Wait for it.** `handle.join()` waits until that thread has finished and then hands over its answer. It is a `Result`, because the thread might have crashed instead; quest 05 deals with that, so `unwrap()` it for now. Waiting matters for another reason too: when the main thread ends, the whole process ends, and any thread still working is stopped mid-step.

```rust
use std::thread;

let handle = thread::spawn(|| {
    "photo.jpg".len() * 2      // the thread's answer
});
// the main thread could do other work here
let answer = handle.join().unwrap(); // 18
```

## Your turn

- In `start_check`, spawn a thread that computes `checksum("report.pdf")` and return its handle without waiting.
- In `check_and_wait`, start the check, wait for it, and return the answer.
