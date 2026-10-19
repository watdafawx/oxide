**Why now:** the mod database needs a save file and the UI needs data over the wire. Both are JSON. **serde** turns a Rust struct into JSON and back with two derives, and a few attributes cover real-world messes: old save files with missing fields, and enums with specific spellings.

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]                       // any field missing in the JSON uses Default
struct ModpackProfile {
    name: String,
    mod_count: u32,
    last_played: Option<String>,        // None <-> null
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]      // NotInstalled <-> "notinstalled"
enum ModStatus { Enabled, Disabled, NotInstalled }

let json = serde_json::to_string(&profile)?;
let back: ModpackProfile = serde_json::from_str(r#"{"mod_count": 120}"#)?;  // other fields default
```

- `Serialize` writes, `Deserialize` reads. Field names become JSON keys as they are.
- `Option<T>` is `null` when `None`.
- `#[serde(default)]` is how you keep **old saves loading** after you add fields.
- `rename_all` (and `rename` on one field) match whatever spelling the other side uses.
- All of this is checked by round-trip tests: serialize, deserialize, compare.

## Your turn

In `src/lib.rs` the structs are listed; add the derives and attributes, then:

- `to_json` / `from_json`: the two serde_json calls.
- `status_to_json`: serialise one `ModStatus`.
- `LauncherSettings::default`: loader `"fabric"`, auto-update on, keep 3 versions.
