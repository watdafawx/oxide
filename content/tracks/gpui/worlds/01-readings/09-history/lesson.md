Hover a stat on Kinetik's bar and you get a graph of its last minute. At one reading a second, that is the last 60 values: each second a new one arrives and the oldest falls off. Your `History` keeps exactly that.

**Not a plain Vec.** A `Vec` is quick to add to at the end, but taking the first item out shifts every other item one place left, every second, for every stat. There is a better tool.

**The word for it.** A **VecDeque** (say "vec-deck", a double-ended queue) is a list that is quick to change at both ends. `push_back` adds the newest reading at the back, `pop_front` takes the oldest off the front.

**Both ends.** A queue of Kinetik's last alerts uses the same two calls: the newest goes in at the back, and the oldest comes off the front. `pop_front` hands back an `Option`, `None` when the queue is empty:

```rust
let mut alerts = VecDeque::new();
alerts.push_back("CPU hot");
alerts.push_back("Battery low");
alerts.pop_front(); // Some("CPU hot"), the oldest
```

**Push, then trim.** `push` already adds at the back. After that, the history may hold one reading too many, and then the oldest has to go. Which comparison with `cap` says "one too many"? A history of 60 must end up with exactly 60.

**A cap you choose.** Kinetik lets you pick 30 seconds, 1, 2 or 5 minutes of graph. That is `History::new(30)` to `History::new(300)`, with no other change.

**For the curious.** A `VecDeque` is a ring buffer: one block of memory with a start that moves. Popping the front moves the start one place instead of shifting every value, which is why both ends are quick.

## Your turn

- In `push`, under `push_back`, drop the oldest reading when the history now holds more than `self.cap`.
- Save. Press Run to push 100 readings into a 60-reading history.
