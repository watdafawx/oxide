Kinetik reads its sensors once per update interval, one second by default. The spawned block should wait that long before reading. How it waits matters: the window has to keep drawing and answering the mouse while the bar waits.

**Do not sleep.** The spawn block runs on the app's own thread. `std::thread::sleep` there stops that thread, and the whole window freezes for a second.

**Await a timer.** `cx.background_executor().timer(interval)` gives a future that finishes after `interval`. `.await` pauses only this block; GPUI goes on drawing and comes back when the time is up:

```rust
cx.background_executor().timer(interval).await;
```

**Copy before you go.** The block cannot borrow `self`, so `start` copies `self.interval` into `interval` first. A `Duration` is small and `Copy`.

**Time in tests.** Tests do not wait a real second. `advance_clock` moves GPUI's test clock forward, so the test checks 999 ms and 1000 ms in no time at all.

## Your turn

- In `start`, at the top of the `spawn` block, await the timer for `interval`.
- Save.
