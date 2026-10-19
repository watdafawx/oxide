A `Vec` has all its items now. A **stream** gives them one at a time, with `.await` between: lines arriving from a socket, events from a server, chunks of a download.

- A stream is an **async iterator**: `stream.next().await` returns `Some(item)` or `None`.
- Import **`StreamExt`** (from the `futures` crate) to get `map`, `filter`, `take`, `fold`, `collect`, ...
- `stream::iter(vec)` turns any iterator into a stream. Great for tests.
- Closures in `filter` and `fold` return a **future**, not a plain value. `futures::future::ready(x)` or `async move { x }` does it.

```rust
use futures::{stream, StreamExt};
use futures::future::ready;

let words = stream::iter(vec!["a", "bb", "ccc"]);
let long: Vec<usize> = words
    .filter(|w| ready(w.len() > 1))
    .map(|w| w.len())
    .collect()
    .await; // [2, 3]
```

Like iterators, streams are lazy: nothing happens until you `collect`, `fold` or loop over them.

## Your turn

In `src/lib.rs` (`futures` is already a dependency):

- `doubled_evens`: keep the even numbers, double them.
- `first_n_squares`: squares of `1, 2, 3, ...` from an endless stream, but only the first `n`.
- `total_length`: add up the length of every word with `fold`.
