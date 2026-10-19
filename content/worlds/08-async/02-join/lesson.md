Downloading five files one after another takes five times as long as downloading them together. **`tokio::join!`** starts several futures and waits until all are done.

- `a.await; b.await;` runs them **in sequence**: total time = a + b.
- `join!(a, b)` runs them **together**: total time = the slower one.
- It returns a tuple, one result per future, in the order you wrote them.
- Do not `.await` the futures yourself inside `join!`.

```rust
use std::time::Duration;
use tokio::time::sleep;

async fn wait_and_return(n: u32) -> u32 {
    sleep(Duration::from_millis(100)).await; // wait without blocking the thread
    n
}

async fn both() -> (u32, u32) {
    tokio::join!(wait_and_return(1), wait_and_return(2)) // ~100 ms total
}
```

`sleep(..).await` is the async way to wait. `std::thread::sleep` would freeze the whole thread (more on that in the read quest).

## Your turn

In `src/lib.rs` (`slow_double` is given):

- `double_both`: run `slow_double(a)` and `slow_double(b)` with `join!`.
- `sum_of_both`: same, but return the sum.
