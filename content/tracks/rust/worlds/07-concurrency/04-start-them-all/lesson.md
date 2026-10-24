Checking one file on a thread is not faster than checking it on the main thread. The win comes from many files at once: six files that each take 150 milliseconds should take about 150 in total, not 900. That only happens if all six threads are running at the same time.

**Try the obvious thing.** Spawn a thread for the file and `join` it, inside the loop, for each file. It works and gives the right answers, but `join` waits for that thread to finish before the loop goes on to start the next one. Only one thread runs at any moment, so it is as slow as no threads at all.

**Start, then wait.** Split the work into two loops. The first loop only spawns: it pushes each `JoinHandle` into a `Vec` and moves on at once, so every thread is running by the end of it. The second loop joins the handles one by one and collects the answers.

**The order stays.** The threads may finish in any order, as quest 01 showed. But you join the handles in the order you pushed them, so the answers come out in the input order anyway. A thread that finished early simply waits until its turn to be collected.

```rust
use std::thread;

let names = vec![String::from("a.txt"), String::from("b.txt")];

let mut handles = Vec::new();
for name in names {
    handles.push(thread::spawn(move || format!("checked {name}")));
}

let mut lines = Vec::new();
for handle in handles {
    lines.push(handle.join().unwrap());
}
```

## Your turn

- In `check_all`, spawn one thread per size that calls `slow_checksum(size)`, keeping every handle.
- Then join the handles in order and return the checksums. The second objective checks that the six checks overlap.
