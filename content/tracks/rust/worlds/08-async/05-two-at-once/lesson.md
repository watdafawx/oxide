Before it downloads, the app asks the server how big each episode is, to show "48 MB" next to it. Each answer takes 300 ms of waiting. With two episodes you would like to pay those 300 ms once, not twice.

**Try the obvious thing.** `let a = episode_size(1).await; let b = episode_size(2).await;` works, but the second question is only sent after the first answer came back. `.await` pauses *this* function until the value is ready, so two awaits in a row also wait in a row: about 600 ms.

**Wait for both.** `tokio::join!(f1, f2)` takes several futures, polls all of them, and finishes when the last one is done. The waits overlap, so the total is the slowest wait, not the sum. It hands back a tuple of the results, in the order you wrote the futures.

```rust
let (feed, cover) = tokio::join!(fetch_feed("daily"), fetch_cover("daily")); // one wait, not two
```

**No .await inside.** Pass the futures themselves: `join!(episode_size(1), episode_size(2))`. Writing `episode_size(1).await` inside would finish that one first and give `join!` a plain number. `join!` is itself a macro (the `!`) that writes the polling code, and it awaits for you.

**Still one task.** `join!` starts no thread and no new task. The current task polls both futures in turn, which is why it costs nothing. It is also why one future that refuses to give the thread back freezes its partner, the topic of the next quest.

**For the curious.** `join!` needs the futures written out. For a list whose length you only know at run time, `futures::future::join_all(list).await` does the same and gives back a `Vec`.

## Your turn

- In `sizes_in_turn`, await one size, then the other, and return both.
- In `sizes_together`, run both questions with `join!` and return its tuple.
