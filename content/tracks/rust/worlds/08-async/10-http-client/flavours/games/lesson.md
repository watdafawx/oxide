**Why now:** before the downloader can fetch a jar it has to ask the mod site which file to fetch: an HTTP request that answers with JSON. `reqwest` is the async HTTP client, `serde_json::Value` holds whatever JSON comes back.

```rust
use serde_json::Value;

async fn fetch_mod_json(url: &str) -> Result<Value, reqwest::Error> {
    reqwest::get(url)          // send GET
        .await?                // wait for the response
        .error_for_status()?   // 404/500 become Err
        .json()                // parse the body
        .await
}

let v = fetch_mod_json("https://example.com/mod/jei").await?;
println!("{}", v["title"]);    // Value indexes with [] and gives Null when missing
```

- `reqwest::get(url).await?` only fails for network problems. A 404 is still an `Ok(response)` until you call `error_for_status()`.
- `.json::<T>()` can parse into your own struct with serde. `Value` is the "I do not know the shape yet" type.
- `value["title"].as_str()` returns `Option<&str>`: handy for fields that may be missing.
- Tests here start a **local** server on `127.0.0.1` with a random port. No real network is touched.

## Your turn

In `src/lib.rs`:

- `fetch_mod_json`: GET, reject HTTP errors, parse JSON.
- `mod_title`: fetch, then return the `"title"` string if there is one.
