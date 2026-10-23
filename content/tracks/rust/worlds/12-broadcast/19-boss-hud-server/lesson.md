Every piece of the HUD server is written: frames, one event shape, the live feed, goal dedup, grace, moments, triggers, OSC, the overlay socket, Tauri commands and events, and the replay check. This boss writes the step that joins them.

**Always the HUD.** Score and clock come from every `UpdateState`, even in the grace period: the scorebug should be right from the first second.

**Then the gates.** In the grace period nothing fires. A repeated goal fires nothing either, and does not count towards a hat-trick.

**Cues in order.** A goal (or the match end) first, then the moments the event caused: a third goal gives `goal`, then `hat_trick`, at the same time. Each cue carries its triggers' outputs:

```rust
Action { at: now, cue, detail, outputs: self.outputs_for(cue) }
```

**The proof match.** The objectives replay a whole made-up match: a stale goal in the boot burst, one goal sent twice, a hat-trick, four demolitions, 30 seconds left, overtime and an overtime winner. The golden file says what must come out.

**Run it.** `cargo run` replays the match and prints every action. `cargo run -- --live` connects to Rocket League's Stats API and serves overlays on `ws://127.0.0.1:8787/ws`.

## Your turn

- Write `Pipeline::step`: HUD, grace, dedup, then goal or match end, then moments, each with its outputs.
- Press Run to see the match replay.
