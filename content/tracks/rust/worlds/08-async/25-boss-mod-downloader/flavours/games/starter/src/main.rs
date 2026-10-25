use async_boss_downloader::*;
use std::panic::{self, AssertUnwindSafe};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

type Files = Arc<Vec<(String, Vec<u8>)>>;

/// Answers one HTTP request: /manifest.json, /files/<name>, or 404.
/// `sodium.jar` fails with a 500 the first time, to show the retry.
async fn handle(mut stream: TcpStream, files: Files, flaky: Arc<AtomicBool>) {
    let mut buf = Vec::new();
    let mut chunk = [0u8; 1024];
    while !buf.windows(4).any(|w| w == b"\r\n\r\n") {
        match stream.read(&mut chunk).await {
            Ok(0) | Err(_) => return,
            Ok(n) => buf.extend_from_slice(&chunk[..n]),
        }
    }
    let request = String::from_utf8_lossy(&buf).to_string();
    let path = request.split_whitespace().nth(1).unwrap_or("/");
    let (status, body) = if path == "/manifest.json" {
        let entries: Vec<String> = files
            .iter()
            .map(|(name, data)| {
                format!(r#"{{"name":"{name}","path":"/files/{name}","size":{}}}"#, data.len())
            })
            .collect();
        ("200 OK", format!("[{}]", entries.join(",")).into_bytes())
    } else if path == "/files/sodium.jar" && flaky.swap(false, Ordering::SeqCst) {
        ("500 Internal Server Error", b"oops".to_vec())
    } else if let Some(file) = files.iter().find(|(name, _)| format!("/files/{name}") == path) {
        ("200 OK", file.1.clone())
    } else {
        ("404 Not Found", b"missing".to_vec())
    };
    let head = format!(
        "HTTP/1.1 {status}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        body.len()
    );
    stream.write_all(head.as_bytes()).await.ok();
    stream.write_all(&body).await.ok();
}

async fn demo() {
    let files: Files = Arc::new(
        [("sodium.jar", 40_000), ("lithium.jar", 25_000), ("iris.jar", 60_000)]
            .into_iter()
            .map(|(name, size)| (name.to_string(), vec![7u8; size]))
            .collect(),
    );
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let base = format!("http://{}", listener.local_addr().unwrap());
    let flaky = Arc::new(AtomicBool::new(true));
    {
        let files = Arc::clone(&files);
        tokio::spawn(async move {
            while let Ok((stream, _)) = listener.accept().await {
                tokio::spawn(handle(stream, Arc::clone(&files), Arc::clone(&flaky)));
            }
        });
    }
    println!("Fake mod server at {base}");

    let manifest = match fetch_manifest(&base).await {
        Ok(manifest) => manifest,
        Err(e) => return println!("Could not fetch the manifest: {e}"),
    };
    println!("Manifest: {} files", manifest.len());

    let dir = std::env::temp_dir().join("oxide-mod-downloader-demo");
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();

    let (tx, mut rx) = mpsc::channel(8);
    let printer = tokio::spawn(async move {
        while let Some((name, done, total)) = rx.recv().await {
            println!("  [{done}/{total}] {name}");
        }
    });
    println!("Downloading 2 at a time ...");
    let result = download_all(manifest.clone(), &base, &dir, 2, tx, CancellationToken::new()).await;
    printer.await.ok();
    match result {
        Ok(()) => println!("All downloaded."),
        Err(e) => println!("Download failed: {e}"),
    }
    let bad = verify(&dir, &manifest);
    if bad.is_empty() {
        println!("Verify: every file is present and the right size.");
    } else {
        println!("Verify: missing or wrong size: {}", bad.join(", "));
    }
    let _ = std::fs::remove_dir_all(&dir);
}

fn main() {
    println!("== Mod downloader ==");
    panic::set_hook(Box::new(|_| {}));
    let rt = tokio::runtime::Runtime::new().unwrap();
    let result = panic::catch_unwind(AssertUnwindSafe(|| rt.block_on(demo())));
    if result.is_err() {
        println!("Finish `fetch_manifest`, `download_one`, `download_with_retry`, `download_all` and `verify` to download the pack.");
    }
}
