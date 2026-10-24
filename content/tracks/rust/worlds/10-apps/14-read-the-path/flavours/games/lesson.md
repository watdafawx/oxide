The mod manager ranks your library in tiers: tier 1 under 100 MB, one more tier every 100 MB. A launcher asks "what tier is 250 MB?" with `GET /tier/250` and wants JSON back: `{"mb": 250, "tier": 3}`. The number sits in the path itself, so the handler has to pull it out, and the answer has to be JSON instead of plain text.

**A gap in the path.** A route path can have a gap in braces: `/tier/{mb}` matches `/tier/250`, `/tier/0`, `/tier/lots`, one segment of anything. The name inside the braces is only a label for you.

**An extractor.** You get the gap by asking for it in the handler's parameters: a parameter of type `Path<u32>` tells axum to take the path's gap and turn it into a `u32` before calling you. Anything a handler asks for this way is an **extractor**. Writing it as `Path(mb): Path<u32>` unpacks the number straight into `mb`, the same unpacking as `let (a, b) = pair;` from world 01.

**Bad input never reaches you.** If the gap can't become a `u32`, as in `/tier/lots`, axum answers `400 Bad Request` itself and your handler never runs. The type in the signature is the check, so the handler's body can trust `mb`.

**JSON out.** Return `Json(value)` instead of text, where `value` derives `Serialize` from earlier in this world. axum writes it with serde_json, sends `200 OK`, and marks the body as JSON so the launcher knows how to read it.

```rust
#[derive(Serialize)]
struct ChunkInfo {
    x: i32,
    block_x: i32,
}

async fn chunk(Path(x): Path<i32>) -> Json<ChunkInfo> {
    Json(ChunkInfo { x, block_x: x * 16 })
}
// .route("/chunk/{x}", get(chunk))
```

## Your turn

- Give `tier` a `Path<u32>` parameter for the megabytes.
- Answer with a `TierInfo` as JSON, using `tier_for`.
- Add the route `GET /tier/{mb}` to `app`.
