**Why now:** a download does not arrive in one piece: chunks of bytes show up over time. A **`Stream`** is the async version of an `Iterator`: you pull the next item with `.await`, and you shape it with `map`, `filter`, `take` and `fold`.

```rust
use futures::{stream, StreamExt};
use futures::future::ready;

let big: Vec<u64> = stream::iter(vec![500u64, 2048, 4096])
    .filter(|kb| ready(*kb >= 1024))   // filter takes a closure returning a future
    .map(|kb| kb / 1024)
    .collect()
    .await;                             // [2, 4]

let first_three: Vec<u64> = stream::iter(0u64..).take(3).collect().await;  // endless source, cut with take
```

- `fold` works like the iterator one, but the closure returns a future: `stream::iter(vec![5, 7]).fold(0, |acc, _| async move { acc + 1 }).await` counts the items (`2`).
- Add the `futures` crate and `use futures::StreamExt` to get these methods.
- Streams are lazy like iterators: nothing happens until you `collect`, `fold` or loop with `while let Some(x) = s.next().await`.
- `filter` and `fold` take closures that return **futures**: `ready(bool)` for `filter`, `async move { .. }` for `fold`.
- An endless stream plus `collect` without `take` never finishes. The tests time out to show you.

## Your turn

In `src/lib.rs`:

- `big_jars_in_mb`: keep sizes `>= 1024` KB, convert to MB.
- `first_n_chunk_offsets`: the byte offset of the first `n` chunks of 1024 bytes, from an endless stream.
- `total_name_length`: add up the lengths of all names with `fold`.
