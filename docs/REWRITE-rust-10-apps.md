# Rust world 10: rewrite map

World 10 was rewritten in the golden shape (`docs/CONTENT.md`, `content/golden/`): one idea per quest,
an unlabeled intro, kicker cards, one code block showing a similar example (not the solution), a short
Your turn, two hints with the exact code only in the last, and two points per exercise. Audit:
`docs/audit-2026-10-24/rust-08-10.md` (rated 4.5). Ledger rows: the scratchpad
`ledger/rust-10-apps.md`, which the parent merges.

## One spine per flavour

- **Plain: Oxide's progress app.** A `quests` table (id, title, xp, later done), the `Profile` JSON,
  a progress API (`/health`, `/level/{xp}`, `/xp-total`, `/xp`, `/quests`), the quest-card text, the
  daily-goal picker, the XP bar, the quest list, the Level up! pop-up, the `award_xp` / `start_quest`
  commands, and the boss: Oxide's home and quest screens. The old notes table, notes API,
  hello/sum/bump API and generic counter are gone.
- **Games: a mod manager.** A `mods` table (id, name, enabled, later favourite), the
  `ModpackProfile` JSON, a mod server API (`/status`, `/tier/{mb}`, `/total-size`, `/downloads`,
  `/mods`), mod-card text, a render-distance settings picker, the download bar, the mod list, the
  Tier up! pop-up, the `add_download` / `install_mod` commands, and a new games boss: the mod
  manager's home and pack screens. Every games quest has the same objectives in the same order as plain.

## 12 quests became 28

| old | new |
|---|---|
| (none: SQL never taught) | **01-what-a-database-is** (read: table, row, column, key, the five SQL statements, WHERE / ORDER BY, SQLite) |
| 01-rusqlite-basics (connection, SQL, params, injection, query_map, collect at once) | 02-your-first-table (connection, `execute`, CREATE TABLE), 03-never-glue-values-in (meets the apostrophe bug and SQL injection with `format!`, then `?1` + `params!`), 04-read-one-row (`query_row`, `row.get`, `COUNT(*)`), 05-read-every-row (`prepare`, `query_map`, collect) |
| 02-transactions-and-migrations (4 objectives, 10 min) | 06-all-or-nothing (transaction, rollback on drop), 07-upgrade-old-files (`user_version` migrations, ALTER TABLE). Uses `tx.execute` in the loop; the prepared statement inside a block is a for-the-curious line |
| (none: JSON never taught) | **08-what-json-is** (read: objects, arrays, the six value kinds, serialize / deserialize) |
| 03-serde-models (4 objectives, 4 attributes) | 09-structs-to-json (derives, `to_string`, `from_str`), 10-old-saves-still-load (`#[serde(default)]`, nested structs), 11-spell-it-their-way (`rename_all`, `rename = "type"`) |
| (none: HTTP never taught) | **12-what-a-request-is** (read: client/server, method, path, body, status codes, route, port) |
| 05-axum-hello ("tower Service", `&'static str` unexplained) | 13-hello-axum (handler, Router, free 404/405, `oneshot` explained as "hand the request to the router", `'static` glossed) |
| 06-axum-json-api (hello/sum/bump, 4 objectives, 12 min) | 14-read-the-path (`Path`, `Json` out, free 400), 15-json-in (`Json` body, free 422), 16-say-no-with-a-status (`Result` with `(StatusCode, Json)`), 17-remember-between-requests (`State`, `Arc<AtomicU32>`, 201) |
| 07-axum-sqlite-crud (notes, 4 objectives, 15 min) | 18-share-the-database (`Arc<Mutex<Connection>>`, `internal`, POST + GET list), 19-not-found-is-404 (`.optional()`, `match` into 404, DELETE 204/404) |
| 04-choosing-a-ui-stack (GPUI, `cx.notify()`, "Zed, Hopper", a table) | 20-how-a-screen-is-drawn (read: frames, event loop, widgets, retained vs immediate mode, webview and sandbox). GPUI and Hopper removed |
| 07-mod-card-text (plain was mod text) | 21-card-text (plain: quest lines and "2 of 3 quests done"; games keeps the mod card) |
| 08-egui-viewmodel (generic counter, TODOs were the solution) | 22-a-screen-you-can-test (plain: daily-goal picker; games: render distance. The model methods are given; the learner writes `ui`; "every frame" is explained) |
| (boss "small tools": clamp, ProgressBar) | **23-fill-the-bar** (bridge: `as f32`, NaN/infinity, `clamp`, `ProgressBar`, `ui.add`) |
| (boss "small tools": the borrow-in-match workaround) | **24-click-then-act** (bridge: E0502 when calling `&mut self` inside a loop over `&self.titles`; remember the click, act after) |
| (boss "small tools": Modal, should_close) | **25-a-pop-up** (bridge: `egui::Modal`, local close flag, `should_close`, the context) |
| 09-tauri-commands (learner wrote `xp / 100 + 1` and lines given in TODOs) | 26-tauri-commands (the JS `invoke` side and the sandbox are explained; the `_inner` logic is given; the learner writes both `#[tauri::command]` wrappers and the `manage` + `generate_handler!` wiring; tests go through `mock_builder`) |
| 10-shipping (GPUI, Hopper) | 27-shipping (read; GPUI and Hopper removed) |
| 11-boss-oxide-in-rust (90 min, 12-item "small tools" block, no games flavour) | 28-boss-oxide-in-rust (slug kept, renumbered; 75 min; the glossary is gone: cards point at quests 21-25, and `key_pressed` gets one card. `let-else`, `as_ref`, `map_or`, `is_some_and` and `Id::unique` were replaced by `match` / `Id::new` in the solution, hints and `main.rs`. New games flavour: the mod manager's Home / Pack screens, `Modpack`, `VerifyResult`, `Install`, Tier up!) |

## Cut on purpose

- `execute_batch` (07 uses `tx.execute(sql, [])`; the tests still use `execute_batch` to build an old file).
- `serde_json::Value` indexing for the learner (only the tests use it).
- `Option::map` / `ok_or` in the 404 handler (a `match` instead; named in a for-the-curious card).
- The "Why now:" labels, "small tools" glossaries and "More pieces from the same family" lists.

## Id changes and references

- `rust/10-apps/06-axum-json-api` no longer exists. **gpui** `content/tracks/gpui/worlds/10-dashboard/09-web-server/quest.toml`
  requires it; not edited here. Replace it with `rust/10-apps/17-remember-between-requests`
  (Json in/out, status codes and shared `State`, the last axum-JSON quest). `docs/tracks/gpui.md` line 124 names the old id too.
- `rust/10-apps/11-boss-oxide-in-rust` → `rust/10-apps/28-boss-oxide-in-rust`. Updated the `requires`
  in `content/tracks/rust/worlds/12-broadcast/01-the-booth/quest.toml`. `docs/tracks/rust-12-broadcast.md` line 40 names the old id.
- `01-what-a-database-is` has no `requires`, so it still chains to the last quest of world 09.
  The old `07-mod-card-text` / `08-egui-viewmodel` explicit `requires` lines are gone (the folder order is enough now).

## Notes

- 26-tauri-commands: the mock IPC URL is `tauri://localhost` off Windows (`http://tauri.localhost` on
  Windows), the same fix rust 12 uses; tauri 2.12 rejects the old URL on Linux. The mock-app test helper
  returns the app together with the webview so the app stays alive.
- 23-fill-the-bar finds the bar with `get_by_role(egui::accesskit::Role::ProgressIndicator)`.
- The boss's test quest id is now `rust/01-foundations/02-the-count-changes` (test data only).
