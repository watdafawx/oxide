A 1% low over two hours of play is mostly about the first hour: a hitch now barely moves it. Kinetik's lows cover the last 10 seconds, so they react to what is happening on screen right now. `FrameWindow` keeps exactly those frames.

**A window by time.** The `History` in Readings kept a fixed number of readings. Frames come unevenly, 60 or 240 a second, so a fixed count would mean 4 seconds in one game and 16 in another. This window keeps a span of time instead: every frame newer than 10 seconds before the latest one.

**From the front.** Frames arrive in time order, so the oldest is always at the front of the `VecDeque` from Readings. New frames go on at the back with `push_back`; old ones leave from the front with `pop_front`.

**Push, then trim.** Add the new frame first. Then, while the frame at the front is older than the cutoff, `frame.time - WINDOW_SECONDS`, drop it. `front()` gives an `Option`, `None` for an empty window, so `is_some_and(|old| ..)` from the Widget world asks the question safely.

**A loop, not one pop.** After a long hitch, one new frame can push several old ones out at once. A `while` loop keeps popping as long as its condition holds. Here the same shape trims a ping list by count instead of by time:

```rust
self.pings.push_back(ping);
while self.pings.len() > 60 {
    self.pings.pop_front();
}
```

**Ten is enough.** Ten seconds holds about 1200 frames at 120 FPS: enough for a 1% low of 12 frames, short enough to react within seconds.

## Your turn

- In `FrameWindow::push`, push the frame at the back.
- Then pop from the front while the oldest frame is older than `frame.time - WINDOW_SECONDS`.
- Save.
