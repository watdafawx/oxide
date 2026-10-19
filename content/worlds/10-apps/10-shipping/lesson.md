**Why now:** a mod manager nobody can install is a demo, so before the boss you learn how Oxide gets from `cargo run` to something a friend can double-click.

Four things turn your project into something you can hand to a friend.

**1. Build with `--release`.** Debug builds are 10 to 50 times slower. Tune the release profile once:

```toml
[profile.release]
lto = true           # optimise across crates: smaller and faster
codegen-units = 1    # slower build, better code
strip = true         # drop symbols: a much smaller file
# panic = "abort"    # smaller still, but no unwinding
```

**2. Know where output goes.** `cargo build --release` writes to `target/release/`. Tauri's `cargo tauri build` goes on to make installers under `target/release/bundle/` (`msi` / `nsis` on Windows, `dmg` on macOS, `deb` / `AppImage` on Linux). egui and GPUI apps are a single executable you zip or wrap in an installer yourself.

**3. Never write user data next to the exe.** Installs often live in read-only folders. Ask the OS for the right place:

```rust
let dir = dirs::data_dir().unwrap().join("oxide");   // %APPDATA% on Windows
std::fs::create_dir_all(&dir)?;
let db = rusqlite::Connection::open(dir.join("progress.db"))?;
```

Tauri hands you the same paths through `app.path().app_data_dir()`. Keep your schema migrations from earlier in this world: next month's version must open this month's file.

**4. Ship a version.** Bump `version` in `Cargo.toml` (and `tauri.conf.json`), tag the commit, attach the binary to a release. Hopper does exactly this after every change.

## Your turn

Answer the two questions below.
