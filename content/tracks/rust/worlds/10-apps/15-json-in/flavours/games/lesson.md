Before a modpack download starts, the launcher sends the server the size of every file and asks for the total, so it can warn about disk space: `POST /total-size` with the body `{"sizes_mb": [120, 30, 4]}`. This time the data is not in the path but in the request's body, as JSON.

**The body is an extractor.** The same `Json<T>` you returned last quest also works as a parameter. `Json(req): Json<SizesRequest>` asks axum to read the body and deserialize it into a `SizesRequest`, so the struct needs `Deserialize`. As a return value, `Json<T>` needs `Serialize`; as a parameter, `Deserialize`.

**The wrong shape is refused.** If the body is JSON but doesn't fit the struct, say `{"size": 5}` with no `sizes_mb` list, axum answers `422 Unprocessable Entity` ("I read it, it isn't what this route takes"). If the body isn't JSON at all, it answers `400`. Either way your handler never runs with half-filled data.

**Order matters, a little.** A handler can take several extractors, like `Path` and `Json` together. The body can be read only once, so `Json` must be the last parameter; the compiler complains if it isn't.

```rust
#[derive(Deserialize)]
struct Playtime {
    sessions_min: Vec<u32>,
}

async fn minutes_played(Json(req): Json<Playtime>) -> Json<Played> {
    Json(Played { minutes: req.sessions_min.iter().sum() })
}
```

**Adding up.** From world 03, `.iter().sum()` adds every number in a list, and the field it goes into (`total_mb: u32`) tells it what kind of number to make. An empty list sums to 0; the next quest decides whether that is acceptable.

## Your turn

- Give `total_size` a `Json<SizesRequest>` parameter for the body.
- Add up the sizes and answer with a `Total` as JSON.
- The route `POST /total-size` is already in `app`.
