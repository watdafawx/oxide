Open the multiplayer screen and the launcher shows every server's version, player count and coloured MOTD before you join. It gets them with the **Server List Ping**, a tiny conversation every Minecraft Java server answers. Now you write it, and aim it at your own server: every part is something this world built, from big-endian numbers and VarInts to a careful reader, TCP with timeouts and JSON.

**Three packets.** The whole conversation is one TCP connection and three packets. The launcher sends a **Handshake** that says "I want your status", then an empty **Status Request**. The server answers with a **Status Response**: one string of JSON.

**A packet's frame.** Every packet is `[VarInt length][VarInt id][payload]`. The length counts the id **plus** the payload, so the reader knows exactly how many bytes belong to this packet: TCP is a stream, and this is where one packet ends. All three packets here have id `0`.

**The handshake.** Its payload is four fields in a row: a VarInt protocol version (`767`, Minecraft 1.21.1), the host as a String, the port as a `u16` big-endian, and a VarInt next state, `1` for "status" (`2` would start a login). A protocol **String** is a VarInt byte length followed by the UTF-8 bytes. The handshake to `localhost:25565` is, byte for byte:

```text
10        length: 16 bytes follow
00        id 0
ff 05     VarInt 767, the protocol version
09 6c 6f 63 61 6c 68 6f 73 74    String: length 9, then "localhost"
63 dd     port 25565, big-endian
01        next state: status

then the Status Request:  01 00   (length 1, id 0, no payload)
```

**Building the bytes.** `vec.extend_from_slice(&bytes)` appends a whole slice to a `Vec<u8>`, the way `push` appends one byte. `write_varint` and `write_string` are already written and append to a `Vec` you pass in. Build the body first, then a second `Vec` with the body's length in front.

**Never trust the length.** `read_packet` reads the length with `read_varint`, and that number comes from the server. A fake server can say 268 million, and `vec![0u8; len]` would try to allocate it and take your launcher down. So check first: `(1..=MAX_PACKET_LEN).contains(&len)` is true when the length is in the range, and anything else is an error before a single byte is allocated. Then `read_exact` fills the buffer, as in the connect quest.

**Reading inside the frame.** The id is a VarInt at the start of the packet's buffer, but `read_varint` reads from a stream. `io::Cursor::new(&body[..])` is std's version of your cursor: it wraps a slice so it can be read like a stream, and `cur.position()` then says how many bytes the id took. The payload is the rest of the buffer. The given `read_string` does the same move; read it as an example.

**One error type.** The status JSON must have `version.name`, `version.protocol`, `players.online` and `players.max`: read them with `as_str`, `as_i64` and `as_u64`, as in the JSON quests, and cast the numbers to the struct's types with `as`. The given `invalid("...")` makes an `io::Error` of kind `InvalidData`, so a missing field and a dropped connection come back as the same `io::Result`.

**A MOTD in pieces.** `description`, the MOTD, is either a plain string or a **text component**: an object with a `text` and a list of `extra` parts, each with its own colour or bold, and each part is again a string or a component. `flatten_text` matches on the `Value` like the player sample did, and for each `extra` part calls itself, the recursion from world 06, and appends the result with `push_str`.

**Putting it together.** `ping` connects with `TcpStream::connect_timeout`, sets read and write timeouts (a server that accepts and then says nothing must not freeze the server list), sends the handshake with `addr.ip().to_string()` and `addr.port()`, then the status request, reads one packet, checks its id is `0`, and reads its payload as a String of JSON.

**For the curious.** Real servers also answer a **Ping** packet (id `1`, an 8-byte number) by sending the same 8 bytes back. Timing that round trip is how the launcher draws the green signal bars.

## Your turn

`src/lib.rs` already has `write_varint`, `read_varint`, `write_string` and `read_string`. The tests run a fake server on `127.0.0.1`. Fill in, in this order:

- `Packet::encode`, `status_request` and `handshake`.
- `read_packet`, checking the length before allocating.
- `parse_status` and `flatten_text`, then `ping` with timeouts.

Press Run to ping a fake server, or `cargo run -- 192.168.50.3:25565` to ping your own server on your network.
