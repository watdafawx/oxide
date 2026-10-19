**Why now:** the next quest shares one download counter between many tasks. Before that, check that you know what happens to a download when its deadline passes or the user cancels.

Two rules from the last quest:

- **`select!`** runs the branch of the first future to finish and **drops** the others.
- **`timeout(dur, fut)`** wraps a future. If time runs out, the future is **dropped** and you get `Err(Elapsed)`.

A dropped future stops at its next `.await`. Nothing else cleans up after it, so a half-written `.part` file stays on disk unless you delete it. The boss does exactly that.

## Your turn

Answer the two questions below.
