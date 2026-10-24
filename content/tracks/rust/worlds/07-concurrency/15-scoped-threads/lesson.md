The hasher's `parallel_sum` gets a borrowed `&[u64]` of file sizes and should add up each half on its own thread. It does not own the sizes, so it cannot `move` them in, and wrapping them in an `Arc` would mean copying them first. There is a third way.

**Try the obvious thing.** Cut off the left half with `&sizes[..mid]` and hand it to `thread::spawn(move || ...)`. The compiler says `error[E0521]: borrowed data escapes outside of function`, and under it: "argument requires that `'1` must outlive `'static`". It is quest 03's rule again: a spawned thread might outlive the function, so it may not hold a borrow.

**A scope that waits.** `thread::scope(|s| { ... })` opens a block for threads. Inside it, `s.spawn(|| ...)` starts a thread, just like `thread::spawn`. The difference: `thread::scope` does not return until every thread started in it has finished. Since no thread can outlive the scope, and the scope cannot outlive your function, the threads may borrow your function's data. No `move`, no `Arc`, no `'static`.

**Getting answers out.** `s.spawn` also returns a handle, and `join()` on it gives the thread's answer, as before. Whatever the scope's closure hands back at its end is what `thread::scope` returns. Threads you never join are joined for you at the closing brace.

```rust
use std::thread;

let names = vec![String::from("a.txt"), String::from("photo.jpg")];
let letters = thread::scope(|s| {
    let first = s.spawn(|| names[0].len());   // borrows names, no move
    let second = s.spawn(|| names[1].len());
    first.join().unwrap() + second.join().unwrap()
});
println!("{letters} {}", names.len());        // 14 2: names is still ours
```

## Your turn

- In `parallel_sum`, cut `sizes` into a left and a right half at `sizes.len() / 2`.
- Inside a `thread::scope`, sum each half on its own scoped thread and return the two sums added together.
