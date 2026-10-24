Rocket League's Stats API sends the game's events over TCP as JSON objects back to back: `{...}{...}{...}`, with no newline between them and no length in front. Before the HUD can react to anything, it has to cut that stream into whole objects, which the HUD calls **frames**.

**Bytes, not messages.** TCP delivers a stream of bytes, not a list of messages. One read may hold three frames and half of a fourth; the next read brings the rest. The code has to find where each object ends, and keep the half for later.

**Values one by one.** `serde_json::Deserializer::from_slice(bytes).into_iter::<Value>()` reads values from the bytes one at a time. Each `.next()` gives `Some(Ok(value))`, `Some(Err(e))` when the bytes are not a whole value, or `None` when nothing is left.

**A loop with a match.** The usual shape is a `loop` that calls `next()` and matches the answer, with a `return` in every arm that ends it. Here it is on a different stream, a list of scores, where junk simply stops the count:

```rust
let mut scores = serde_json::Deserializer::from_str("3 4 x").into_iter::<u32>();
let mut total = 0;
loop {
    match scores.next() {
        Some(Ok(score)) => total += score,
        Some(Err(_)) => return total,
        None => return total,
    }
}
```

**Where it got to.** `stream.byte_offset()` is how many bytes have been read so far. Ask it *before* each `next()`: if that value turns out to be unfinished, the offset you saved is exactly where the half frame starts.

**Unfinished is not broken.** When the last object is cut off, the error's `e.is_eof()` is `true`: it ran out of bytes ("end of file"). A match guard catches that case first, `Some(Err(e)) if e.is_eof() =>`, and keeps those bytes. Any other error is junk on the wire; count every byte as used so the junk is thrown away.

**The buffer.** `FrameBuffer::push` (already written) adds each read to what was left over, calls your function, and drains only the bytes it says were used. The half frame stays in `pending` for the next read.

## Your turn

- In `split_frames`, loop over the values: push each whole one into a list.
- Return the list with the bytes used: the saved offset for an unfinished value, all of them for junk, the final offset when the bytes ran out.
