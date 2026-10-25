A jar download doesn't arrive in one piece. The bytes come in chunks over time, with a wait before each one, and the launcher wants to fill its progress bar as each chunk lands. A `Vec` has all its items now; this needs a list that hands out one item at a time, with an `.await` in between.

**A stream.** A **stream** is the async version of an iterator from world 03. Where an iterator's `next()` gives the next item straight away, a stream's `next().await` waits for it: `Some(item)`, or `None` when the stream is over. A `while let Some(chunk) = s.next().await` loop reads one.

**The same tools.** The `futures` crate's `StreamExt` trait adds the methods you know from iterators: `map`, `filter`, `take`, `collect`, `fold`. They are lazy like iterators, and nothing runs until you `collect` or loop.

**From any iterator.** `stream::iter(list)` turns a `Vec`, a range or any iterator into a stream, which is handy for tests. `stream::iter(0u64..)` is endless, since `0u64..` is a range with no end, so cut it with `.take(n)` before `collect`, or the launcher waits forever.

```rust
use futures::{stream, StreamExt};
use futures::future::ready;

let mods = stream::iter(vec!["jei", "sodium", "iris"]);
let long: Vec<usize> = mods
    .filter(|name| ready(name.len() > 3)) // the test is wrapped in ready(..)
    .map(|name| name.len())
    .collect()
    .await; // [6, 4]
```

**Why ready.** The closure for `filter` may itself want to wait, for example to ask the mod server, so it must give back a future of a `bool`. When the answer is already known, `ready(x)` is a future that is finished from the start. `map` takes a plain closure.

**For the curious.** `fold` takes a closure that gives back a future too, written in the older spelling from quest 09: `.fold(0, |acc, kb| async move { acc + kb })`.

## Your turn

- In `big_jars_in_mb`, keep jars of at least 1024 KB and divide each by 1024.
- In `first_n_chunk_offsets`, turn the endless chunk numbers into offsets (times 1024) and keep the first `n`.
