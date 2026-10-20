**Why now:** this is the finished server pinger: the same ping the launcher's server list sends, aimed at your own server.

The launcher shows a name, player count and version without logging in, using the **Server List Ping**, a tiny TCP conversation. You write the client.

## The conversation

```
client -> server   Handshake      (next state = 1)
client -> server   Status Request (empty)
server -> client   Status Response (one JSON string)
```

Everything is a **packet**: `[VarInt length][VarInt id][payload]`. *Length* counts the id **plus** the payload. All three packets here have id `0x00`.

- **Handshake** payload: VarInt protocol version (`767` = 1.21.1), String host, unsigned short port (big-endian), VarInt next state (`1`).
- **Status Request**: no payload. **Status Response**: one String, the JSON.
- A **String** is a VarInt byte length, then UTF-8 bytes. Handshake to `localhost:25565` is `10 00 ff 05 09 "localhost" 63 dd 01` (`0x10` = 16 = everything after it).

```json
{"version":{"name":"1.21.1","protocol":767},
 "players":{"max":20,"online":3},
 "description":{"text":"Hello ","extra":[{"text":"world"}]}}
```

`description` may be a plain string or a **text component** (`text` plus nested `extra` parts). Flatten it to plain text.

**Small tools you will need**

- `vec.extend_from_slice(&other)` appends a slice to a `Vec<u8>`; `port.to_be_bytes()` gives the 2 big-endian bytes of a `u16`.
- `(1..=MAX).contains(&len)` is `true` when `len` is in the range; `!` flips it.
- A `Vec` of the right size to fill: `vec![0u8; len as usize]`; `r.read_exact(&mut buf)?` fills it.
- `io::Cursor::new(&buf[..])` reads a buffer like a stream, and `cur.position()` says how far it got: use it to find where the payload starts after the id VarInt.
- JSON: `v["players"]["online"]` digs in (a missing key is `Null`, never a panic); `.as_u64()` / `.as_i64()` / `.as_str()` / `.as_array()` give `Option`s. A `Value` is an enum, so `match v { Value::String(s) => ..., Value::Object(_) => ..., _ => ... }` tells the cases apart. `serde_json::from_str::<Value>(text)` parses a `&str`.
- A function may call itself: for `extra` parts that hold more parts, call `flatten_text` on each part and `push_str` the result.
- `TcpStream::connect_timeout(&addr, Duration::from_secs(3))?` connects but gives up after the time; `set_read_timeout` and `set_write_timeout` do the same for reads and writes.

## Your turn

`src/lib.rs` already has `write_varint`, `read_varint`, `write_string`, `read_string`. Tests run a fake server on `127.0.0.1`. Fill in, in this order: `Packet::encode`, `status_request`, `handshake`, `read_packet` (never trust the length blindly), `parse_status` (+ `flatten_text`), `ping` (set timeouts).

Press Run to play it, or `cargo run -- 192.168.50.3:25565` to ping your own server. Real servers also answer a Ping packet (id `0x01`, 8 bytes) with the same 8 bytes: that is how the launcher shows latency.
