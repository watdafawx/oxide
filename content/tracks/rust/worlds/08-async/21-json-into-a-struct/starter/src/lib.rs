use reqwest::Client;
use serde::Deserialize;

/// One episode, as the feed server lists it.
// TODO: let serde write the code that reads an Episode from JSON
#[derive(Debug, PartialEq)]
pub struct Episode {
    pub id: u32,
    pub title: String,
    pub size: u64,
}

/// GET `url`, fail on 4xx/5xx, and read the body as a JSON list of episodes.
pub async fn fetch_feed(client: &Client, url: &str) -> Result<Vec<Episode>, reqwest::Error> {
    // TODO: GET url and turn an error status into an Err, as in the last quest
    // TODO: read the body as a list of Episode values
    let _ = (client, url);
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;
    use tokio::time::timeout;

    /// Starts a local HTTP server that answers every request with 200 and `body`.
    async fn serve(body: &'static str) -> String {
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
                        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
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
    async fn fetch_feed_reads_every_episode() {
        let base = serve(
            r#"[{"id":3,"title":"Monday","size":48000000,"artwork":"m.jpg"},
                {"id":4,"title":"Tuesday","size":51000000}]"#,
        )
        .await;
        let feed = timeout(Duration::from_secs(5), fetch_feed(&Client::new(), &base))
            .await
            .expect("timed out")
            .expect("the feed should parse; unknown fields like artwork are skipped");
        assert_eq!(
            feed,
            vec![
                Episode { id: 3, title: "Monday".into(), size: 48_000_000 },
                Episode { id: 4, title: "Tuesday".into(), size: 51_000_000 },
            ]
        );
    }

    #[tokio::test]
    async fn a_missing_field_is_an_error() {
        let base = serve(r#"[{"id":5,"title":"Wednesday"}]"#).await;
        let r = timeout(Duration::from_secs(5), fetch_feed(&Client::new(), &base))
            .await
            .expect("timed out");
        assert!(r.is_err(), "an episode without a size must not parse, got {r:?}");
    }
}
