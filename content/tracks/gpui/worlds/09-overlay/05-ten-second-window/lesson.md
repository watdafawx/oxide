A 1% low over two hours of play is mostly about the first hour; a hitch now barely moves it. Kinetik's lows cover the last 10 seconds, so they react to what is happening on screen.

**A window by time.** History in Readings kept a number of readings. Frames come unevenly, 60 or 240 a second, so this window keeps a span of time: every frame newer than 10 seconds before the latest.

**Push, then trim.** Add the new frame at the back. Then, while the frame at the front is older than the cutoff, drop it. One new frame can push several old ones out:

```rust
while self.frames.front().is_some_and(|old| old.time < frame.time - WINDOW_SECONDS) {
```

**From the front.** Frames arrive in time order, so the oldest is always at the front: the `VecDeque` from Readings again.

**Ten is enough.** Ten seconds holds about 1200 frames at 120 FPS: enough for a 1% low of 12 frames, short enough to react within seconds.

## Your turn

- In `FrameWindow::push`, push the frame at the back.
- Then pop from the front while the oldest is older than `frame.time - WINDOW_SECONDS`.
- Save.
