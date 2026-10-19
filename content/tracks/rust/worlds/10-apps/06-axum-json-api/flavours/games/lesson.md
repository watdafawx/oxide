**Why now:** a status string is not an API. The launcher wants to send JSON ("here are the file sizes") and get JSON back, with proper status codes when it sends rubbish, and the server has to remember things between requests (how many installs so far).

```rust
use axum::{extract::{Path, State}, http::StatusCode, Json, Router, routing::{get, post}};

async fn welcome(Path(player): Path<String>) -> Json<Welcome> {          // /welcome/Steve
    Json(Welcome { welcome: format!("Welcome, {player}!") })
}

async fn total_size(Json(req): Json<SizesRequest>)                         // JSON body in
    -> Result<Json<Total>, (StatusCode, Json<ApiError>)> {                 // JSON out, or a status + JSON error
    if req.sizes_kb.is_empty() {
        return Err((StatusCode::BAD_REQUEST, Json(ApiError { error: "no files".into() })));
    }
    Ok(Json(Total { total_kb: req.sizes_kb.iter().sum() }))
}
```

- **Extractors** pull data out of the request by their type: `Path<T>`, `Json<T>`, `State<T>`.
- `Json<T>` as an argument parses the body (`Deserialize`). As a return it serialises it (`Serialize`).
- A handler returning `Result<_, (StatusCode, Json<_>)>` chooses the status: `Err` is a 400, `Ok` a 200.
- Shared server state goes in `State`, built with `.with_state(...)`. Use `Arc<Atomic..>` or `Arc<Mutex<..>>`: handlers run concurrently.
- A tuple `(StatusCode::CREATED, Json(..))` sets a non-200 success status.

## Your turn

In `src/lib.rs` (the types are given):

- `welcome`, `total_size`, `install` (the POST) and `installs` (the GET) handlers.
- `app`: wire `/welcome/{player}`, `/total-size`, and `/installs` (GET and POST) with the shared state.
