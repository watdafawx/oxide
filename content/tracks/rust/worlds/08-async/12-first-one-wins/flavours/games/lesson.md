Every mod is on two mirrors. Sometimes one is slow, and you don't know which. Ask both at once and take whichever answers first; there is no reason to wait for the other.

**Race them.** `tokio::select!` takes several branches, each a future with an arm after `=>`. It polls all the futures, and as soon as one finishes it runs that branch's arm and gives back its value. `join!` waits for all; `select!` waits for the first.

**Name the winner's value.** Each branch starts with a pattern: `page = fetch_page(..)` binds the finished future's output to `page` for the arm to use, and `_ = ..` throws it away when only "it finished" matters.

```rust
use tokio::time::{sleep, Duration};

let page = tokio::select! {
    page = fetch_page("sodium") => page,
    _ = sleep(Duration::from_secs(2)) => String::from("(mod site is slow)"),
};
```

**The losers are dropped.** When one branch wins, `select!` drops every other future, so the slower request stops at its next `.await` and costs nothing more. The `timeout` from the last quest is a small `select!` like the one above: the work against a timer.

**For the curious.** If two branches are ready at the same poll, `select!` picks one at random, so neither always wins. Write `biased;` as its first line to try the branches top to bottom instead.

## Your turn

- In `fastest_mirror`, ping both mirrors at the same time with `select!`.
- Return `"mirror-a"` or `"mirror-b"`, for the one that answered first.
