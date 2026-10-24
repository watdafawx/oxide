A folder of 5,000 files should not get 5,000 threads: each costs memory and start-up time, and a computer with 8 cores runs only 8 at once anyway. A **worker pool** starts a few threads, say 4, and they share the jobs: whoever is free takes the next file. Rayon does this inside; the hasher's boss builds one by hand.

**A queue of jobs.** The jobs go into a channel: the parent sends every file size in at `job_tx`. But a `Receiver` cannot be cloned, because a channel has a single consumer. So the workers share the one `Receiver`, and sharing a thing that every worker changes is quest 13's `Arc<Mutex<..>>`.

**Read it inside out.** `Arc<Mutex<Receiver<u64>>>` reads from the inside: a receiver of `u64` sizes, behind a lock, shared by many owners. "A shared, lockable receiver of sizes." Each worker gets an `Arc::clone` and locks it to take one job.

**Lock for one job only.** `let job = jobs.lock().unwrap().recv();` locks, waits for the next job, and the guard is dropped at the `;`. The lock is free again *before* the worker starts the slow work, so the others can take jobs meanwhile. Writing it as `while let Ok(size) = jobs.lock().unwrap().recv()` looks neater but keeps the guard alive for the whole loop body: still correct, but only one worker could be working at a time.

**Repeat until done.** `loop { .. }` repeats its block forever, until a `break`. `recv()` gives `Ok(size)` while jobs remain and `Err` once the channel is closed and empty, which is the worker's signal to `break`. The channel closes when the parent drops `job_tx` after sending the last job.

**Results flow back.** Each worker also gets a clone of a second channel's `Sender` for its answers. The parent drops its own `res_tx` (quest 07's trap) and collects everything from `res_rx`. Workers finish in any order, so sort the results.

```rust
use std::sync::{Arc, Mutex, mpsc};
use std::thread;

let (job_tx, job_rx) = mpsc::channel::<String>();
let jobs = Arc::new(Mutex::new(job_rx));
let mine = Arc::clone(&jobs);
let worker = thread::spawn(move || {
    let mut seen = 0;
    loop {
        let job = mine.lock().unwrap().recv();   // guard dropped here
        match job {
            Ok(name) => seen += name.len(),
            Err(_) => break,                     // closed and empty: stop
        }
    }
    seen
});
job_tx.send(String::from("a.txt")).unwrap();
drop(job_tx);
println!("{}", worker.join().unwrap());          // 5
```

## Your turn

- In `check_with_pool`, start `workers` threads. Each loops: take a job, send `slow_checksum(size)` on the results channel, and `break` when the job channel is closed.
- Send every size, then drop both of your own `Sender`s.
- Collect the results, sort them and return them.
