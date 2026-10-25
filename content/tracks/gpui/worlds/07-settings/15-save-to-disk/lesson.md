Settings that vanish on restart are not settings. Kinetik keeps them in the registry; this bar keeps them as a JSON file in `%APPDATA%\KinetikStats\settings.json`, which you can open and read.

**To text.** `serde_json::to_string_pretty(settings)` turns the struct into indented JSON, thanks to the derives from quest 2. It gives a `Result`; `?` hands any error to the caller, and `serde_json`'s error converts into `io::Error`, the error type `save` returns. Reading a file back works the same way:

```rust
let text = fs::read_to_string(path)?;
```

**Never half a file.** Writing straight over `settings.json` and crashing halfway leaves a broken file, and every setting is lost. Write to `settings.tmp` first (`path.with_extension("tmp")`), then rename it over the real one. A rename is all or nothing.

**Rename last.** `fs::rename(&temp, path)` replaces the old file in one step. It returns an `io::Result<()>` too, so it can be the function's last line, its result, with no `?`.

**Folder first.** On a fresh computer, `%APPDATA%\KinetikStats` does not exist and every write fails. `fs::create_dir_all(folder)` makes it, and does nothing when it is already there. `path.parent()` gives the folder, as an `Option`.

**Where it lives.** `settings_path()` finds `%APPDATA%`. `main.rs` loads from there at start; saving on every change comes in the boss.

## Your turn

- In `save`, create the file's folder with `fs::create_dir_all`, then turn `settings` into pretty JSON, both with `?`.
- Write it to `path.with_extension("tmp")` with `?`, then rename that onto `path`.
- Save.
