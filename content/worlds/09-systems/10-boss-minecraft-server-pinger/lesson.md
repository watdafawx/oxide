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

## Your turn

`src/lib.rs` already has `write_varint`, `read_varint`, `write_string`, `read_string`. Tests run a fake server on `127.0.0.1`. Fill in, in this order: `Packet::encode`, `status_request`, `handshake`, `read_packet` (never trust the length blindly), `parse_status` (+ `flatten_text`), `ping` (set timeouts).

Press Run to play it, or `cargo run -- 192.168.50.3:25565` to ping your own server. Real servers also answer a Ping packet (id `0x01`, 8 bytes) with the same 8 bytes: that is how the launcher shows latency.
