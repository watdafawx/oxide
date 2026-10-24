# GPUI worlds 10 (dashboard) and 11 (screen time): rewrite map

Both worlds were moved to the golden shape (`docs/CONTENT.md`). Audit: `docs/audit-2026-10-24/gpui-06-11.md`.
Ledger rows: the scratchpad `ledger/gpui-10-11.md`, which the parent merges into `docs/tracks/gpui-terms.md`.

Across every exercise:

- The lesson's one code block shows a similar example, never the solution line. For example, a 2-column grid of labels in 01, rows that fit a list in 03, the Terminal arm in 06 (the learner writes the other three), a `stat_count` handler in 09, `as_deref` on a name in 10, and launches counted per program in 11-02.
- The TODO comments say the goal in words. Only the last hint has the exact code.
- Two points per exercise, with varied `answer` positions, and the right choice is not the longest.

## 10-dashboard: 12 quests became 14

| old | new |
|---|---|
| 01-dashboard-window | 01 (cards: the builder chain returns the element, `as u16`, the spacing scale: `gap_4` = 16 px, `px(..)` for exact sizes) |
| 02-a-card | 02 (`and_then` vs `map` explained, why `--` and not `0`) |
| 03-fit-the-window | 03 (`Pixels` is a length, `as usize` drops the fraction) |
| 04-scale-the-text | 04 (where the 32 px in the card width comes from: `p_4` on both sides) |
| 05-a-gauge | 05 (radians defined: a full turn is 2π, half a turn is π) |
| 06-themes | 06 (every colour the test needs is in the lesson) |
| 07-layout-editor | 07 |
| 08-snapshot-json | 08 (beginner card on what JSON is, plus a closure with a block body) |
| 09-web-server (8 min: runtime, Arc<Mutex>, poisoning, into_response all unexplained) | 09 (request/response/HTTP/route/handler cards; refreshers that point at the Rust track's Apps world for axum and Concurrency world for `Arc<Mutex>` and poisoning; why `Response` + `.into_response()`. The runtime card moved to 12. 7 min, 45 XP) |
| 10-secret-key | 10 (query string and `as_deref` explained) |
| 11-local-only | 11 (IP address and port explained) |
| — | **12-its-own-runtime** (new bridge, read): GPUI vs tokio executors, `thread::spawn` refresher (Rust 07), `Runtime::new`, `block_on`, `?` inside a closure that returns `io::Result` |
| — | **13-serve-on-a-thread** (new bridge, exercise): the learner writes `serve_dashboard` (`TcpListener::bind`, `axum::serve`) |
| 12-boss-dashboard | **14-boss-dashboard** (slug kept). `serve_dashboard` now ships pre-filled in the starter, because quest 13 taught it, so its objective passes early. The lesson explains the borrow order, the 32 px and `unwrap_or_default`. 45 min, 400 XP |

## 11-screen-time: 11 quests became 12

| old | new |
|---|---|
| 01..04 | same ids (01: `rsplit`/`next`/`unwrap_or` explained; 02: dereference `*` through `or_insert`'s `&mut`; 03: 5 min / 30 XP; 04: `\|\|` short circuit) |
| 05-day-files | 05. The solution now uses plain `?` on `serde_json::to_string_pretty`, as 07-14 taught; `map_err(std::io::Error::other)` is gone from every copy of `screen.rs` in 06-12. A card says why `?` works (`From`) |
| 06-keep-400-days | 06 (`read_dir` entries, `OsStr` to `&str`, `parse_from_str(..).ok()`, why the name and not the modified time; the last hint now has the date line too. 7 min / 45 XP) |
| 07-day-summary | 07 (one `*` per reference: `**secs` and `\|&hour\|` explained; ties go to the last) |
| 08-hour-chart | 08 (`NaN` from 0/0, cast before dividing) |
| 09-hot-for-30-seconds | 09 (`*` copies the `Instant` out, `&&` stops at false, so the insert goes last) |
| 10-toast | 10 (the round-1 titles card is kept; the binding arm `other => ..` is explained with a sibling match) |
| — | **11-dashes-for-nothing** (new bridge, read): `unwrap_or_else`, a closure in a `let`, a named function passed to `map` (`.map(clock)`), `{:02}` |
| 11-boss-screen-time | **12-boss-screen-time** (slug kept). The lesson now covers `let _ =` for errors on purpose, cloning out of `Entity::read`, `.text_xl()`, `.w_full()`, `.flex_none()` and `enumerate` |

## Id changes and references

- `gpui/10-dashboard/12-boss-dashboard` → `14-boss-dashboard`. Updated `requires` in `11-screen-time/01-whats-in-front/quest.toml`.
- `gpui/11-screen-time/11-boss-screen-time` → `12-boss-screen-time`. Nothing requires it (there is no world 12 folder yet).
- `10-dashboard/09-web-server` and `08-snapshot-json` keep their folders, so 09's `requires` line, which names a rust quest, was not touched. `10-dashboard/01`'s `requires = ["09-overlay/15-boss-game-overlay"]` was not touched either. If world 09's boss is renumbered, the 09 owner updates it.
- **Not updated (outside these worlds):** `docs/tracks/gpui.md` (the world 10/11 tables still list the old quests and the boss ids), and `docs/tracks/gpui-terms.md` (the parent merges the ledger file).

## Notes

- Lessons mention the Rust track's worlds by name (Concurrency, Async, Apps) rather than linking to quest routes, because those worlds are being renumbered right now.
- `10-dashboard/13-serve-on-a-thread` was built from the boss starter. It has its own package name. `dashboard.rs` with `render` comes from 11's solution, and the `Render` impl stub was removed from `lib.rs`. Its other tests (render, key) already pass in the starter; only `server_answers_on_localhost` is its objective.
