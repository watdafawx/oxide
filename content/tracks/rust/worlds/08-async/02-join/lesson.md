**Why now:** the downloader must fetch every jar of a modpack together, not one after another; `join!` is the first step to that.

Five downloads one after another take five times as long as together. **`tokio::join!`** starts several futures and waits until all are done.

- `a.await; b.await;` runs them **in sequence**: total time = a + b.
- `join!(a, b)` runs them **together**: total time = the slower one.
- It returns a tuple, one result per future, in the order you wrote them.
- Do not `.await` the futures yourself inside `join!`.

```rust
use std::time::Duration;
use tokio::time::sleep;

async fn fetch_jar(id: u32) -> u32 {
    sleep(Duration::from_millis(100)).await; // wait without blocking the thread
    id
}

async fn both() -> (u32, u32) {
    tokio::join!(fetch_jar(1), fetch_jar(2)) // ~100 ms total
}
```

`sleep(..).await` is the async way to wait. `std::thread::sleep` would freeze the whole thread (more on that in the read quest).

## Your turn

In `src/lib.rs` (`slow_double` and `greet` are given):

- `greet_twice`: `.await` `greet(name)` two times, one after the other, and join the results with a space. This is the sequential version to compare with.
- `double_both`: run `slow_double(a)` and `slow_double(b)` with `join!`.
- `sum_of_both`: same, but return the sum.
