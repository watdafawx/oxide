Kinetik v3.1.2 fixed a nasty bug class: a settings file with a bad value, or one that could not be read at all, either crashed the app or was silently overwritten. Its fix: correct out-of-range values, keep the bad file aside, and always start.

**Out of range.** A file edited by hand, or written by an older version, can say `"interval_ms": 99`. `clamp_settings` pulls every number back into its range: 0.5 to 5 s, 8 to 24 px, and so on.

**Not readable.** A file of garbage gives an `Err` from `serde_json::from_str`. Do not crash, and do not overwrite it: rename it to `settings.bak` so someone can still look, then start with the defaults:

```rust
Err(_) => { let _ = fs::rename(path, path.with_extension("bak")); Settings::default() }
```

**Missing is fine.** No file at all means a first start: the defaults, quietly. The `let .. else` at the top already handles that.

**Missing fields too.** A file from an older version lacks newer fields. `#[serde(default)]` fills them, so those files still read.

## Your turn

- In `load`, `match` on `serde_json::from_str::<Settings>(&text)`.
- `Ok` goes through `clamp_settings`; `Err` renames the file to `.bak` and gives `Settings::default()`.
- Save.
