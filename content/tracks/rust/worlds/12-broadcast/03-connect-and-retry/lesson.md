Rocket League opens the Stats API port when a match loads and closes it when you quit to the menu. The HUD runs all day: it reads while the game is there and quietly reconnects when it comes back.

**Async reads.** A tokio `TcpStream` reads with `.read(&mut buf).await`. While it waits for bytes, the thread is free for other tasks: the overlay socket, OSC sends, the Tauri app.

**Zero means closed.** `Ok(0)` is the game going away, not "nothing yet". Return and let `run_feed` connect again; it waits 1, 2, 4, 8, then 10 seconds between tries (`backoff`).

**Events into a channel.** `read_frames` does not decide anything; it sends every event into an `mpsc` channel for the rest of the server:

```rust
events.send(event).await?;
```

**Nobody listening.** `send` fails only when the receiver is gone, which means the server is shutting down. `?` hands that error back and `run_feed` stops.

## Your turn

- In `read_frames`, read into a buffer in a loop; stop on 0 bytes or an error.
- Push each read into a `FrameBuffer` and send every normalized event. Save.
