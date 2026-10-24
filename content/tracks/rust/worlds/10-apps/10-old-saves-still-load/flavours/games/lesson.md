The first mod manager saved only how many mods a pack had. Later versions added the pack's name, your favourites, the day you last played and launcher settings. A player who updates today still has the old file, and the new manager must read it instead of forgetting their packs.

**Read it and see.** `from_json(r#"{"mod_count": 120}"#)` with today's `ModpackProfile` fails, because `Deserialize` insists that every field is in the text:

```text
Error("missing field `name`", line: 1, column: 18)
```

**Fill the gaps.** Put `#[serde(default)]` on the line above the struct, and any field missing from the text is taken from the struct's `Default` instead: an empty `String` or `Vec`, `None`, `0`, or whatever your own `impl Default` says. Fields that are in the text still win. The struct needs a `Default`, which you met in world 05: `#[derive(Default)]` or a hand-written one.

**An attribute for serde.** `#[serde(..)]` is an **attribute**, an instruction attached to the item below it, and this one is read only by serde's derive. It changes the code the derive writes, not your struct. Other `serde` attributes rename fields or skip them; the next quest uses one.

```rust
#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
struct Server {
    port: u16,
    whitelist: bool,
}
// {"port": 25565} reads as Server { port: 25565, whitelist: false }
```

**Inside, too.** The attribute works per struct. A nested `LauncherSettings` with only `{"auto_update": false}` in the file needs its own `#[serde(default)]` to fill `loader` and `keep_versions` from `LauncherSettings::default()`; the outer struct's attribute only fills in a `settings` that is missing entirely.

**Why it matters.** Every app that saves files meets this the first time it adds a field. Adding a field with a default is safe forever; renaming or removing one breaks old saves, so think twice before you do.

## Your turn

- Give `ModpackProfile` a derived `Default` and the `#[serde(default)]` attribute.
- Give `LauncherSettings` the attribute too (its `Default` is written for you).
