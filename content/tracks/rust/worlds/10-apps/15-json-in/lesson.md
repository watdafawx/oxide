At the end of a session the screen sends the engine every XP award it showed, and asks for the total: `POST /xp-total` with the body `{"amounts": [50, 30, 40]}`. This time the data is not in the path but in the request's body, as JSON.

**The body is an extractor.** The same `Json<T>` you returned last quest also works as a parameter. `Json(req): Json<XpRequest>` asks axum to read the body and deserialize it into an `XpRequest`, so the struct needs `Deserialize`. As a return value, `Json<T>` needs `Serialize`; as a parameter, `Deserialize`.

**The wrong shape is refused.** If the body is JSON but doesn't fit the struct, say `{"amount": 5}` with no `amounts` list, axum answers `422 Unprocessable Entity` ("I read it, it isn't what this route takes"). If the body isn't JSON at all, it answers `400`. Either way your handler never runs with half-filled data.

**Order matters, a little.** A handler can take several extractors, like `Path` and `Json` together. The body can be read only once, so `Json` must be the last parameter; the compiler complains if it isn't.

```rust
#[derive(Deserialize)]
struct Minutes {
    sessions: Vec<u32>,
}

async fn study_time(Json(req): Json<Minutes>) -> Json<Total> {
    Json(Total { total: req.sessions.iter().sum() })
}
```

**Adding up.** From world 03, `.iter().sum()` adds every number in a list, and the field it goes into (`total: u32`) tells it what kind of number to make. An empty list sums to 0; the next quest decides whether that is acceptable.

## Your turn

- Give `xp_total` a `Json<XpRequest>` parameter for the body.
- Add up the amounts and answer with a `Total` as JSON.
- The route `POST /xp-total` is already in `app`.
