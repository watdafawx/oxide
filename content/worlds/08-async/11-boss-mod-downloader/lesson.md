Time to build the heart of a mod manager: given a server address, download every file of a modpack, fast, safely, and cancellably. Everything you need was in this world.

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

Go stage by stage and run the tests after each one: the first objective turns green within a minute.

## Your turn

In `src/lib.rs`: fill in every `todo!()`. `is_server_error` and `remove_partials` are given.

Press Run to play it.
