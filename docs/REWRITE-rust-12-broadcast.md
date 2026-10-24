# Rust world 12: rewrite map

World 12 (the HUD server) kept its project arc and its strong lessons (04, 13 tokio-or-rayon, 17 replay
check, the boss). The rewrite fixes what the audit (`docs/audit-2026-10-24/rust-11-12.md`) found:

- **The answer, but not the shape.** Every exercise's code block now shows the *surrounding* control flow
  (the loop, the match, the running min/max, the two rows) on other data. The solution's core line is only in
  the last hint. Examples: a `loop`/`match` that sums a stream of scores (02), a byte-counting read loop (04),
  a let-chain on a roster (05), `is_some_and` on the fastest goal (06), Pascal's triangle for the two-row loop
  (14), earliest/latest goal time for min/max (15), `spawn_blocking` reading a file (18), `emit("score", ..)` (20),
  a tuple-of-Options match lining up two teams (21).
- **Broadcast vocabulary.** New read quest `01-the-booth` explains Stats API, SOS, HUD, scorebug, stinger,
  OBS browser source, cue, OSC, Notch, Resolume, Companion, Stream Deck and producer, with a picture of the path.
- **Dynamic programming.** New read quest `13-edit-distance` works the `fen`/`fan` table by hand: what a cell
  means, the three neighbours, why the smallest neighbour is right, why two rows are enough.
- **Used before taught.** `iter::once` is gone (tags are built with `String::from(",")` + `push`, taught in 10;
  the carried-forward `osc.rs` files changed the same way). `enumerate_pixels`, alpha / `pixel[3]` and the
  `u32::MAX` start are taught in the new 15. `Message::Text(text.into())` and the `loop { match recv().await }`
  shape are taught in 11. `usize::from(bool)`, `(0..=n).collect()`, `.min` chains in 14. `Option::map_or` is
  gone from 03: a given helper `stats_api_name` replaces it (also in every later `event.rs`). `Ok(0) | Err(_)`
  is explained in 04, `unwrap_or_else(|p| p.into_inner())` in 19, `csv::Reader::from_reader` / `records` in 12.
- **Points.** All re-written; answers spread over positions 0-3 and the right one is not the longest.
  15-tauri-commands' ill-posed point is now "What does Electron ship that Tauri does not?" → Chromium.
- **Import wall.** Each `lib.rs` keeps only the imports that quest's code or tests use (the first exercise
  had tauri and image imports in a JSON quest).

## 19 quests became 23

| old | new |
|---|---|
| (new) | 01-the-booth (read; takes over `requires = ["10-apps/11-boss-oxide-in-rust"]`) |
| 01-frames-on-the-wire | 02-frames-on-the-wire |
| 02-one-event-shape | 03-one-event-shape (SOS lookup given as `stats_api_name`) |
| 03-connect-and-retry | 04-connect-and-retry |
| 04-one-goal-one-trigger | 05-one-goal-one-trigger |
| 05-grace-period | 06-grace-period |
| 06-moments (3 objectives, 8 min) | 07-thirty-seconds-left (overtime edge + fire-once flag, 1 objective), 08-count-the-moments (hat-trick + demos per team, 2 objectives; the `UpdateState` arm is given) |
| 07-trigger-config | 09-trigger-config |
| 08-osc-by-hand | 10-osc-by-hand (push loop instead of `iter::once`) |
| 09-overlay-socket | 11-overlay-socket |
| 10-roster-from-a-sheet | 12-roster-from-a-sheet |
| (new) | 13-edit-distance (read) |
| 11-names-vs-files | 14-names-vs-files |
| 12-player-cuts (box + crop + rayon) | 15-the-visible-box (new exercise `visible_box`, tests `box_holds_every_visible_pixel`, `empty_photo_has_no_box`), 16-player-cuts (`trim_transparent` = crop to `visible_box`, or keep the image) |
| 13-tokio-or-rayon | 17-tokio-or-rayon |
| 14-off-the-runtime | 18-off-the-runtime |
| 15-tauri-commands | 19-tauri-commands |
| 16-tauri-events | 20-tauri-events |
| 17-replay-check | 21-replay-check |
| 18-update-channels | 22-update-channels |
| 19-boss-hud-server | 23-boss-hud-server (slug kept) |

Package names follow the new folders (`broadcast_02_frames_on_the_wire` ... `broadcast_23_boss_hud_server`;
the boss's `main.rs` uses the new crate name). The carried-forward `cuts.rs` in 18-23 now has `visible_box` +
the short `trim_transparent`.

## Id changes and references

No `requires` anywhere outside world 12 named a world-12 quest (grep of `content/`, `src/`), and gpui does not
reference world 12. Every id changed except the boss slug's folder number (19 → 23). The world's first quest still
requires `10-apps/11-boss-oxide-in-rust`.

## Notes

- Ledger rows: scratchpad `ledger/rust-12-broadcast.md`; they replace the existing 12-broadcast rows of
  `docs/tracks/rust-terms.md`.
- `docs/tracks/rust-12-broadcast.md` (the world plan) still lists the old 19 quests; not edited (outside this world).
- World 12 has no flavours.
- **Tauri mock IPC fix.** With tauri 2.12.5 (what a fresh resolve picks), the tests' `get_ipc_response` with
  `url: "http://tauri.localhost"` is rejected on Linux ("hud_state not allowed. Plugin not found"): that URL is the
  local origin only on Windows. The tests in 19-21 and 23 now use `tauri://localhost` off Windows. The same bug
  is in `10-apps/09-tauri-commands` (both flavours), which this rewrite did not touch.
- Building any world-12 quest on Linux needs `libwebkit2gtk-4.1-dev libgtk-3-dev libsoup-3.0-dev` (installed with apt in this sandbox).
