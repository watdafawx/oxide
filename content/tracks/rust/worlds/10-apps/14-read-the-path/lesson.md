The level badge on the screen asks the engine "what level is 250 XP?" with `GET /level/250`, and wants JSON back: `{"xp": 250, "level": 3}`. The number sits in the path itself, so the handler has to pull it out, and the answer has to be JSON instead of plain text.

**A gap in the path.** A route path can have a gap in braces: `/level/{xp}` matches `/level/250`, `/level/0`, `/level/lots`, one segment of anything. The name inside the braces is only a label for you.

**An extractor.** You get the gap by asking for it in the handler's parameters: a parameter of type `Path<u32>` tells axum to take the path's gap and turn it into a `u32` before calling you. Anything a handler asks for this way is an **extractor**. Writing it as `Path(xp): Path<u32>` unpacks the number straight into `xp`, the same unpacking as `let (a, b) = pair;` from world 01.

**Bad input never reaches you.** If the gap can't become a `u32`, as in `/level/lots`, axum answers `400 Bad Request` itself and your handler never runs. The type in the signature is the check, so the handler's body can trust `xp`.

**JSON out.** Return `Json(value)` instead of text, where `value` derives `Serialize` from earlier in this world. axum writes it with serde_json, sends `200 OK`, and marks the body as JSON so the screen knows how to read it.

```rust
#[derive(Serialize)]
struct StreakInfo {
    days: u32,
    bonus: u32,
}

async fn streak(Path(days): Path<u32>) -> Json<StreakInfo> {
    Json(StreakInfo { days, bonus: days * 5 })
}
// .route("/streak/{days}", get(streak))
```

## Your turn

- Give `level` a `Path<u32>` parameter for the XP.
- Answer with a `LevelInfo` as JSON, using `level_for`.
- Add the route `GET /level/{xp}` to `app`.
