# Oxide

A gamified Rust course, beginner to master, as a desktop app. You edit real cargo projects in your
own editor; Oxide watches the folder, runs the tests on every save, ticks objectives, awards XP,
keeps your streak. Twelve worlds, each ending in a boss project. The last boss is porting Oxide's
own UI to Rust.

## Layout

| path | what |
|---|---|
| `crates/oxide-core` | engine: catalog, grader, workspace, watcher, progress (no Tauri) |
| `crates/content-check` | validates every quest (schema, starter builds, solution passes) |
| `src-tauri` | Tauri 2 commands and events over the engine |
| `src` | SvelteKit UI (static adapter, SPA) |
| `content/worlds` | the quests |
| `docs/CONTENT.md` | how to write a quest |
| `docs/API.md` | UI ↔ core contract |

## Run

```
pnpm install
pnpm tauri dev
```

Quests are copied to `~/oxide-quests/` (override with `OXIDE_QUESTS`). Progress lives in the
app data dir (`%APPDATA%/com.micha.oxide` on Windows).

## Check content

```
cargo run -p content-check -- --fast     # catalog only
cargo run -p content-check               # builds every starter and solution
```
