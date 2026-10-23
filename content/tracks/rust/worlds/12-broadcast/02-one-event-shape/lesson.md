rl-hud reads two feeds: the official Stats API and the older BakkesMod SOS plugin. Same events, different spelling: `{"Event":"Demolish","Data":{..}}` against `{"event":"game:demolish","data":{..}}`. Everything after this quest sees one `GameEvent`.

**Envelope first.** Every frame is a name and some data. `Envelope` reads just that, keeping the data as a `Value`. `#[serde(alias = "event")]` accepts SOS's lower-case key too.

**Same name.** `SOS_NAMES` pairs each SOS name with its Stats API name. Find the frame's name there; if it is not there, keep the name as it was.

**Name picks the struct.** A `match` on the name reads the data into the right struct; `?` gives `None` when known data is broken:

```rust
"GoalScored" => GameEvent::GoalScored(serde_json::from_value(data).ok()?),
```

**Unknown is fine.** The game sends events this HUD ignores (`ReplayCreated`, `CountdownBegin`). They become `Other(name)`, so logs still show them.

**PascalCase fields.** The structs use `#[serde(rename_all = "PascalCase")]`: `goal_time` in Rust, `GoalTime` on the wire.

## Your turn

- In `normalize`, read the `Envelope`, then map an SOS name to its Stats API name.
- `match` the name: four known events, `Other` for the rest. Save.
