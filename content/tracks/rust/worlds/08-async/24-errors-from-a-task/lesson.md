Each download task can fail on its own: the server may answer "big" where a number belongs. The task should report that with `?` like any other code, and the downloader should get one `Result` for the whole job. Two small snags stand in the way.

**One error type for all.** The functions here return `Error`, a name for `Box<dyn std::error::Error + Send + Sync>`: "any error", from world 04. The `Send + Sync` part matters now, since the error is made in one task and read in another, maybe on another thread.

**Try the obvious thing.** Write `?` in a spawned block and end it with `Ok(size)`. Rust stops with `error[E0282]: type annotations needed`. An `async` block has no signature, so nothing tells `?` which error type to turn a parse error into. Name both types on the last line with the turbofish from world 03: `Ok::<u64, Error>(size)`.

```rust
set.spawn(async move {
    let text = episode_count(&show).await;
    let count: u32 = text.parse()?;   // ParseIntError -> Error
    Ok::<u32, Error>(count)           // the block's types, spelled out
});
```

**A Result in a Result.** `join_next` now gives `Result<Result<u64, Error>, JoinError>`: the outer one says whether the *task* ended well (no panic, not cancelled), the inner one is what your block returned. `result??` peels both: the first `?` passes on a `JoinError`, the second your own error. `?` turns each into `Error` on the way out, so the function stays one `Result`.

**The first error wins.** As soon as `?` passes an error out of `total_size`, the function returns and drops its `JoinSet`, which aborts the tasks still running (quest 23). Nothing is left downloading in the background.

## Your turn

- In `total_size`, spawn one task per id into a `JoinSet`: await `size_text(id)`, parse it with `?`, and end with `Ok::<u64, Error>(..)`.
- Add up the results with `result??` as they come out, and return `Ok(total)`.
