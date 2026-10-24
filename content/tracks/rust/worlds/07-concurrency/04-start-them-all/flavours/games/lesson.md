Hashing one jar on a thread is not faster than hashing it on the main thread. The win comes from many jars at once: six jars that each take 150 milliseconds should take about 150 in total, not 900. That only happens if all six workers are running at the same time.

**Try the obvious thing.** Spawn a worker for the jar and `join` it, inside the loop, for each jar. It works and gives the right hashes, but `join` waits for that worker to finish before the loop goes on to start the next one. Only one worker runs at any moment, so it is as slow as no threads at all.

**Start, then wait.** Split the work into two loops. The first loop only spawns: it pushes each `JoinHandle` into a `Vec` and moves on at once, so every worker is running by the end of it. The second loop joins the handles one by one and collects the hashes.

**The order stays.** The workers may finish in any order, as quest 01 showed. But you join the handles in the order you pushed them, so the hashes come out in folder order anyway. A worker that finished early simply waits until its turn to be collected.

```rust
use std::thread;

let jars = vec![String::from("iris.jar"), String::from("jei.jar")];

let mut handles = Vec::new();
for jar in jars {
    handles.push(thread::spawn(move || format!("hashed {jar}")));
}

let mut lines = Vec::new();
for handle in handles {
    lines.push(handle.join().unwrap());
}
```

## Your turn

- In `hash_all`, spawn one worker per jar size that calls `slow_hash(size)`, keeping every handle.
- Then join the handles in order and return the hashes. The second objective checks that the six jars overlap.
