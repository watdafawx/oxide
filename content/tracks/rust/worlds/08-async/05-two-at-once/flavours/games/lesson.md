Before it downloads, the launcher asks the mod server how big each jar is, to show "4 MB" next to it. Each answer takes 300 ms of waiting. With two mods you would like to pay those 300 ms once, not twice.

**Try the obvious thing.** `let a = jar_size(1).await; let b = jar_size(2).await;` works, but the second question is only sent after the first answer came back. `.await` pauses *this* function until the value is ready, so two awaits in a row also wait in a row: about 600 ms.

**Wait for both.** `tokio::join!(f1, f2)` takes several futures, polls all of them, and finishes when the last one is done. The waits overlap, so the total is the slowest wait, not the sum. It hands back a tuple of the results, in the order you wrote the futures.

```rust
let (page, icon) = tokio::join!(fetch_page("sodium"), fetch_icon("sodium")); // one wait, not two
```

**No .await inside.** Pass the futures themselves: `join!(jar_size(1), jar_size(2))`. Writing `jar_size(1).await` inside would finish that one first and give `join!` a plain number. `join!` is itself a macro (the `!`) that writes the polling code, and it awaits for you.

**Still one task.** `join!` starts no thread and no new task. The current task polls both futures in turn, which is why it costs nothing. It is also why one future that refuses to give the thread back freezes its partner, the topic of the next quest.

**For the curious.** `join!` needs the futures written out. For a modpack whose length you only know at run time, `futures::future::join_all(list).await` does the same and gives back a `Vec`.

## Your turn

- In `jar_sizes_in_turn`, await one size, then the other, and return both.
- In `jar_sizes_together`, run both questions with `join!` and return its tuple.
