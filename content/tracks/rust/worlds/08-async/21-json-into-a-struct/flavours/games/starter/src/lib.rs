use reqwest::Client;
use serde::Deserialize;

/// One file of a modpack, as the mod server lists it.
// TODO: let serde write the code that reads a ModFile from JSON
#[derive(Debug, PartialEq)]
pub struct ModFile {
    pub id: u32,
    pub name: String,
    pub size: u64,
}

/// GET `url`, fail on 4xx/5xx, and read the body as a JSON list of mod files.
pub async fn fetch_modpack(client: &Client, url: &str) -> Result<Vec<ModFile>, reqwest::Error> {
    // TODO: GET url and turn an error status into an Err, as in the last quest
    // TODO: read the body as a list of ModFile values
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
    async fn fetch_modpack_reads_every_file() {
        let base = serve(
            r#"[{"id":3,"name":"sodium.jar","size":4200000,"icon":"s.png"},
                {"id":4,"name":"lithium.jar","size":610000}]"#,
        )
        .await;
        let pack = timeout(Duration::from_secs(5), fetch_modpack(&Client::new(), &base))
            .await
            .expect("timed out")
            .expect("the modpack should parse; unknown fields like icon are skipped");
        assert_eq!(
            pack,
            vec![
                ModFile { id: 3, name: "sodium.jar".into(), size: 4_200_000 },
                ModFile { id: 4, name: "lithium.jar".into(), size: 610_000 },
            ]
        );
    }

    #[tokio::test]
    async fn a_missing_field_is_an_error() {
        let base = serve(r#"[{"id":5,"name":"iris.jar"}]"#).await;
        let r = timeout(Duration::from_secs(5), fetch_modpack(&Client::new(), &base))
            .await
            .expect("timed out");
        assert!(r.is_err(), "a mod file without a size must not parse, got {r:?}");
    }
}
