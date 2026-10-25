# Rust world 12: Broadcast tools (Tauri)

Plan and record of the world at the end of the Rust track, built from five of the owner's ESL
broadcast projects. Not content. Format and rules: `docs/CONTENT.md`. Research date: 2026-10-23.

> Rewritten in October 2026 (19 → 23 quests); the current list is in `docs/REWRITE-rust-12-broadcast.md`.

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
| Stats API and SOS events into one shape | 02 | an envelope, then `match` on the name; `rename_all`, `alias`, `Other(name)` |
| duplicate `GoalScored`, Riot timestamp dedup, 2 s startup grace | 04, 05 | time windows on the feed's clock, let-chains, `is_some_and` |
| moments: 30 s left, OT, hat-trick, demo counts | 06 | a state machine over events, fire-once flags |
| `triggers.json`: cue to OSC / Companion / HTTP | 07, 08 | serde tagged and untagged enums as config, OSC packets by hand over UDP |
| overlays get HUD state live | 09 | axum WebSocket, `tokio::sync::broadcast`, `Lagged` |
| roster CSV and sheet-type column mapping | 10 | `csv` + headers found by name |
| asset checker: sheet names vs files, renames | 11 | normalizing strings, Levenshtein by hand |
| Apollo player cuts | 12 | `image`: trim transparency, square canvas, resize; `rayon` |
| tokio, rayon, threads: which and why | 13, 14 | read; `spawn_blocking` to keep the feed flowing |
| Electron to Tauri | 15, 16 | Tauri v2 commands, managed state, events, mock runtime tests |
| rl-hud replay check | 17 | replay on the recording's clock, diff against a golden file |
| update channels (app / UI / logos, force) | 18 | read: semver, channels, `tauri-plugin-updater` signatures |
| all of it | 19 | boss |

## 1. Quests (`12-broadcast`), project "HUD server", 19 quests

`requires` for 01: `10-apps/28-boss-oxide-in-rust` (axum, serde, a first Tauri command); it uses
tokio from 08 Async and bytes from 09 Systems. Every quest crate has the same dependencies, so the
app's shared `.target` compiles them once.

| slug | new | objectives | min | XP | kind | runs |
|---|---|---|---|---|---|---|
| 01-frames-on-the-wire | `split_frames`: `Deserializer::from_slice(..).into_iter()`, `byte_offset`, `is_eof`; `FrameBuffer` keeps the unfinished tail | `three_frames_from_one_read`, `half_frame_waits` | 4 | 25 | exercise ⚡ | unit |
| 02-one-event-shape | `normalize`: `Envelope` (with SOS `alias`es), SOS names mapped, `match` on the name, `Other(name)` | `goal_scored_parses`, `sos_and_stats_api_agree`, `unknown_event_kept` | 7 | 45 | exercise | unit |
| 03-connect-and-retry | `read_frames`: tokio `TcpStream` reads into a `FrameBuffer`, events into an `mpsc`; `run_feed` reconnects with backoff (*starter*) | `frames_arrive_in_order`, `reconnects_after_drop` | 8 | 60 | exercise | unit |
| 04-one-goal-one-trigger | `GoalDedup::is_repeat`: same scorer and game time within 2 s | `double_goal_fires_once`, `two_real_goals_both_fire` | 5 | 30 | exercise ⚡ | unit |
| 05-grace-period | `Grace::allows`: nothing before a connection or in its first 2 s | `boot_burst_ignored` | 4 | 25 | exercise ⚡ | unit |
| 06-moments | `MomentTracker::step`: 30 seconds left once, overtime, hat-trick, demos per team | `thirty_seconds_fires_once`, `hat_trick_on_third_goal`, `demos_counted_per_team` | 8 | 60 | exercise | unit |
| 07-trigger-config | `load_triggers`; `Output` (`tag = "kind"`) and `OscArg` (`untagged`) given; errors name their line | `goal_maps_to_companion_press`, `bad_output_kind_says_where` | 5 | 30 | exercise | unit |
| 08-osc-by-hand | `encode`: address, type tags, big-endian args, 4-byte padding; `send_osc` over `UdpSocket` | `osc_bytes_match_spec`, `udp_packet_arrives` | 7 | 45 | exercise | unit |
| 09-overlay-socket | `forward`: `broadcast::Receiver` to an axum WebSocket; `Lagged` skips, `Closed` ends | `overlay_gets_goal`, `lagging_overlay_does_not_block` | 8 | 60 | exercise | unit |
| 10-roster-from-a-sheet | `read_roster`: `csv` headers found by name, cells trimmed, blank rows skipped | `columns_mapped_by_header`, `missing_column_named_in_error` | 6 | 40 | exercise | unit |
| 11-names-vs-files | `levenshtein` with one row; `normalize_name` and `compare_names` given | `missing_and_extra_files`, `renamed_file_suggested` | 7 | 45 | exercise | unit |
| 12-player-cuts | `trim_transparent` with `crop_imm`; `square`, `cut` and a `par_iter` `cut_folder` given | `trim_then_square`, `batch_writes_every_file` | 7 | 45 | exercise | unit |
| 13-tokio-or-rayon | tokio for waiting, rayon for computing, threads for one long job; crossbeam, smol; `spawn_blocking` to mix | quiz: 3 | 4 | 15 | read | - |
| 14-off-the-runtime | `off_the_runtime`: `spawn_blocking(..).await`, `Send + 'static` | `feed_keeps_flowing_while_cutting`, `cut_folder_from_async` | 4 | 25 | exercise ⚡ | unit |
| 15-tauri-commands | `#[tauri::command] hud_state` with `State<Mutex<HudState>>`, poisoned locks; mock runtime | `command_returns_score` | 5 | 30 | exercise | unit |
| 16-tauri-events | `publish_moment`: `app.emit("moment", payload)`; `listen_any` in the test | `moment_event_emitted_once` | 6 | 40 | exercise | unit |
| 17-replay-check | `diff_lines` against `fixtures/match.golden`; `replay` and `Pipeline` given | `replay_is_identical`, `changed_trigger_shows_diff` | 7 | 45 | exercise | unit |
| 18-update-channels | semver plus a build number, offer vs force, `tauri-plugin-updater` feed and signature, assets as data | quiz: 3 | 4 | 15 | read | - |
| 19-boss-hud-server | `Pipeline::step` from scratch; `main.rs` replays the match, or `--live` reads the Stats API and serves `ws://127.0.0.1:8787/ws` | `parses_feed`, `goal_dedup`, `moments_fire`, `triggers_sent`, `overlay_updates`, `tauri_command_live`, `replay_identical` | 75 | 500 | boss | unit |

Totals: 19 quests, 1,180 XP, about 3 h (181 min).

## 2. Fixtures (made up, generated so the numbers are exact)

- `fixtures/match.jsonl`: a 5-minute 3v3 match plus overtime, 338 frames with times. "Blue Herons"
  (Wren, Fen, Ash) against "Orange Owls" (Pike, Moss, Teal). A stale goal in the boot burst, one
  goal sent twice, Wren's hat-trick, four demolitions, 30 seconds left, overtime and an overtime
  winner (3-4). Simplified Stats API shapes: `Data` is an object.
- `fixtures/match.golden`: the 15 action lines a correct server prints for it.
- `fixtures/triggers.json` (five triggers) and `fixtures/triggers-bad.json` (an unknown kind on line 3).
- `fixtures/roster.csv`: six players, extra columns in another order, a blank row.
- Photos are drawn in the tests, so there are no binary fixtures.

## 3. Spikes (run 2026-10-23, all pass)

| id | question | result |
|---|---|---|
| B1 | Tauri commands and events under plain `cargo test`, no window? | Yes: `mock_builder` + `get_ipc_response` for commands, `listen_any` + `emit` for events. Cold build of the world's dependency set: 43 s with crates cached. |
| B2 | Build size | 1.3 GB per target. The app's shared `.target` holds it once for the whole world. |
| B3 | `StreamDeserializer` and an unfinished last value | `byte_offset()` before `next()` plus `is_eof()` on the error gives the exact split point. |
| B4 | `#[serde(other)]` in a `tag` + `content` enum when data is present | Fails ("expected unit"). Quest 02 reads an envelope and matches the name by hand instead, which keeps the unknown name. |

## 4. Later (not in this world)

- A shared "trigger engine" crate for the Riot servers (poll the Barracks API instead of a TCP
  feed; the same dedup, grace and outputs). Could be a world of its own if EWC 26 needs it.
- Apollo as a full Tauri app (presets, watermark, folder watch), and the asset checker's Google
  Sheets API (service-account JWT) inside it.
- OBS WebSocket camera switching and the "director" lead/follow PCs from rl-hud.
