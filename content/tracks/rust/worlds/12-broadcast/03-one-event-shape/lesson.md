rl-hud reads two feeds: the official Stats API and the older BakkesMod SOS plugin. Same events, different spelling: `{"Event":"Demolish","Data":{..}}` against `{"event":"game:demolish","data":{..}}`. Everything after this quest sees one `GameEvent`, whichever feed it came from.

**Envelope first.** Every frame is a name and some data. `Envelope` (already written) reads just those two, keeping the data as a raw `Value` for later. Its `#[serde(alias = "event")]` lets the same field accept SOS's lower-case key too.

**Why not all at once?** Which struct the data becomes, `Goal` or `Demo`, depends on the name. So read the name first, then decide. `serde_json::from_value(frame).ok()?` reads a `Value` into any type that derives `Deserialize`; `.ok()?` turns a failure into `None` and returns it at once.

**Same name.** `stats_api_name` (already written) swaps an SOS name like `game:demolish` for the Stats API's `Demolish`, and leaves any other name as it was. Match on what it gives back.

**A match on text.** A `match` on a `&str` lists the names you know and ends with a name binding that catches the rest. The same shape, picking a team number from a colour:

```rust
let team = match colour {
    "blue" => 0,
    "orange" => 1,
    other => return Err(format!("no team called {other}")),
};
```

**Unknown is fine.** The game sends events this HUD ignores (`ReplayCreated`, `CountdownBegin`). They become `GameEvent::Other(name)` rather than an error, so the logs still show them.

**PascalCase fields.** The structs use `#[serde(rename_all = "PascalCase")]`: the field is `goal_time` in Rust and `GoalTime` on the wire.

## Your turn

- In `normalize`, read the frame into an `Envelope`; give back `None` if it is not one.
- `match` on `stats_api_name` of its event: four known names read `data` into their struct, any other name is `Other`. Save.
