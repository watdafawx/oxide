Every server list in the Minecraft launcher shows a name, a player count and a version without logging in. It does that with the **Server List Ping**, a tiny TCP conversation. You will write the client, and then you can point it at your own server.

## The conversation

```
client -> server   Handshake      (next state = 1)
client -> server   Status Request (empty)
server -> client   Status Response (one JSON string)
```

Everything is a **packet**: `[VarInt length][VarInt id][payload]`, where *length* counts the id **plus** the payload.

| Packet | id | payload |
|---|---|---|
| Handshake (C->S) | `0x00` | VarInt protocol version (`767` = 1.21.1), String host, Unsigned Short port (big-endian), VarInt next state (`1`) |
| Status Request (C->S) | `0x00` | nothing |
| Status Response (S->C) | `0x00` | String (the JSON) |

A **String** is a VarInt byte length, then UTF-8 bytes. Example: handshake to `localhost:25565` is
`10 00 ff 05 09 "localhost" 63 dd 01` (first byte `0x10` = 16 = the length of everything after it).

The JSON looks like:

```json
{"version":{"name":"1.21.1","protocol":767},
 "players":{"max":20,"online":3},
 "description":{"text":"Hello ","extra":[{"text":"world"}]}}
```

`description` may be a plain string or a **text component** (`text` plus nested `extra` parts). Flatten it to plain text.

## Your turn

`src/lib.rs` already has `write_varint`, `read_varint`, `write_string`, `read_string`. The tests run a fake server on `127.0.0.1` that speaks the protocol. Fill in, in this order:

- `Packet::encode`, `status_request`, `handshake`: building bytes.
- `read_packet`: read one frame. Never trust the length blindly.
- `parse_status` (+ `flatten_text`): JSON with `serde_json`.
- `ping`: connect, send both packets, read the reply, parse it. Set timeouts.

## Afterwards: try it on your server

Add `src/main.rs`:

```rust
fn main() {
    let addr = std::env::args().nth(1).expect("usage: pinger 1.2.3.4:25565");
    println!("{:?}", sys_boss_mc_pinger::ping(addr.parse().unwrap()));
}
```

Run `cargo run -- 192.168.50.3:25565`. Real servers also answer a Ping packet (id `0x01`, 8 bytes) with the same 8 bytes: that's how the launcher shows latency.
