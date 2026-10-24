Kinetik v3.1.2 fixed a nasty bug class: a settings file with a bad value, or one that could not be read at all, either crashed the app or was silently overwritten. Its fix: correct out-of-range values, keep the bad file aside, and always start.

**Out of range.** A file edited by hand, or written by an older version, can say `"interval_ms": 99`. `clamp_settings` pulls every number back into its range: 0.5 to 5 s, 8 to 24 px, and so on.

**Not readable.** A file of garbage gives an `Err` from `serde_json::from_str::<Settings>(..)`; the `::<Settings>` says what to read it as. Do not crash, and do not overwrite it: rename it to `settings.bak` so someone can still look, then start with the defaults. If even the rename fails, there is nothing better to do, so its result is dropped with `let _ =`.

**Both paths answer.** A `match` with an `Ok` arm and an `Err` arm gives a `Settings` either way. The same shape for a number typed by hand:

```rust
match "250".parse::<u64>() { Ok(ms) => ms.clamp(500, 5000), Err(_) => 1000 }
```

**Missing is fine.** No file at all means a first start: the defaults, quietly. The `let .. else` at the top already handles that.

**Missing fields too.** A file from an older version lacks newer fields. `#[serde(default)]` fills them, so those files still read.

## Your turn

- In `load`, `match` on `serde_json::from_str::<Settings>(&text)` in place of the `unwrap` line.
- `Ok` goes through `clamp_settings`; `Err` renames the file to `.bak` and gives `Settings::default()`.
- Save.
