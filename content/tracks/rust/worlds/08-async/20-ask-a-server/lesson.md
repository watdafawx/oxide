The downloader's first real request asks the server for a feed: `GET /feeds/daily.json`, which answers with JSON. The **reqwest** crate is the usual async HTTP client in Rust, and three of its calls do the whole job.

**One client for all requests.** A `reqwest::Client` holds the open connections and the settings. Make one with `Client::new()` and pass `&client` around: every request through it can reuse a connection, while a new client per request would connect from scratch each time. Cloning a `Client` is cheap and shares the same connections.

**Build, send, await.** `client.get(url)` builds a GET request, and `.send().await` sends it and waits for the answer's first lines. Both the network and the server can fail, so it gives a `Result` and you add `?`, the error shortcut from world 04.

**A 404 is not an error yet.** reqwest gives `Ok(response)` for *any* status: a 404 is an answer, so the network worked. `response.error_for_status()` turns a 4xx or 5xx into an `Err` and passes a 2xx through. Skip it and you would try to read the server's error page as your data.

```rust
use reqwest::Client;

async fn fetch_cover_bytes(client: &Client, url: &str) -> Result<Vec<u8>, reqwest::Error> {
    let response = client.get(url).send().await?.error_for_status()?;
    let bytes = response.bytes().await?; // the body, as raw bytes
    Ok(bytes.to_vec())
}
```

**Read the body.** The response has a method for each way to read the body: `.bytes().await` for raw bytes (a file), `.text().await` for a `String`, and `.json().await` to parse JSON into the type your function returns. `serde_json::Value` is the type for "any JSON": `v["title"]` reads a field and gives `Null` when it's missing.

**No real internet.** The tests start a tiny HTTP server on `127.0.0.1`, your own computer, and point your function at it. The first build compiles reqwest and serde_json, which adds a few minutes once.

## Your turn

- In `fetch_json`, send a GET for `url` with `client`.
- Turn an error status into an `Err`, then parse the body as JSON.
