Your mod manager will ask a server "which versions of this mod exist?" and get JSON back. **reqwest** is the usual async HTTP client.

- `reqwest::get(url).await?` sends a GET and gives a `Response`.
- A `404` or `500` is **not** an error by default. `response.error_for_status()?` turns it into one.
- `response.json::<T>().await?` parses the body into any serde type. `serde_json::Value` means "any JSON".
- Every step can fail (network, status, bad JSON), so use `?` and return a `Result`.

```rust
use serde_json::Value;

async fn ip(url: &str) -> Result<String, reqwest::Error> {
    let resp = reqwest::get(url).await?.error_for_status()?;
    let v: Value = resp.json().await?;
    Ok(v["origin"].as_str().unwrap_or("?").to_string())
}
```

Tests here never touch the internet: they start a tiny HTTP server on `127.0.0.1` inside the test and point your function at it.

## Your turn

In `src/lib.rs`:

- `fetch_json`: GET the url, fail on error statuses, parse as `Value`.
- `title_of`: use `fetch_json` and return the `"title"` string, if any.
