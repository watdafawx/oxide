use serde_json::Value;

/// GET `url`, fail on 4xx/5xx, parse the body as JSON.
pub async fn fetch_mod_json(url: &str) -> Result<Value, reqwest::Error> {
    // TODO: reqwest::get(url).await? -> error_for_status()? -> json().await
    todo!()
}

/// The "title" field of the JSON at `url` (None if there is no string title).
pub async fn mod_title(url: &str) -> Result<Option<String>, reqwest::Error> {
    // TODO: fetch_mod_json, then value["title"].as_str()
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;
    use tokio::time::timeout;

    /// Starts a local HTTP server that answers every request with `status` and `body`.
    /// Returns its base url, e.g. "http://127.0.0.1:40123". No real network involved.
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
        let base = serve(200, r#"{"title":"JEI","versions":[1,2,3]}"#).await;
        let v = timeout(Duration::from_secs(5), fetch_mod_json(&format!("{base}/mod")))
            .await
            .expect("timed out")
            .expect("request failed");
        assert_eq!(v["title"], "JEI");
        assert_eq!(v["versions"].as_array().map(|a| a.len()), Some(3));
    }

    #[tokio::test]
    async fn fetch_mod_json_rejects_http_errors() {
        let base = serve(404, r#"{"error":"not found"}"#).await;
        let r = timeout(Duration::from_secs(5), fetch_mod_json(&format!("{base}/missing")))
            .await
            .expect("timed out");
        assert!(r.is_err(), "a 404 must be an Err, got {r:?}");
    }

    #[tokio::test]
    async fn mod_title_reads_a_field() {
        let with = serve(200, r#"{"title":"Sodium"}"#).await;
        let without = serve(200, r#"{"name":"Sodium"}"#).await;
        let a = timeout(Duration::from_secs(5), mod_title(&with)).await.expect("timed out");
        assert_eq!(a.unwrap().as_deref(), Some("Sodium"));
        let b = timeout(Duration::from_secs(5), mod_title(&without)).await.expect("timed out");
        assert_eq!(b.unwrap(), None);
    }
}
