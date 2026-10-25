The first Oxide saved only your XP. Later versions added the list of finished quests, the day you were last active and your settings. A player who updates today still has the old file, and the new app must read it instead of losing their progress.

**Read it and see.** `from_json(r#"{"xp": 120}"#)` with today's `Profile` fails, because `Deserialize` insists that every field is in the text:

```text
Error("missing field `completed`", line: 1, column: 11)
```

**Fill the gaps.** Put `#[serde(default)]` on the line above the struct, and any field missing from the text is taken from the struct's `Default` instead: an empty `Vec`, `None`, `0`, or whatever your own `impl Default` says. Fields that are in the text still win. The struct needs a `Default`, which you met in world 05: `#[derive(Default)]` or a hand-written one.

**An attribute for serde.** `#[serde(..)]` is an **attribute**, an instruction attached to the item below it, and this one is read only by serde's derive. It changes the code the derive writes, not your struct. Other `serde` attributes rename fields or skip them; the next quest uses one.

```rust
#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
struct Window {
    width: u32,
    maximised: bool,
}
// {"width": 800} reads as Window { width: 800, maximised: false }
```

**Inside, too.** The attribute works per struct. A nested `Settings` with only `{"sound": false}` in the file needs its own `#[serde(default)]` to fill `focus_minutes` from `Settings::default()`; the outer struct's attribute only fills in a `settings` that is missing entirely.

**Why it matters.** Every app that saves files meets this the first time it adds a field. Adding a field with a default is safe forever; renaming or removing one breaks old saves, so think twice before you do.

## Your turn

- Give `Profile` a derived `Default` and the `#[serde(default)]` attribute.
- Give `Settings` the attribute too (its `Default` is written for you).
