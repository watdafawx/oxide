use std::path::Path;

use serde::Deserialize;
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

/// Any error. `?` converts reqwest and io errors into it.
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;

/// One file of the modpack, as listed in manifest.json.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Entry {
    pub name: String,
    pub path: String,
    pub size: u64,
}

/// (file name, files finished so far, total files)
pub type Progress = (String, usize, usize);

impl Entry {
    /// base "http://host:80" (maybe with a trailing /) + path "/files/a.jar".
    pub fn url(&self, base: &str) -> String {
        // TODO: trim a trailing slash from base, then add self.path
        todo!()
    }
}

/// GET {base}/manifest.json and parse it as a list of entries.
pub async fn fetch_manifest(base: &str) -> Result<Vec<Entry>> {
    // TODO: reqwest::get, error_for_status, .json::<Vec<Entry>>()
    todo!()
}

/// Download one file into `dir`. Write `<name>.part` first, rename to `<name>` when complete.
pub async fn download_one(
    client: &reqwest::Client,
    base: &str,
    entry: &Entry,
    dir: &Path,
) -> Result<()> {
    // TODO: GET the entry's url with the client and fail on a 4xx/5xx status
    // TODO: read the body bytes, write them to the .part file, then rename it to the real name
    todo!()
}

/// Given: true if the error is an HTTP 5xx answer.
fn is_server_error(e: &Error) -> bool {
    e.downcast_ref::<reqwest::Error>()
        .and_then(|r| r.status())
        .is_some_and(|s| s.is_server_error())
}

/// Like download_one, but a 5xx answer is retried once. Other errors (404...) are not.
pub async fn download_with_retry(
    client: &reqwest::Client,
    base: &str,
    entry: &Entry,
    dir: &Path,
) -> Result<()> {
    // TODO: first attempt; on Err(e) with is_server_error(&e) try once more
    todo!()
}

/// Given: deletes leftover `*.part` files in `dir`.
async fn remove_partials(dir: &Path) {
    if let Ok(mut rd) = tokio::fs::read_dir(dir).await {
        while let Ok(Some(e)) = rd.next_entry().await {
            if e.path().extension().is_some_and(|x| x == "part") {
                let _ = tokio::fs::remove_file(e.path()).await;
            }
        }
    }
}

/// Downloads all entries into `dir`, at most `max_concurrent` at a time.
/// After each finished file sends `(name, done, total)` on `progress`.
/// If `token` is cancelled: stop all downloads, remove `.part` files, return Err.
pub async fn download_all(
    entries: Vec<Entry>,
    base: &str,
    dir: &Path,
    max_concurrent: usize,
    progress: mpsc::Sender<Progress>,
    token: CancellationToken,
) -> Result<()> {
    // TODO: Semaphore (in an Arc) + JoinSet; one task per entry; permit first, then download_with_retry
    // TODO: loop, racing the token's cancellation against the next finished task: on cancel stop
    // every task, remove the partial files and return an error; otherwise report progress
    let _ = remove_partials; // delete this line once you use remove_partials
    todo!()
}

/// Names of manifest files that are missing in `dir` or have a different size.
pub fn verify(dir: &Path, manifest: &[Entry]) -> Vec<String> {
    // TODO: read each file's metadata and compare its length with entry.size
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::path::PathBuf;
    use std::sync::{Arc, Mutex};
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::time::{Duration, Instant};
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;
    use tokio::time::{sleep, timeout};

    // ---- test helpers: a local HTTP server and a temp dir. No real network. ----

    struct Route {
        status: u16,
        body: Vec<u8>,
        delay_ms: u64,
        fail_first: bool, // answer 500 the first time, then `status`
    }

    impl Route {
        fn ok(body: impl Into<Vec<u8>>) -> Route {
            Route { status: 200, body: body.into(), delay_ms: 0, fail_first: false }
        }
    }

    struct Server {
        base: String,
        hits: Arc<Mutex<HashMap<String, usize>>>,
        max_active: Arc<AtomicUsize>,
    }

    impl Server {
        fn hits(&self, path: &str) -> usize {
            *self.hits.lock().unwrap().get(path).unwrap_or(&0)
        }
    }

    async fn serve(routes: Vec<(&str, Route)>) -> Server {
        let routes: Arc<HashMap<String, Route>> =
            Arc::new(routes.into_iter().map(|(p, r)| (p.to_string(), r)).collect());
        let hits = Arc::new(Mutex::new(HashMap::new()));
        let active = Arc::new(AtomicUsize::new(0));
        let max_active = Arc::new(AtomicUsize::new(0));
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let (r2, h2, a2, m2) = (routes.clone(), hits.clone(), active.clone(), max_active.clone());
        tokio::spawn(async move {
            loop {
                let (mut sock, _) = listener.accept().await.unwrap();
                let (routes, hits, active, max_active) =
                    (r2.clone(), h2.clone(), a2.clone(), m2.clone());
                tokio::spawn(async move {
                    let mut buf = [0u8; 2048];
                    let mut seen = Vec::new();
                    while !seen.windows(4).any(|w| w == b"\r\n\r\n") {
                        match sock.read(&mut buf).await {
                            Ok(0) | Err(_) => return,
                            Ok(n) => seen.extend_from_slice(&buf[..n]),
                        }
                    }
                    let head = String::from_utf8_lossy(&seen).to_string();
                    let path = head.split_whitespace().nth(1).unwrap_or("/").to_string();
                    let (status, body) = match routes.get(&path) {
                        None => (404, Vec::new()),
                        Some(route) => {
                            let nth = {
                                let mut h = hits.lock().unwrap();
                                let c = h.entry(path.clone()).or_insert(0);
                                *c += 1;
                                *c
                            };
                            let now = active.fetch_add(1, Ordering::SeqCst) + 1;
                            max_active.fetch_max(now, Ordering::SeqCst);
                            sleep(Duration::from_millis(route.delay_ms)).await;
                            active.fetch_sub(1, Ordering::SeqCst);
                            let status = if route.fail_first && nth == 1 { 500 } else { route.status };
                            (status, route.body.clone())
                        }
                    };
                    let head = format!(
                        "HTTP/1.1 {status} X\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                        body.len()
                    );
                    let _ = sock.write_all(head.as_bytes()).await;
                    let _ = sock.write_all(&body).await;
                    let _ = sock.shutdown().await;
                });
            }
        });
        Server { base: format!("http://{addr}"), hits, max_active }
    }

    struct TempDir(PathBuf);

    impl TempDir {
        fn new(tag: &str) -> TempDir {
            static N: AtomicUsize = AtomicUsize::new(0);
            let p = std::env::temp_dir().join(format!(
                "oxide_boss_{tag}_{}_{}",
                std::process::id(),
                N.fetch_add(1, Ordering::SeqCst)
            ));
            std::fs::create_dir_all(&p).unwrap();
            TempDir(p)
        }
        fn path(&self) -> &Path {
            &self.0
        }
    }

    impl Drop for TempDir {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    /// `count` fake mod files: entries plus the routes that serve them.
    fn fixture(count: usize, delay_ms: u64) -> (Vec<Entry>, Vec<(String, Route)>) {
        let mut entries = Vec::new();
        let mut routes = Vec::new();
        for i in 0..count {
            let body = vec![b'a' + i as u8; 10 + i];
            let entry = Entry {
                name: format!("mod{i}.jar"),
                path: format!("/files/mod{i}.jar"),
                size: body.len() as u64,
            };
            let mut route = Route::ok(body);
            route.delay_ms = delay_ms;
            routes.push((entry.path.clone(), route));
            entries.push(entry);
        }
        (entries, routes)
    }

    fn as_str_routes(routes: &[(String, Route)]) -> Vec<(&str, Route)> {
        routes
            .iter()
            .map(|(p, r)| {
                (
                    p.as_str(),
                    Route {
                        status: r.status,
                        body: r.body.clone(),
                        delay_ms: r.delay_ms,
                        fail_first: r.fail_first,
                    },
                )
            })
            .collect()
    }

    // ---- the objectives ----

    #[test]
    fn entry_url_joins_base_and_path() {
        let e = Entry { name: "a.jar".into(), path: "/files/a.jar".into(), size: 1 };
        assert_eq!(e.url("http://127.0.0.1:9"), "http://127.0.0.1:9/files/a.jar");
        assert_eq!(e.url("http://127.0.0.1:9/"), "http://127.0.0.1:9/files/a.jar");
    }

    #[tokio::test]
    async fn fetch_manifest_parses_entries() {
        let manifest = r#"[{"name":"a.jar","path":"/files/a.jar","size":3},
                           {"name":"b.jar","path":"/files/b.jar","size":5}]"#;
        let server = serve(vec![("/manifest.json", Route::ok(manifest))]).await;
        let got = timeout(Duration::from_secs(5), fetch_manifest(&server.base))
            .await
            .expect("timed out")
            .expect("fetch_manifest failed");
        assert_eq!(got.len(), 2);
        assert_eq!(got[0], Entry { name: "a.jar".into(), path: "/files/a.jar".into(), size: 3 });
        assert_eq!(got[1].name, "b.jar");
    }

    #[tokio::test]
    async fn download_one_writes_the_file() {
        let (entries, routes) = fixture(1, 0);
        let server = serve(as_str_routes(&routes)).await;
        let dir = TempDir::new("one");
        let client = reqwest::Client::new();
        timeout(
            Duration::from_secs(5),
            download_one(&client, &server.base, &entries[0], dir.path()),
        )
        .await
        .expect("timed out")
        .expect("download_one failed");
        let saved = std::fs::read(dir.path().join("mod0.jar")).expect("mod0.jar was not written");
        assert_eq!(saved, routes[0].1.body);
        assert!(!dir.path().join("mod0.jar.part").exists(), "the .part file must be renamed away");
    }

    #[tokio::test]
    async fn retry_once_on_5xx() {
        let entry = |p: &str| Entry { name: format!("{}.jar", &p[1..]), path: p.into(), size: 4 };
        let mut flaky = Route::ok("data");
        flaky.fail_first = true;
        let mut broken = Route::ok("data");
        broken.status = 500;
        let mut missing = Route::ok("");
        missing.status = 404;
        let server = serve(vec![("/flaky", flaky), ("/broken", broken), ("/missing", missing)]).await;
        let dir = TempDir::new("retry");
        let client = reqwest::Client::new();
        let run = |e: Entry| {
            let (client, base, dir) = (client.clone(), server.base.clone(), dir.path().to_path_buf());
            async move {
                timeout(Duration::from_secs(5), download_with_retry(&client, &base, &e, &dir))
                    .await
                    .expect("timed out")
            }
        };

        assert!(run(entry("/flaky")).await.is_ok(), "one 500 then 200 must succeed");
        assert_eq!(server.hits("/flaky"), 2, "exactly one retry");
        assert!(dir.path().join("flaky.jar").exists());

        assert!(run(entry("/broken")).await.is_err(), "two 500s in a row must fail");
        assert_eq!(server.hits("/broken"), 2, "retry only once");

        assert!(run(entry("/missing")).await.is_err());
        assert_eq!(server.hits("/missing"), 1, "a 404 is not retried");
    }

    #[tokio::test]
    async fn download_all_fetches_everything_and_reports_progress() {
        let (entries, routes) = fixture(5, 0);
        let server = serve(as_str_routes(&routes)).await;
        let dir = TempDir::new("all");
        let (tx, mut rx) = mpsc::channel::<Progress>(2);
        let collector = tokio::spawn(async move {
            let mut events = Vec::new();
            while let Some(ev) = rx.recv().await {
                events.push(ev);
            }
            events
        });
        let result = timeout(
            Duration::from_secs(10),
            download_all(
                entries.clone(),
                &server.base,
                dir.path(),
                3,
                tx,
                CancellationToken::new(),
            ),
        )
        .await
        .expect("download_all timed out");
        result.expect("download_all failed");
        let events = timeout(Duration::from_secs(2), collector)
            .await
            .expect("progress channel never closed: is the sender dropped?")
            .unwrap();

        assert_eq!(events.len(), 5, "one event per file, got {events:?}");
        for (i, (_, done, total)) in events.iter().enumerate() {
            assert_eq!((*done, *total), (i + 1, 5), "events must count up: {events:?}");
        }
        let mut names: Vec<_> = events.iter().map(|e| e.0.clone()).collect();
        names.sort();
        assert_eq!(names, vec!["mod0.jar", "mod1.jar", "mod2.jar", "mod3.jar", "mod4.jar"]);
        for (entry, (_, route)) in entries.iter().zip(&routes) {
            let saved = std::fs::read(dir.path().join(&entry.name)).expect("file missing");
            assert_eq!(saved, route.body, "{} has the wrong content", entry.name);
        }
    }

    #[tokio::test]
    async fn download_all_respects_max_concurrent() {
        let (entries, routes) = fixture(6, 80);
        let server = serve(as_str_routes(&routes)).await;
        let dir = TempDir::new("conc");
        let (tx, mut rx) = mpsc::channel::<Progress>(16);
        tokio::spawn(async move { while rx.recv().await.is_some() {} });
        timeout(
            Duration::from_secs(10),
            download_all(entries, &server.base, dir.path(), 2, tx, CancellationToken::new()),
        )
        .await
        .expect("download_all timed out")
        .expect("download_all failed");
        let peak = server.max_active.load(Ordering::SeqCst);
        assert!(peak <= 2, "{peak} downloads ran at once, the limit is 2");
        assert_eq!(peak, 2, "downloads must run concurrently, not one by one");
    }

    #[test]
    fn verify_lists_mismatched_sizes() {
        let dir = TempDir::new("verify");
        let entry = |name: &str, size: u64| Entry {
            name: name.into(),
            path: format!("/{name}"),
            size,
        };
        std::fs::write(dir.path().join("good.jar"), b"12345").unwrap();
        std::fs::write(dir.path().join("short.jar"), b"12").unwrap();
        let manifest = vec![entry("good.jar", 5), entry("short.jar", 5), entry("gone.jar", 3)];
        let mut bad = verify(dir.path(), &manifest);
        bad.sort();
        assert_eq!(bad, vec!["gone.jar", "short.jar"]);
        assert!(verify(dir.path(), &manifest[..1]).is_empty());
    }

    #[tokio::test]
    async fn cancel_stops_midway_and_leaves_no_partials() {
        let (entries, mut routes) = fixture(3, 0);
        routes[2].1.delay_ms = 10_000; // mod2.jar hangs, like a stalled server
        let server = serve(as_str_routes(&routes)).await;
        let dir = TempDir::new("cancel");
        let (tx, mut rx) = mpsc::channel::<Progress>(16);
        tokio::spawn(async move { while rx.recv().await.is_some() {} });
        let token = CancellationToken::new();
        let stopper = token.clone();
        tokio::spawn(async move {
            sleep(Duration::from_millis(400)).await;
            stopper.cancel();
        });

        let start = Instant::now();
        let result = timeout(
            Duration::from_secs(5),
            download_all(entries.clone(), &server.base, dir.path(), 3, tx, token),
        )
        .await
        .expect("download_all did not stop after cancel");
        assert!(result.is_err(), "a cancelled download must return Err");
        assert!(start.elapsed() < Duration::from_secs(3), "cancel took {:?}", start.elapsed());

        let mut left: Vec<String> = std::fs::read_dir(dir.path())
            .unwrap()
            .map(|e| e.unwrap().file_name().to_string_lossy().to_string())
            .collect();
        left.sort();
        assert!(left.iter().all(|n| !n.ends_with(".part")), "partial files left: {left:?}");
        assert!(!left.contains(&"mod2.jar".to_string()), "the stalled file cannot be complete");
        assert!(
            verify(dir.path(), &entries).contains(&"mod2.jar".to_string()),
            "verify must flag the missing file"
        );
    }
}
