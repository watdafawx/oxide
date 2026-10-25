Kinetik reads its sensors once per update interval, one second by default. The spawned block should wait that long before reading. How it waits matters: the window has to keep drawing and answering the mouse while the bar waits.

**Do not sleep.** The spawn block runs on the app's own thread. `std::thread::sleep` there stops that thread, and the whole window freezes for a second.

**Await a timer.** The `AsyncApp` inside the spawn can reach GPUI's **executors**, the parts that run async work. `cx.background_executor().timer(d)` gives a future that finishes after the `Duration` `d`, and `.await` pauses only this block: GPUI goes on drawing and comes back when the time is up. A quarter-second pause:

```rust
cx.background_executor().timer(Duration::from_millis(250)).await;
```

**Copy before you go.** The block runs after `start` has returned, so it cannot borrow `self`: that is what `move` and the weak `this` are for. `start` copies `self.interval` into `interval` first. A `Duration` is small and `Copy`.

**Time in tests.** Tests do not wait a real second. `advance_clock` moves GPUI's test clock forward, so the test checks 999 ms and 1000 ms in no time at all.

## Your turn

- In `start`, at the top of the `spawn` block, await a timer for `interval`.
- Save.
