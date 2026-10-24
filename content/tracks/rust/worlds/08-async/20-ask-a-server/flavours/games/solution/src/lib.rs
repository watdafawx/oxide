use reqwest::Client;
use serde_json::Value;

/// GET `url` with the launcher's shared client, fail on 4xx/5xx, and parse the body as any JSON.
pub async fn fetch_mod_json(client: &Client, url: &str) -> Result<Value, reqwest::Error> {
    let response = client.get(url).send().await?;
    let response = response.error_for_status()?;
    response.json().await
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;
    use tokio::time::timeout;

    /// Starts a local HTTP server that answers every request with `status` and `body`.
    /// Returns its address, e.g. "http://127.0.0.1:40123". No real network involved.
    async fn serve(status: u16, body: &'static str) -> String {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        tokio::spawn(async move {
            loop {
                let (mut sock, _) = listener.accept().await.unwrap();
                tokio::spawn(async move {
                    let mut buf = [0u8; 2048];
                    let mut seen = Vec::new();
                    while !seen.windows(4).any(|w| w == b"\r\n\r\n") {
                        match sock.read(&mut buf).await {
                            Ok(0) | Err(_) => return,
                            Ok(n) => seen.extend_from_slice(&buf[..n]),
                        }
                    }
                    let response = format!(
                        "HTTP/1.1 {status} X\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                        body.len()
                    );
                    let _ = sock.write_all(response.as_bytes()).await;
                    let _ = sock.shutdown().await;
                });
            }
        });
        format!("http://{addr}")
    }

    #[tokio::test]
    async fn fetch_mod_json_parses_the_body() {
        let base = serve(200, r#"{"title":"Sodium","versions":[3,4,5]}"#).await;
        let client = Client::new();
        let v = timeout(Duration::from_secs(5), fetch_mod_json(&client, &format!("{base}/mod/sodium.json")))
            .await
            .expect("timed out")
            .expect("request failed");
        assert_eq!(v["title"], "Sodium");
        assert_eq!(v["versions"].as_array().map(|a| a.len()), Some(3));
    }

    #[tokio::test]
    async fn fetch_mod_json_rejects_http_errors() {
        let base = serve(404, r#"{"error":"not found"}"#).await;
        let client = Client::new();
        let r = timeout(Duration::from_secs(5), fetch_mod_json(&client, &format!("{base}/mod/gone.json")))
            .await
            .expect("timed out");
        assert!(r.is_err(), "a 404 must be an Err, got {r:?}");
    }
}
