# Rust world 12: Broadcast tools (Tauri)

Planning doc for a new world at the end of the Rust track, built from five of the owner's ESL
broadcast projects. Not content. Format and rules: `docs/CONTENT.md`. Research date: 2026-10-23.

## 0. What the ESL projects are, and which are worth moving to Rust

| project | today | what it does | move to Rust? |
|---|---|---|---|
| `Rocket League/rl-hud` | Electron + Node (Express, socket.io, ws) + React, ~8 k lines of server/electron JS | Live RL broadcast HUD. Reads the official Stats API (TCP :49123, back-to-back JSON) or BakkesMod SOS (WS :49122), normalizes both to one event shape, tracks moments (demos, 30 s left, OT, series), presses Companion buttons, switches OBS cameras, follows a lead "director" PC, pushes HUD state to overlay pages, self-updates app / UI / logos / players on separate channels, and has a replay-and-diff check for server changes. | **Yes, first.** The server is the part that must never hiccup on air, and it is all I/O plus small state machines: a good fit for tokio. Tauri keeps the React HUD as is, cuts the installer from ~100 MB to ~10 MB, and `tauri-plugin-updater` replaces the hand-written updater. The replay check becomes a Rust golden test. |
| `img-pro-ui` (Apollo) | Electron + Express + Jimp + React | Batch image tool: crop, resize, trim transparency, square canvas, expand, watermark, rename from a CSV, per-game presets, watch a folder. | **Yes.** Jimp is pure JS and slow on big batches; `image` + `rayon` does the same work many times faster on every core. Same Electron-to-Tauri win. Last touched 2025, so lower priority than the HUD. |
| `img checker` (ESL asset checker) | Wails (Go + React), 23 MB exe, self-updater | Compares the player names in a Google Sheet with the files in a folder: normalizes names, spots likely renames (Levenshtein), watches the folder, exports a report. | **Only as part of Apollo.** It is already a small native exe; a Rust rewrite alone gains little. Folded into a Tauri Apollo it becomes one asset tool: check the names, then process the images. |
| `RIOT GAMES` (LoL, TFT, Valorant stage integration) | three Node/Express + React servers | Poll the Barracks API (200 ms to 2 s), dedup by timestamp, a 2 s startup grace period, look up `triggers.json`, send OSC to Notch/Resolume and HTTP to Companion, lighting and the dashboard. | **Maybe, as one shared crate.** The three servers repeat the same pipeline. A Rust trigger engine (poll, dedup, grace, OSC/HTTP out) shared by all three games and the RL HUD is worth it if EWC 26 needs them again; rewriting each one separately is not. |
| `fullstack-sheet-parser` | Node/Express + Prisma (SQLite) + Clerk + React | Turns Google Sheets into JSON APIs, with column mappings in `sheet-types/*.json`, game tags, expiry cleanup. | **No.** A web service with auth and an ORM; Node is fine there and nothing is slow. Its sheet-type idea (map columns by header name into nested JSON) is good quest material. |

**Spine:** the world builds `hud-server`, a Rust backend for a Rocket League broadcast HUD
inside a Tauri app, plus the asset tools the HUD needs (roster, name matching, player cuts).
Names follow rl-hud: `Frame`, `GameEvent`, `Moment`, `Trigger`, `Output`, `HudState`,
`Roster`, `Replay`. Everything is graded on made-up fixtures: no ESL data, no real players.

| ESL idea | quest | Rust idea it teaches |
|---|---|---|
| Stats API sends JSON objects back to back on TCP | 01, 03 | `serde_json::Deserializer::into_iter`, a read buffer, tokio `TcpStream`, reconnect |
| Stats API and SOS events into one shape | 02 | `#[serde(tag = ...)]` enums, `rename_all`, a catch-all variant |
| duplicate `GoalScored`, Riot timestamp dedup, 2 s startup grace | 04, 05 | `Instant` windows, a small state struct |
| moments: 30 s left, OT, hat-trick, demo counts | 06 | a state machine over events, fire-once flags |
| `triggers.json`: event to OSC / Companion / HTTP | 07, 08 | serde enums as config, OSC packets by hand over UDP |
| overlays get HUD state live | 09 | axum WebSocket, `tokio::sync::broadcast` |
| roster CSV and sheet-type column mapping | 10 | `csv` + headers mapped by name |
| asset checker: sheet names vs files, renames | 11 | normalizing strings, Levenshtein by hand |
| Apollo player cuts | 12 | `image`: crop, trim transparency, square canvas; `rayon` |
| Electron to Tauri | 13, 14 | Tauri v2 commands, managed state, events, mock runtime tests |
| rl-hud replay check | 15 | record frames, replay, diff outputs: golden tests |
| update channels (app / UI / logos, force) | 16 | read: semver, channels, `tauri-plugin-updater` |
| all of it | 17 | boss |

## 1. Quests (`12-broadcast`), project "HUD server", 17 quests

`requires` for 01: `rust/10-apps/11-boss-oxide-in-rust` (axum, serde, a first Tauri command) and
it uses tokio from 08 Async and bytes from 09 Systems.

| slug | new | objectives | min | XP | kind | runs |
|---|---|---|---|---|---|---|
| 01-frames-on-the-wire | split a buffer of back-to-back JSON objects with `Deserializer::from_slice(..).into_iter()`; keep the unfinished tail | `three_frames_from_one_read`, `half_frame_waits` | 4 | 25 | exercise ⚡ | unit |
| 02-one-event-shape | `GameEvent` enum, `#[serde(tag = "Event", content = "Data")]`, PascalCase in, `Other` for unknown events; SOS names mapped to the same variants | `goal_scored_parses`, `sos_and_stats_api_agree`, `unknown_event_kept` | 7 | 45 | exercise | unit |
| 03-connect-and-retry | tokio `TcpStream` to the Stats API port, frames into an `mpsc`, reconnect with backoff 1, 2, 4 s capped at 10; a local listener replays the fixture | `frames_arrive_in_order`, `reconnects_after_drop` | 8 | 60 | exercise | unit |
| 04-one-goal-one-trigger | ignore a second `GoalScored` with the same scorer and time within 2 s | `double_goal_fires_once`, `two_real_goals_both_fire` | 5 | 30 | exercise ⚡ | unit |
| 05-grace-period | no triggers in the first 2 s after a (re)connect: the game resends state on connect | `boot_burst_ignored` | 4 | 25 | exercise ⚡ | unit |
| 06-moments | `MomentTracker::step`: 30 seconds left (once, re-armed above 30), overtime, hat-trick, demo count per team | `thirty_seconds_fires_once`, `hat_trick_on_third_goal`, `demos_counted_per_team` | 8 | 60 | exercise | unit |
| 07-trigger-config | `triggers.json` into `Vec<Trigger>`: event name to `Output::Osc { address, args }`, `Output::Companion { page, button }`, `Output::Http { url }`; unknown output kinds are an error with the line | `goal_maps_to_companion_press`, `bad_output_kind_says_where` | 6 | 40 | exercise | unit |
| 08-osc-by-hand | an OSC message in bytes: address and type tags padded to 4, `i` / `f` / `s` big-endian; sent with `UdpSocket` | `osc_bytes_match_spec`, `udp_packet_arrives` | 7 | 45 | exercise | unit |
| 09-overlay-socket | axum WebSocket route; every `HudState` change goes out through `broadcast::Sender`; a slow overlay that lags is skipped, not waited for | `overlay_gets_goal`, `lagging_overlay_does_not_block` | 8 | 60 | exercise | unit |
| 10-roster-from-a-sheet | roster CSV from a sheet export: columns found by header name through a mapping table (the sheet-type idea), blank rows skipped | `columns_mapped_by_header`, `missing_column_named_in_error` | 6 | 40 | exercise | unit |
| 11-names-vs-files | normalize names (case, accents, spaces, `_`), Levenshtein written by hand, "likely rename" under distance 2 | `missing_and_extra_files`, `renamed_file_suggested` | 7 | 45 | exercise | unit |
| 12-player-cuts | `image`: trim transparent edges, centre on a square canvas, resize to 500; `par_iter` over a folder | `trim_then_square`, `batch_writes_every_file` | 7 | 45 | exercise | unit |
| 13-tauri-commands | Tauri v2: `#[tauri::command] fn hud_state`, `State<Mutex<HudState>>`, `invoke_handler`; tested with `tauri::test::mock_builder` and `get_ipc_response` | `command_returns_score` | 5 | 30 | exercise ⚡ | unit |
| 14-tauri-events | `app.emit("goal", payload)` from the event loop; the React page listens; tested with a listener on the mock app | `goal_event_emitted_once` | 6 | 40 | exercise | unit |
| 15-replay-check | record frames with timestamps to JSON lines, replay them through the pipeline on a fake clock, diff every output against a saved golden file | `replay_is_identical`, `changed_trigger_shows_diff` | 7 | 45 | exercise | unit |
| 16-update-channels | semver, separate channels for app / UI / assets, "force" vs "offer", what `tauri-plugin-updater` checks (signature, feed JSON) | quiz: 3 | 4 | 15 | read | - |
| 17-boss-hud-server | `HudServer::run` from scratch: feed (TCP or replay) to events, dedup, grace, moments, triggers out, overlay socket, Tauri command; replays the fixture match identically | `parses_feed`, `goal_dedup`, `moments_fire`, `triggers_sent`, `overlay_updates`, `tauri_command_live`, `replay_identical` | 75 | 500 | boss | unit |

Totals: 17 quests, 1,150 XP, about 2 h 55 min (174 min).

## 2. Fixtures (made up, generated so the numbers are exact)

- `fixtures/match.jsonl`: one 5-minute 3v3 match as Stats API frames with timestamps. Teams
  "Orange Owls" and "Blue Herons", six invented players. Contains one doubled `GoalScored`, a
  hat-trick, four demos, 30 seconds left, and overtime.
- `fixtures/sos-sample.jsonl`: the same three events in SOS shape, for quest 02.
- `fixtures/roster.csv` and `fixtures/files/`: 12 players, two missing files, one renamed file.
- `fixtures/cuts/`: three small PNGs with transparent borders.
- `fixtures/triggers.json`: five triggers, one with a bad output kind (quest 07 only).

## 3. Risks and spikes (before content)

| id | question | why it matters |
|---|---|---|
| B1 | Does `tauri = { version = "2", features = ["test"] }` build and run `mock_builder` tests under plain `cargo test` on Windows, with no frontend build and no WebView2 window? How long is the first build? | Quests 13, 14 and the boss. If not, Tauri moves to a lib crate tested without the runtime, and `look` steps run the real window. |
| B2 | Target size with tauri + axum + image in one quest crate. | Disk: each quest crate is its own target. May need `CARGO_TARGET_DIR` shared per world (as the other tracks do). |
| B3 | Does `serde_json::StreamDeserializer` report the byte offset of an unfinished trailing object cleanly (`byte_offset()` + `is_eof()`)? | Quest 01's whole idea. |

## 4. Later (not in this world)

- A shared "trigger engine" crate for the Riot servers (poll the Barracks API instead of a TCP
  feed; the same dedup, grace and outputs). Could be a world of its own if EWC 26 needs it.
- Apollo as a full Tauri app (presets, watermark, folder watch), and the asset checker's Google
  Sheets API (service-account JWT) inside it.
- OBS WebSocket camera switching and the "director" lead/follow PCs from rl-hud.
