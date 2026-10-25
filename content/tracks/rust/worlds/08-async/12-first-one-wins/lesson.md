Every episode is on two servers, the main one and a backup. Sometimes one is slow, and you don't know which. Ask both at once and take whichever answers first; there is no reason to wait for the other.

**Race them.** `tokio::select!` takes several branches, each a future with an arm after `=>`. It polls all the futures, and as soon as one finishes it runs that branch's arm and gives back its value. `join!` waits for all; `select!` waits for the first.

**Name the winner's value.** Each branch starts with a pattern: `feed = fetch_feed(..)` binds the finished future's output to `feed` for the arm to use, and `_ = ..` throws it away when only "it finished" matters.

```rust
use tokio::time::{sleep, Duration};

let feed = tokio::select! {
    feed = fetch_feed("daily") => feed,
    _ = sleep(Duration::from_secs(2)) => String::from("(no feed yet)"),
};
```

**The losers are dropped.** When one branch wins, `select!` drops every other future, so the slower request stops at its next `.await` and costs nothing more. The `timeout` from the last quest is a small `select!` like the one above: the work against a timer.

**For the curious.** If two branches are ready at the same poll, `select!` picks one at random, so neither always wins. Write `biased;` as its first line to try the branches top to bottom instead.

## Your turn

- In `first_answer`, ask both servers at the same time with `select!`.
- Return `"main"` or `"backup"`, for the one that answered first.
