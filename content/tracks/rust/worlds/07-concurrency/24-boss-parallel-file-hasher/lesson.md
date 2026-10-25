This is the finished tool: point it at a folder and get one hash per file, fast, with a progress count a window could show as "37 / 500". Every piece is a quest from this world; the boss is putting them together in the right order.

**Bottom up.** Start with the three small pieces, the first three objectives. `Progress` wraps one `AtomicUsize`, like the files-done counter from quest 18. `fnv1a` is the fingerprint of bytes from quest 21; its constants are in the doc comment. `walk` is quest 20's every-file-in-a-folder.

**The pool, with a counter.** `hash_all_tracked` is quest 23's worker pool, with paths as jobs and `(path, hash)` pairs as results. Each worker calls `progress.inc()` after each file. `progress` is a borrowed `&Progress`, and quest 15 showed that `thread::spawn` refuses borrows, so start the workers with `thread::scope` and `s.spawn(move || ..)`. The `move` moves each worker's `Arc` clone and `Sender` clone in, and copies the `&Progress` reference, which stays valid because the scope ends before your function does.

**Sort, so timing never shows.** Workers finish in whatever order the system runs them, as quest 01 showed. Sorting the pairs by path makes the result the same on every run, which is what lets a test, or a person, compare two runs.

**A second opinion.** `hash_all_rayon` gets the same answer with rayon's `par_iter`, quest 17, in a few lines. The tests compare it with the pool, so if both agree, both are probably right.

**The report.** `report` makes the text: a first line like `"25 files"`, then one line per file in quest 22's format, joined with `"\n"` and no newline at the end.

**Tests that cannot hang.** Each test runs your pool on a helper thread with a 10-second limit, so a `Sender` you forgot to drop fails the test instead of freezing it. The tests make their own files in your system's temp folder and delete them afterwards.

```rust
// a borrowed counter, shared by scoped threads:
let done = AtomicUsize::new(0);
let done = &done;                        // a & reference is Copy
thread::scope(|s| {
    for _ in 0..4 {
        s.spawn(move || done.fetch_add(1, Ordering::Relaxed)); // move copies the &
    }
});
```

## Your turn

- Replace each `todo!()` in objective order: `Progress`, `fnv1a`, `walk`, `hash_all_tracked`, `hash_all`, `hash_all_rayon`, `report`. `hash_file` is given.
- Press Run to hash the quest's own `src` folder.
