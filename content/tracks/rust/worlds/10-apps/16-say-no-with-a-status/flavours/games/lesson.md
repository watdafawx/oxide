A bug in the launcher sends `{"sizes_mb": []}`, and the server happily answers `{"total_mb": 0}` with `200 OK`. The launcher says "0 MB to download, ready to play", the pack never arrives, and the bug lives on. An empty list is a mistake by the caller, and HTTP has a status for exactly that.

**Say no.** A handler can return a `Result`, like any function. axum turns `Ok(..)` into the normal response and `Err(..)` into the error response, so both sides of the `Result` must be things axum can send. `Json<T>` is one; so is a pair `(StatusCode, Json<T>)`, which sends that status with that body.

**The return type.** `Result<Json<Total>, (StatusCode, Json<ApiError>)>` reads: "a total as JSON, or a status with an error as JSON". The success side still sends `200 OK`; the error side sends whatever status you put in the pair.

**Status codes are constants.** `StatusCode::BAD_REQUEST` is 400, `StatusCode::NOT_FOUND` is 404, `StatusCode::CREATED` is 201; they live in `axum::http::StatusCode`. Picking 400 tells the launcher "you sent something wrong", so it can show the message instead of a size.

```rust
async fn minutes_played(Json(req): Json<Playtime>)
    -> Result<Json<Played>, (StatusCode, Json<ApiError>)> {
    if req.sessions_min.iter().any(|m| *m > 1440) {
        let error = "a session can't be longer than a day".to_string();
        return Err((StatusCode::BAD_REQUEST, Json(ApiError { error })));
    }
    Ok(Json(Played { minutes: req.sessions_min.iter().sum() }))
}
```

**An error with words.** A bare 400 says only "no". A small JSON body like `{"error": "no files to download"}` says why, and the person fixing the launcher reads it in their logs. `ApiError` in the starter is that body.

## Your turn

- Change `total_size` to return a `Result`: a total as JSON, or a status with an `ApiError`.
- When `sizes_mb` is empty, answer `400` with an `ApiError` that says why.
- Otherwise answer the total, wrapped in `Ok`.
