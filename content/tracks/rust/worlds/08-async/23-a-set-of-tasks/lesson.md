Quest 07 kept a `Vec` of handles and awaited them in order. If the first episode is the slowest, the progress bar shows nothing until it is done, although the others finished long ago. The downloader wants each result the moment it lands, and a way to stop all the leftovers at once.

**A bag of tasks.** A `tokio::task::JoinSet` holds many tasks of the same result type. `set.spawn(fut)` works like `tokio::spawn` and puts the new task in the set, and you don't keep any handles yourself.

**First come, first out.** `set.join_next().await` waits for whichever task in the set finishes next and gives `Some(result)`, or `None` once the set is empty. The result is the `Result<T, JoinError>` you know from awaiting a handle. A `while let` loop over it sees the results in finish order, not spawn order.

```rust
use tokio::task::JoinSet;

let mut set = JoinSet::new();
for show in ["daily", "tech", "sport"] {
    set.spawn(refresh_feed(show)); // each feed in its own task
}
while let Some(result) = set.join_next().await {
    println!("refreshed: {:?}", result.unwrap()); // fastest first
}
```

**Cancel the rest.** `set.abort_all()` cancels every task still in the set: each one stops at its next `.await`. Their results still come out of `join_next`, as an `Err` whose `is_cancelled()` is true, so you can count them or simply let them pass.

**Unlike a handle.** Dropping a lone `JoinHandle` leaves its task running (quest 14). Dropping a whole `JoinSet` aborts every task in it, so tasks can't outlive the set you forgot about. The boss uses this: it holds its downloads in a `JoinSet` and calls `abort_all` on Cancel.

## Your turn

- In `finish_order`, spawn every download into one `JoinSet`, then collect the ids in the order they come out.
- In `first_and_cancel_rest`, keep the first finished id, abort the rest, and count the results that say they were cancelled.
