Oxide's UI and its engine talk in JSON. **serde** turns your structs into JSON and back, and you write almost none of that code yourself.

- Add `#[derive(Serialize, Deserialize)]` and the struct can be converted to and from any format.
- `serde_json::to_string(&value)` gives a `String`; `serde_json::from_str::<T>(&text)` parses one. Both return a `Result`.
- `Option<String>` is `null` or a string in JSON.
- Attributes tune the shape: `#[serde(rename = "type")]` renames one field, `#[serde(rename_all = "lowercase")]` renames every enum variant, `#[serde(default)]` fills missing fields from `Default`.

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
enum Kind { Exercise, Boss }

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Quest { title: String, kind: Kind, note: Option<String> }

let q = Quest { title: "Box".into(), kind: Kind::Boss, note: None };
let json = serde_json::to_string(&q)?;   // {"title":"Box","kind":"boss","note":null}
let back: Quest = serde_json::from_str(&json)?;
assert_eq!(q, back);
```

Old save files lack newer fields. `#[serde(default)]` on the struct keeps them loading.

## Your turn

In `src/lib.rs` (the shapes follow `Profile` in the Oxide API docs):

- Derive `Serialize` and `Deserialize` on `Settings` and `Profile`, and `#[serde(default)]` on `Profile`.
- Make `ObjectiveStatus` serialise as `"passed"`, `"failed"`, `"notrun"`.
- Fill in `to_json`, `from_json` and `status_to_json`.
