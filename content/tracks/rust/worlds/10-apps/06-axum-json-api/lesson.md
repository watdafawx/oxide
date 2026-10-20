**Why now:** the mod API must take a JSON mod in a request, remember it, and answer with the right status; each of those is an axum extractor.

axum reads the URL, a JSON body and shared state with **extractors**: a function parameter that pulls something out of the request.

- `Path<T>` reads `/hello/{name}` segments.
- `Json<T>` parses the body (needs `Deserialize`). The same `Json<T>` as a *return value* writes JSON (needs `Serialize`).
- `State<S>` gives every handler the same shared value. `S` must be `Clone`, so wrap shared data in an `Arc`.
- A `StatusCode` or a `(StatusCode, Json<T>)` tuple sets the status. Returning `Result<Ok, Err>` works when both sides are responses.

```rust
async fn add(
    State(total): State<Arc<AtomicU64>>,
    Json(req): Json<Add>,
) -> Result<Json<Out>, (StatusCode, String)> {
    if req.n == 0 {
        return Err((StatusCode::BAD_REQUEST, "n must be > 0".into()));
    }
    let now = total.fetch_add(req.n, Ordering::SeqCst) + req.n;
    Ok(Json(Out { total: now }))
}
```

More pieces from the same family:

- `Path(name): Path<String>` is the same destructuring as `State(..)`: the URL part lands in `name`.
- Reading a shared counter is **`total.load(Ordering::SeqCst)`**; `fetch_add` bumps it and returns the *old* value, hence the `+ req.n`.
- A success with a custom status is a tuple: `(StatusCode::CREATED, Json(Out { total: 1 }))`.
- `req.numbers.iter().sum()` adds a list; the function's return type tells it what number type to make.

Routes are already wired in `app()`. Your job is the handlers.

## Your turn

In `src/lib.rs`, fill in:

- `hello`: greeting JSON from the path.
- `sum`: total of the numbers, or `400` with an `ApiError` when the list is empty.
- `bump` and `count`: use `state.count` (an `Arc<AtomicU64>`); `bump` answers `201 Created`.
