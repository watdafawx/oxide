The Riot servers keep their cues in `triggers.json`, so a producer can change what a goal does without a developer. Here: each trigger names a cue and one output.

**Three outputs.** `Output` is an enum with fields: OSC for Notch and Resolume, a Companion button for Stream Decks, an HTTP call for dashboards and lights. The JSON says which with `"kind"`:

```rust
#[serde(tag = "kind", rename_all = "snake_case")]
```

**Flexible arguments.** OSC arguments can be whole numbers, decimals or text. `#[serde(untagged)]` on `OscArg` tries each variant until one fits.

**Errors with a place.** A producer who types `"kind": "dmx"` should hear where. serde_json's error knows its line; put it in the message.

**Fail at start.** Read the triggers once when the server starts. A broken file stops it before the show, not halfway through it.

## Your turn

- Write `load_triggers`: read the list with `serde_json::from_str`.
- Turn an error into `triggers.json line N: ...` with `e.line()`. Save.
