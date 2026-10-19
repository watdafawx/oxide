**Why now:** a modpack download arrives chunk by chunk, so the downloader reads it as a stream instead of waiting for a whole `Vec`.

A `Vec` has all its items now. A **stream** gives them one at a time, with `.await` between: chunks of a download, events from a server.

- A stream is an **async iterator**: `stream.next().await` returns `Some(item)` or `None`.
- Import **`StreamExt`** (from the `futures` crate) to get `map`, `filter`, `take`, `fold`, `collect`, ...
- `stream::iter(vec)` turns any iterator into a stream. Great for tests.
- Closures in `filter` and `fold` return a **future**, not a plain value. `futures::future::ready(x)` or `async move { x }` does it.

```rust
use futures::{stream, StreamExt};
use futures::future::ready;

let mods = stream::iter(vec!["jei", "sodium", "iris"]);
let long: Vec<usize> = mods
    .filter(|w| ready(w.len() > 3))
    .map(|w| w.len())
    .collect()
    .await; // [6, 4]
```

Like iterators, streams are lazy: nothing happens until you `collect`, `fold` or loop over them.

## Your turn

In `src/lib.rs` (`futures` is already a dependency):

- `doubled_evens`: keep the even numbers, double them.
- `first_n_squares`: squares of `1, 2, 3, ...` from an endless stream, but only the first `n`.
- `total_length`: add up the length of every word with `fold`.
