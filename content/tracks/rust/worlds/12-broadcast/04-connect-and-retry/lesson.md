Rocket League opens the Stats API port when a match loads and closes it when you quit to the menu. The HUD runs all day: it reads while the game is there and quietly connects again when it comes back.

**Async reads.** A tokio `TcpStream` reads with `stream.read(&mut chunk).await`, which fills the start of a byte array and gives back how many bytes it wrote. While it waits for bytes, the thread is free for other tasks: the overlay socket, OSC sends, the Tauri window.

**Zero means closed.** `Ok(0)` is the game going away, not "nothing yet": a read that has nothing waits instead of returning. An `Err` means the connection broke. Both end this connection; `run_feed` (already written) then connects again, waiting 1, 2, 4, 8, then 10 seconds between tries.

**Two cases, one arm.** In a pattern, `|` means "either": `Ok(0) | Err(_) =>` runs for a closed stream and for a broken one. The read loop has this shape; here it only counts bytes:

```rust
let mut chunk = [0u8; 4096];
let mut total = 0;
loop {
    match stream.read(&mut chunk).await {
        Ok(0) | Err(_) => return total,
        Ok(n) => total += n,
    }
}
```

**Only what was read.** After a read of `n` bytes, only `&chunk[..n]` is new; the rest of the array is left over from before. Push that slice into a `FrameBuffer`, which hands back the whole frames, and `normalize` each one.

**Events into a channel.** `read_frames` decides nothing; it sends every event into an `mpsc` channel for the rest of the server with `events.send(event).await`. `send` fails only when the receiving end is gone, which means the server is shutting down, so `?` hands that error back and `run_feed` stops.

## Your turn

- In `read_frames`, make a `FrameBuffer` and a 4096-byte array, and read in a loop; return `Ok(())` on 0 bytes or an error.
- Push each read's bytes into the buffer, and send every frame that normalizes to an event. Save.
