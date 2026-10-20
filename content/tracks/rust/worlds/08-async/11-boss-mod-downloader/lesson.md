**Why now:** this is the finished mod downloader: a modpack manifest in, every file out, concurrent, retried, verified and cancellable.

Given a server address, download every file of a modpack, fast, safely, and cancellably. Everything you need was in this world.

The test server (in `src/lib.rs`, read it if you are curious) runs on `127.0.0.1` and serves:

- `GET /manifest.json`: a JSON list of `{ "name", "path", "size" }`.
- `GET <path>`: the file bytes. Some files are slow or fail on purpose.

The job, in stages (each one is an objective):

1. `Entry::url`: join base address and file path.
2. `fetch_manifest`: download and parse the manifest (reqwest + serde).
3. `download_one`: fetch a file, write `name.part`, **rename** to `name` when complete. A half-written `.part` is never mistaken for a real file.
4. `download_with_retry`: if the server answers **5xx**, try once more. A 404 is not retried.
5. `download_all`: many files at once, but at most `max_concurrent` (**Semaphore** + **JoinSet**). After each file, send `(name, done, total)` on the **mpsc** channel.
6. `verify`: compare file sizes on disk to the manifest, list the bad names.
7. **Cancellation**: when the **CancellationToken** fires, stop everything, delete `.part` files, return an error.

```rust
let sem = Arc::new(Semaphore::new(2));
let permit = sem.clone().acquire_owned().await?; // waits if 2 are already running
// ...permit dropped at end of scope: slot freed
```

**New tools for the stages** (the hints show the exact calls)

- Text: `base.trim_end_matches('/')` cuts trailing slashes; `format!("{a}{b}")` glues text.
- One request on a shared client: `client.get(url).send().await?` then `.error_for_status()?`, and `.bytes().await?` gives the body as bytes. Clone a `reqwest::Client` freely: clones share one connection pool.
- Files, async flavour: `tokio::fs::write(&path, &bytes).await?` and `tokio::fs::rename(&from, &to).await?`. `dir.join("a.jar")` builds a path.
- Retry: `match first { Err(e) if is_server_error(&e) => ..., other => other }`. The `if` after a pattern is a **match guard**: the arm only applies when it is true.
- Many tasks, one set: `let mut set = JoinSet::new(); set.spawn(async move { ... });` then `while let Some(res) = set.join_next().await { ... }` yields each task's result as it finishes. `set.abort_all()` cancels them all.
- `res??`: `join_next` gives `Result<Result<T>, JoinError>`. The first `?` unwraps the task result, the second your own `Result`.
- A spawned async block that uses `?` needs its error type spelled out: end it with `Ok::<String, Error>(name)`.
- Limiting: `Semaphore::new(n)` hands out `n` permits; `sem.acquire_owned().await?` waits for one (needs the `Arc<Semaphore>`), and the permit frees its slot when dropped.

Go stage by stage and run the tests after each one: the first objective turns green within a minute.

## Your turn

In `src/lib.rs`: fill in every `todo!()`. `is_server_error` and `remove_partials` are given.

Press Run to play it.
