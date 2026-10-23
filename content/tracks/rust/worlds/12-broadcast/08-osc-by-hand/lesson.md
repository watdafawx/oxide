Notch (broadcast graphics) and Resolume (stage video) take cues as OSC: small UDP packets with an address like `/notch/30s` and a few arguments. Crates exist, but the format is small enough to write by hand.

**Three parts.** An address, a type-tag string, then the arguments. The type tags are a comma and one letter per argument: `i` for a 32-bit int, `f` for a float, `s` for a string.

**Four-byte steps.** Every string ends with a 0 byte and is padded with zeros to a multiple of 4. `pad_str` does that.

**Big-endian numbers.** Numbers are 4 bytes, most significant first, the network order from the bytes world:

```rust
OscArg::Float(v) => out.extend_from_slice(&v.to_be_bytes()),
```

**One packet.** UDP sends the whole message as one datagram: no connection, no stream to split. If it is lost, it is lost, which is fine for a cue sent every few seconds.

## Your turn

- In `encode`, write the address, then the type tags, both with `pad_str`.
- Write each argument: ints and floats big-endian, strings padded. Save.
