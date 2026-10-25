Notch and Resolume take their cues as OSC: small UDP packets with an address like `/notch/30s` and a few arguments. Crates for it exist, but the format is small enough to write by hand, and writing it shows exactly what goes over the wire.

**Three parts.** An OSC message is an address, a type-tag string, then the arguments. The type tags are a comma and one letter per argument: `i` for a 32-bit whole number, `f` for a 32-bit decimal, `s` for text. So `/notch/goal` with `3` and `"Wren"` has the tags `,is`.

**Four-byte steps.** Every string ends with a 0 byte and is padded with more zeros to a multiple of 4 bytes, so each field starts on a 4-byte boundary. `pad_str` (already written) appends a string that way, for the address, the tags and text arguments alike.

**Build the tags letter by letter.** Start a `String` with the comma, then `push` one `char` per argument; a `match` can sit right inside the `push`. The same shape on players, one letter per team:

```rust
let mut lineup = String::from("teams:");
for player in &players {
    lineup.push(match player.team_num {
        0 => 'B',
        _ => 'O',
    });
}
```

**Big-endian numbers.** Numbers are 4 bytes, most significant byte first: the network order from the Systems world. `v.to_be_bytes()` gives those 4 bytes for an `i32` or an `f32`, and `out.extend_from_slice(&bytes)` appends them.

**One packet.** UDP sends the whole message as one **datagram**, a packet that arrives whole or not at all: no connection, no stream to split. A lost cue is simply lost, which is fine for cues the show sends again and again.

## Your turn

- In `encode`, write the address with `pad_str`, then build the type tags and write them with `pad_str` too.
- Then write each argument: ints and floats as big-endian bytes, strings with `pad_str`. Save.
