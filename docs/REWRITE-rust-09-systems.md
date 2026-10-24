# Rust world 09: rewrite map

World 09 was rewritten in the golden shape (`docs/CONTENT.md`, `content/golden/`): one idea per quest, an unlabeled intro, kicker cards, one code block that shows a similar example rather than the solution, a short Your turn, two hints (exact code only in the last) and two points. The "**Why now:**" labels and the "Small tools" glossary in the boss are gone. Every exercise has a plain flavour (a server status checker, textbook names) and a games flavour (the Minecraft server pinger: launcher, server list, MOTD, players). Both flavours have the same objectives in the same order. The boss now has a games flavour too: same code, a launcher-voiced lesson. Audit: `docs/audit-2026-10-24/rust-08-10.md`. Ledger rows: the scratchpad `ledger/rust-09-systems.md`, which the parent merges.

The audit praised the bytes → cursor → VarInt → Server List Ping arc and the boss. Both are kept. What changed:

- **Bits, bytes and hex are taught first.** The old world opened with `0x1234_5678`. Now 01 (read) explains bit, binary, byte, `u8` as one byte, `0b` and `_`. Then 02 explains hex, `0x` and two digits per byte, and has the learner write a hex dump with `{:02x}`. Both link to Programming 101's `07-how-the-computer-holds-things` (bits-and-bytes, hex-colours, numbered-boxes) but don't depend on it.
- **Endianness gets its why.** Chip makers went one way and network standards the other, and a wrong order gives a different number, not a crash (300 becomes 738,263,040).
- **Unsafe/FFI is framed honestly as an aside.** The old "Why now" lines said the pinger uses C and raw pointers, which was false. Quests 09-13 now say they are an aside: the boss has no `unsafe`, and they show what is under `&[u8]` and `TcpStream`. The aside sits between the cursor quests and the network quests, so it reads as "under the floor" and doesn't interrupt the lead-up to the boss. A pointer is defined (with a refresher of world 06 and a Programming 101 link), null, dangling and unaligned are explained, and so is what C is and why the OS speaks it. `static mut` and `unsafe impl` are named in one clause ("you won't need these here"). `union` is gone.
- **TCP and sockets get a beginner card.** 14 (read) covers client and server, IP address, `127.0.0.1`, port (a `u16`, port 0), connection, "a stream, not messages", UDP datagrams and socket.
- **serde/JSON is taught at the point of use.** 23 has a one-card JSON explainer plus `serde_json::from_str`, `Value`, `v["a"]["b"]`, `as_*` and `ok_or`. It notes that world 10 teaches typed serde. 24 covers `Value` as an enum and `match` on its shape. The boss's `parse_status` and `flatten_text` need both.
- **No more TODO-as-solution.** The old VarInt starter spelled out the algorithm. TODOs now give the goal in words, and the VarInt puzzle is split across three quests.
- **Untaught syntax removed or taught.** `const` (05), `loop` (16), `b"..."` (15) and `Duration` (19) are now taught. `let ... else` is gone from the solutions (`match` instead). `extend_from_slice`, `(1..=MAX).contains`, `io::Cursor` and `position()` are taught in boss cards.

## 10 quests became 25

| old | new |
|---|---|
| (new) | **01-bits-and-bytes** (read: bit, binary, byte, `u8`, `0b`, `_`) |
| (new) | **02-hex-dump** (hex, `0x`, `{:02x}`; `hex_dump` / `packet_dump`) |
| 01-bytes-and-endianness (3 objectives, 3 ideas) | 03-which-byte-first (to/from_be/le_bytes on arrays, why two orders), 04-the-first-four-bytes (`get(..4)?`, `try_into().ok()?`, short slice → None) |
| 02-bit-ops (`\|`/`&`/`!`, shifts, popcount, untaught `const`) | 05-flags-in-a-byte (`const`, flags, masks, `count_ones`), 06-shift-and-pack (`<<`/`>>`, widen, `as u8`; plain packs an IPv4 address, games a MOTD colour) |
| 03-binary-parsing (4 objectives, 12 min) | 07-a-cursor (`take`, `u8`, check before moving), 08-read-a-header (`u16_be`/`u32_be`, `?` in a struct literal) |
| 04-the-unsafe-contract (read) | 09-the-unsafe-contract (read, "Aside:" title, honest framing, 3rd quiz question "does the boss need unsafe?") |
| 05-raw-pointers (4 objectives, 12 min) | 10-through-a-raw-pointer (pointer = address, `*mut`/`*const`, null), 11-a-slice-from-parts (`from_raw_parts`, safe wrapper) |
| 06-ffi (4 objectives, TODO = solution) | 12-call-into-c (what C is, `extern "C"`, `c_abs`), 13-c-strings (NUL, `CString`, `CStr`, `Cow`) |
| (new) | **14-ports-and-connections** (read: the networking beginner card) |
| 07-tcp-echo (3 objectives, 14 min) | 15-connect-and-send (client: `connect`, `write_all`, `read_exact`, `b".."`; new test with an answer arriving in pieces), 16-echo-until-closed (`read` → `n`, `Ok(0)`, `loop`; new test bigger than the buffer), 17-one-thread-per-client (starter is the "obvious" one-at-a-time server, which fails; `let-else` replaced by `match`) |
| 08-udp (3 objectives, 2 ideas) | 18-datagrams (`send_to`/`recv_from`), 19-dont-wait-forever (`set_read_timeout`, `Duration`, plus the TCP timeouts the boss needs; the test runs the request on a helper thread so the hanging starter fails after 2 s instead of hanging) |
| 09-varints (4 objectives, 12 min, TODO = algorithm) | 20-seven-bits-at-a-time (encode, why `as u32`), 21-read-a-varint-back (decode, bytes used, `Truncated`), 22-a-broken-varint (the learner meets the `shift left with overflow` panic; `TooLong`) |
| (new) | **23-read-the-json** (bridge: JSON + serde_json `Value`, `as_*`, `ok_or`) |
| (new) | **24-a-list-in-the-json** (bridge: `match` on `Value` variants, `players.sample`) |
| 10-boss-minecraft-server-pinger | 25-boss-minecraft-server-pinger (slug kept, renumbered to stay last; lesson rewritten as kicker cards, glossary folded into cards; hints escalate; "Hopper" test strings replaced; **new games flavour**, same code) |

Bridge quests added before the boss: 23 and 24 (JSON and `Value`). 19 also counts as one, since it teaches the TCP timeouts the boss uses.

## Id changes and references

- `rust/09-systems/10-boss-minecraft-server-pinger` → `25-boss-minecraft-server-pinger`. Nothing in `content/` named any 09 quest in `requires`: no other world, no gpui quest. `10-apps/01` has no `requires`, so it still chains to the last quest of 09 (the boss).
- `01-bits-and-bytes` has no `requires`, so it still chains to the last quest of world 08.
- All other old ids (`01-bytes-and-endianness`, `02-bit-ops`, ..., `09-varints`) no longer exist. A grep of `content/` found no references to them.
- `docs/REVIEW-2026-10-19.md` and the audit name old ids. They are historical, so they were left as they are.

## Notes

- Lessons point to worlds 07 (hex printing, threads) and 08 (JSON from web servers) as they stood during this rewrite. If those worlds change, check the one-line references in 02, 17 and 23.
- 19's starter blocks forever in a detached thread. The test fails after 2 s, and the test binary exits normally.
- The plain flavour names Minecraft only as the protocol's owner (VarInt quests, 23, boss), never as a story.
