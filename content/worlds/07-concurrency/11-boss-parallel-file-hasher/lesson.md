You are building the core of a checksum tool: point it at a folder, get one hash per file, fast. It uses everything in this world.

**The pieces**

- **`fnv1a`**: a tiny hash. Start with `0xcbf29ce484222325`; for each byte: XOR it in, then `wrapping_mul(0x100000001b3)`. All in `u64`.
- **`walk`**: recursive listing of files (not folders), sorted.
- **`Progress`**: an `AtomicUsize` the workers bump after each file, so a UI could show "37 / 200".
- **`hash_all`**: a fixed **pool of `workers` threads** (not one thread per file). Paths go in through a channel, `(path, hash)` pairs come out through another. Results are sorted by path, so the output never depends on thread timing.
- **`hash_all_rayon`**: the same result in three lines.
- **`report`**: the text you would print.

```rust
// shape of the pool (not complete):
let (job_tx, job_rx) = mpsc::channel::<PathBuf>();
let job_rx = Arc::new(Mutex::new(job_rx));   // workers take turns receiving
let (res_tx, res_rx) = mpsc::channel();
// spawn N workers: loop { recv a job; hash it; send (path, hash) }
// send all paths, drop(job_tx), drop(res_tx), then collect res_rx
```

Tests run `hash_all` on a helper thread with a 10 s limit, so a channel you forgot to close fails instead of freezing. They create their own files under your system temp folder.

## Your turn

In `src/lib.rs`, replace each `todo!()`: `Progress`, `fnv1a`, `walk`, `hash_all_tracked` (the real pool), `hash_all`, `hash_all_rayon`, `report`. `hash_file` is given. Delete the `#![allow(...)]` line at the top when you are done.

Press Run to play it.
