Under the bytes, a Minecraft server's status is text, and the text is JSON: the version, the player count, the MOTD. Before the boss, the pinger needs to read one field out of it safely: a modded server that leaves a field out must give an error, not a crash or a made-up 0 players.

**JSON in one card.** **JSON** is a text format for data. An **object** in `{ }` holds named fields, `"name": value`; an **array** in `[ ]` is a list; values are strings in double quotes, numbers, `true`, `false` or `null`. They nest: `{"players": {"online": 3, "max": 20}}` is an object whose `players` field is another object. World 08 read JSON from web servers; here it arrives inside a packet.

**A crate reads it.** The **serde_json** crate parses JSON text: `serde_json::from_str(text)` gives a `Result`, because the text may not be JSON at all. Parsed into a `serde_json::Value`, a Rust enum that can hold any JSON, it is ready to dig into. (World 10 shows the other way, reading JSON straight into your own structs with serde's `#[derive(Deserialize)]`.)

**Dig in with `[ ]`.** `v["players"]["online"]` walks into nested objects by key. A key that is not there gives `Value::Null` instead of panicking, so a long chain of `[ ]` is always safe.

**Ask for a type.** A `Value` doesn't know what you expect, so you ask: `.as_u64()`, `.as_i64()` and `.as_str()` each give an `Option`, `Some` when the value is that kind and `None` when it is missing or a different kind. `Null`, or the text `"three"`, gives `None` from `.as_u64()`.

**From Option to Result.** `option.ok_or(err)` turns `Some(x)` into `Ok(x)` and `None` into `Err(err)`, so a missing field becomes a message the server list can show. For the curious: `ok_or_else(|| ...)` does the same but builds the error only when it is needed.

```rust
use serde_json::Value;

// r#"..."# is a raw string: the quotes inside need no backslashes.
let v: Value = serde_json::from_str(r#"{"version": {"name": "1.21.1"}}"#).unwrap();
let name = v["version"]["name"].as_str();       // Some("1.21.1")
let ghost = v["version"]["codename"].as_str();  // None: no such key
let checked = name.ok_or("missing version.name".to_string());
```

## Your turn

- In `online_players`, parse `json` into a `Value`; a parse error becomes `Err` with its message.
- Read `players.online` as a `u64`, and turn "not there" into `Err("missing players.online")`.
