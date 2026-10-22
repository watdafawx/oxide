Hover a stat on Kinetik's bar and you get a graph of its last minute. At one reading a second, that is the last 60 values: each second a new one arrives and the oldest falls off. Your `History` keeps exactly that.

**Not a plain Vec.** A `Vec` is quick to add to at the end, but taking the first item out shifts every other item one place left, every second, for every stat. There is a better tool.

**The word for it.** A **VecDeque** (say "vec-deck", a double-ended queue) is a list that is quick to change at both ends. `push_back` adds the newest reading at the back, `pop_front` takes the oldest off the front.

**Push, then trim.** `push` already adds at the back. After that, if there are more than `cap` values, the oldest goes:

```rust
if self.values.len() > self.cap { self.values.pop_front(); }
```

**A cap you choose.** Kinetik lets you pick 30 seconds, 1, 2 or 5 minutes of graph. That is `History::new(30)` to `History::new(300)`, with no other change.

## Your turn

- In `push`, under `push_back`, add the `if` that pops the front when the length is over `self.cap`.
- Save. Press Run to push 100 readings into a 60-reading history.
