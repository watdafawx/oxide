The engine hands your `Profile` to the screen as JSON, and saves it to disk the same way. Writing `{"xp": ...}` by hand for every field would be long and easy to get wrong, so you let serde write that code from the struct itself.

**Derive it.** From world 02, `#[derive(..)]` asks the compiler to write a trait's code for you. The serde crate adds two derives of its own: `Serialize` (the struct can be written out) and `Deserialize` (it can be read back). The `features = ["derive"]` line in `Cargo.toml` is what switches them on. Each field becomes a key with the same name.

**To text.** serde itself doesn't know any format; the `serde_json` crate is the JSON half. `serde_json::to_string(&value)` gives the JSON text as a `String`, inside a `serde_json::Result`, because some values (a map with non-text keys, say) can't be written as JSON.

**From text.** `serde_json::from_str(text)` reads it back. It needs to know which type to build, and gets it from where the value goes: your function's return type, a `let s: Settings`, or a turbofish `from_str::<Settings>(..)` from world 03. Text with a missing field, a typo or the wrong kind of value gives an `Err` that says where.

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Settings {
    sound: bool,
    editor: Option<String>,
}

let json = serde_json::to_string(&settings)?;   // {"sound":true,"editor":null}
let back: Settings = serde_json::from_str(&json)?;
```

**None is null.** An `Option` field becomes `null` when it is `None` and the plain value when it is `Some`. A `Vec<String>` becomes an array of strings. Nested structs become nested objects, as long as they derive the same two traits.

**Round trip.** The surest test of a format is to write a value, read it back and compare: `from_json(&to_json(&p)?)? == p`. If any field is lost or renamed on the way, the two differ and the test says which.

## Your turn

- Add the two serde derives to `Profile`.
- In `to_json`, write the profile as JSON text.
- In `from_json`, read JSON text back into a `Profile`.
