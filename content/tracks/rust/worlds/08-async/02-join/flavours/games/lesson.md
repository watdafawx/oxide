**Why now:** a mod pack has 80 jars. Downloading them one after the other waits for each network round-trip in turn. **`join!`** starts several futures together and waits for all of them, so the waits overlap.

```rust
use std::time::Duration;
use tokio::time::sleep;

async fn fetch_size(mod_id: u32) -> u32 {
    sleep(Duration::from_millis(300)).await;   // pretend: a network call
    mod_id * 100
}

// one after the other: about 600 ms
let a = fetch_size(1).await;
let b = fetch_size(2).await;

// together: about 300 ms
let (a, b) = tokio::join!(fetch_size(1), fetch_size(2));
```

- Sequential `.await`s run one at a time, even though each one is async.
- `join!` polls all its futures on the same task and returns every result in a tuple.
- Use `tokio::time::sleep`, never `std::thread::sleep`, which would block the whole runtime thread.
- The timing test checks that your two waits really overlap.

## Your turn

In `src/lib.rs`:

- `queue_twice`: await `queue_line` twice and join the lines with a space.
- `fetch_both`: fetch both sizes with `join!`.
- `total_kb`: fetch both with `join!` and add the sizes.
