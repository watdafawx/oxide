**Why now:** the mod manager works on your machine. Shipping it means a binary that is small and fast, data that survives an update, and an installer a friend can double-click.

```toml
# Cargo.toml: slower build, smaller and faster program
[profile.release]
lto = true            # optimise across crates
codegen-units = 1     # one big unit: better optimisation
strip = true          # drop debug symbols
opt-level = 3         # "s" or "z" favour size
```

- `cargo build --release` uses the `release` profile. `cargo run` and `cargo test` use `dev`.
- **Where data lives**: never next to the exe (Program Files is read-only) and never the working directory. Use the OS app-data folder: `%APPDATA%` on Windows, `~/Library/Application Support` on macOS, `~/.local/share` on Linux. The `dirs` crate finds it.
- Create that folder on first run (`create_dir_all`) and open the SQLite file inside it. Keep migrations from the last quests so updates upgrade old databases.
- Tauri builds installers with `cargo tauri build` (an MSI or NSIS setup on Windows, a dmg on macOS). egui apps ship as a plain executable you can zip.
- Always test the release build once. Debug-only behaviour (extra checks, slow paths) hides there.
