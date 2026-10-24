The engine, the API and the screens all run on your machine with `cargo run`. A friend can't use that: they need one file to double-click, it has to be quick, and their progress must survive the next update. Four habits get you there.

**Build for release.** `cargo run` and `cargo test` use the **dev profile**: quick to compile, slow to run, full of extra checks. `cargo build --release` uses the **release profile**, which optimises the code and often runs 10 to 50 times faster. It writes the program to `target/release/`.

**Trade build time for size.** A few lines in `Cargo.toml` make the release build smaller and faster still, at the cost of a slower build. `lto` (link-time optimisation) optimises across all your crates at once instead of one at a time.

```toml
[profile.release]
lto = true           # optimise across crates: smaller and faster
codegen-units = 1    # compile as one unit: slower build, better code
strip = true         # drop debug symbols: a much smaller file
```

**Installers.** An egui app is one executable you can zip. A Tauri app is built with `cargo tauri build`, which also makes installers under `target/release/bundle/`: an `msi` or NSIS setup on Windows, a `dmg` on macOS, `deb` and `AppImage` on Linux.

**Never next to the exe.** Installed programs usually live in read-only folders, and the working directory is wherever the user happened to start the app. Ask the OS for the user's data folder instead: `dirs::data_dir()` from the `dirs` crate gives `%APPDATA%` on Windows, `~/Library/Application Support` on macOS, `~/.local/share` on Linux. Create your folder in it with `std::fs::create_dir_all`, and open the SQLite file there. Tauri offers the same folder as `app.path().app_data_dir()`.

**Ship a version.** Bump `version` in `Cargo.toml`, tag the commit, and attach the build to a release. Keep the migrations from the start of this world in every version: next month's app must open this month's file.

## Your turn

Answer the two questions.
