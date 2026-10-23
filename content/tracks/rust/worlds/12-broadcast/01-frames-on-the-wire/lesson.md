This world builds a server for a Rocket League broadcast HUD. Rocket League's official Stats API sends the game's events over TCP as JSON objects back to back: `{...}{...}{...}`, with no newline between them and no length in front.

**Bytes, not messages.** TCP delivers a stream of bytes. One read may hold three events and half of a fourth; the next read brings the rest. The code has to find where each object ends.

**Values one by one.** serde_json can read a stream of values. `into_iter` gives them one by one, and `byte_offset()` says how far it got:

```rust
let mut stream = serde_json::Deserializer::from_slice(bytes).into_iter::<Value>();
```

**Unfinished is not broken.** When the last object is cut off, the error says `is_eof()`: it ran out of bytes. Keep those bytes. Any other error is junk on the wire; throw the rest away.

**The buffer.** `FrameBuffer::push` adds each read to what was left over, takes out the whole frames, and drains only the bytes they used.

## Your turn

- Write `split_frames`: values until the bytes end, the start of an unfinished last value, or junk.
- Save. The second objective feeds a whole recorded match through in 700-byte pieces.
