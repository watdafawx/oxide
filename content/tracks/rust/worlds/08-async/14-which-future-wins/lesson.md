`timeout` and `select!` both stop work by dropping a future. Before the downloader leans on that for its Cancel button, be sure what dropping does and what it doesn't. Two of the boss's objectives depend on it.

**It stops at an await.** A dropped future is never polled again. It stops at the `.await` where it was paused, and the code after that line never runs. A future that never reaches an `.await`, such as a long loop without one, can't be stopped this way.

**It doesn't undo anything.** Whatever the future did before it stopped stays done. If it had written half an episode to `episode-7.mp3.part`, that file is still on disk. The usual fix: write to a `.part` name, rename only when complete, and on cancel delete the leftovers yourself. The boss does exactly that.

**Spawned tasks are different.** A task from `tokio::spawn` belongs to the runtime, not to its handle. Dropping the `JoinHandle` only throws away the ticket; the task runs on until it ends. To stop tasks you send them a signal (quest 18) or abort them (quest 23).

**For the curious: cancel safety.** If a `select!` loop drops a branch half way and then starts it again, any progress inside that future is lost. A call that loses nothing when dropped this way is called **cancel safe**; tokio's docs say so on each call, and `mpsc::Receiver::recv` is one.

## Your turn

- Answer the questions. No code this time.
