To test a pinger you need a server to ping, so this quest writes the other side: a tiny server that sends back whatever it receives, an **echo** server. It is the skeleton every Minecraft server shares: read, answer, and know when the client has left.

**`read` gives what is there.** `stream.read(&mut buf)` waits until some bytes arrive, copies them into `buf` and returns how many, `n`. That may be fewer than the buffer holds and fewer than the client wrote: TCP is a stream. Only `buf[..n]` is real; the rest is left over from before.

**Zero means goodbye.** When the client closes its end, `read` returns `Ok(0)`. That is how a server knows a player has disconnected. An error is something else: `Err`, which the `?` passes up.

**Write back just those.** `stream.write_all(&buf[..n])` sends exactly the bytes that arrived. Writing the whole `buf` would send 1024 bytes for a 5-byte ping.

**`loop`.** A packet bigger than the buffer comes in several reads, so the server repeats until it sees 0. `loop { ... }` repeats its block forever; a `return` or `break` inside is the only way out. It is the `while` from world 01 for when the reason to stop is found in the middle of the block, not at the top.

```rust
// Count every byte a client sends, until it disconnects.
let mut total = 0;
loop {
    let n = stream.read(&mut buf)?;
    if n == 0 {
        break;
    }
    total += n;
}
```

## Your turn

- In `echo_ping`, in a `loop`, read into `buf`; on 0 bytes give back `Ok(())`.
- Otherwise write back exactly the bytes that arrived.
