The finished mod downloader: given a mod server's address, download every jar of a modpack, at most a few at a time, with a retry for flaky mirrors, a progress line per jar, a check of what landed in the mods folder, and a Cancel button that leaves nothing half-written behind. Every tool came from this world, and each card says which quest.

**The test server.** The tests start a small HTTP server on `127.0.0.1` (it is in `src/lib.rs`, read it if you are curious). `GET /manifest.json` answers with a JSON list of `{ "name", "path", "size" }`, and `GET <path>` with a jar's bytes. Some jars are slow or fail on purpose.

**One jar first.** `Entry::url` glues the base address and the path. `fetch_manifest` is quests 20 and 21; `reqwest::get(url)` is the shortcut for one request without a `Client`. `download_one` writes the bytes to `<name>.part` and only then renames it to `<name>`, so a half-written jar is never loaded by the game (quest 14). `tokio::fs::write` and `tokio::fs::rename` are the async twins of world 04's `std::fs` calls.

**Retry once.** A 5xx may pass, a 404 never will (quest 19). `download_with_retry` is a `match` with a guard, the `Err(e) if ..` arm from world 04, and the starter's `is_server_error` tells the two apart.

**Many at once.** `download_all` spawns one task per entry into a `JoinSet` (quest 23). Each takes a `Semaphore` permit first (quest 22), uses `?` inside and ends with `Ok::<String, Error>(name)` (quest 24). A task must own what it uses (quest 07), so each gets its own copies:

```rust
for entry in entries {
    let client = client.clone(); // cheap: shares the connections (quest 20)
    let dir = dir.to_path_buf(); // an owned PathBuf from a borrowed &Path
    set.spawn(async move { /* uses client, dir and entry */ });
}
```

**Progress and cancel.** Wait in a `loop` around `select!` (quest 18): one branch is `token.cancelled()`, the other `set.join_next()`. A finished task is peeled with `result??`, counted, and reported with `progress.send((name, done, total)).await` (quest 10); write `let _ =` in front, since a closed channel only means the launcher UI is not watching. On cancel: `set.abort_all()`, the given `remove_partials(dir).await`, and `Err("cancelled".into())`.

**Verify.** `std::fs::metadata(path)` gives `Ok(meta)` with `meta.len()` in bytes, or an `Err` when the jar is missing. `verify` is plain code, not async, so the `std` version is right here. Go stage by stage and run the tests after each; the first objective turns green within a minute.

## Your turn

- Fill in every `todo!()` in `src/lib.rs`, in objective order. `is_server_error` and `remove_partials` are given.
- When all stages pass, `main.rs` downloads a small modpack from a fake mod server and prints the progress.
