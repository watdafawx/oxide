use rayon::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, mpsc};
use std::thread;

/// Counts hashed jars. Shared between threads by `&`.
#[derive(Default)]
pub struct Progress {
    done: AtomicUsize,
}

impl Progress {
    /// A counter at zero.
    pub fn new() -> Self {
        Self { done: AtomicUsize::new(0) }
    }

    /// One more jar hashed. Callable from many threads at once.
    pub fn inc(&self) {
        self.done.fetch_add(1, Ordering::Relaxed);
    }

    /// How many jars are hashed so far.
    pub fn done(&self) -> usize {
        self.done.load(Ordering::Relaxed)
    }
}

/// FNV-1a, 64 bit. Start value (offset basis) 0xcbf2_9ce4_8422_2325, prime 0x0000_0100_0000_01b3.
/// For each byte: XOR the byte in, then wrapping-multiply by the prime.
pub fn fnv1a(data: &[u8]) -> u64 {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for byte in data {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}

/// Every file (not folder) below `dir` (the mods folder), at any depth, sorted by path.
/// A folder that cannot be read gives an empty list.
pub fn walk(dir: &Path) -> Vec<PathBuf> {
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(_) => return Vec::new(),
    };
    let mut files = Vec::new();
    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.is_dir() {
                for file in walk(&path) {
                    files.push(file);
                }
            } else {
                files.push(path);
            }
        }
    }
    files.sort();
    files
}

/// Given: reads one jar and hashes its bytes.
pub fn hash_file(path: &Path) -> u64 {
    fnv1a(&fs::read(path).expect("file should be readable"))
}

/// Hashes all jar `files` with a pool of `workers` threads and bumps `progress`
/// once per hashed jar. Returns (path, hash) pairs sorted by path.
pub fn hash_mods_tracked(files: Vec<PathBuf>, workers: usize, progress: &Progress) -> Vec<(PathBuf, u64)> {
    let (job_tx, job_rx) = mpsc::channel::<PathBuf>();
    let jobs = Arc::new(Mutex::new(job_rx));
    let (res_tx, res_rx) = mpsc::channel();

    thread::scope(|s| {
        for _ in 0..workers {
            let jobs = Arc::clone(&jobs);
            let results = res_tx.clone();
            s.spawn(move || {
                loop {
                    let job = jobs.lock().unwrap().recv();
                    match job {
                        Ok(path) => {
                            let hash = hash_file(&path);
                            progress.inc();
                            results.send((path, hash)).unwrap();
                        }
                        Err(_) => break,
                    }
                }
            });
        }
        for file in files {
            job_tx.send(file).unwrap();
        }
        drop(job_tx);
        drop(res_tx);

        let mut out: Vec<(PathBuf, u64)> = res_rx.iter().collect();
        out.sort();
        out
    })
}

/// Same as `hash_mods_tracked`, without a progress counter anyone reads.
pub fn hash_mods(files: Vec<PathBuf>, workers: usize) -> Vec<(PathBuf, u64)> {
    hash_mods_tracked(files, workers, &Progress::new())
}

/// Same result as `hash_mods`, using rayon. Sorted by path.
pub fn hash_mods_rayon(files: &[PathBuf]) -> Vec<(PathBuf, u64)> {
    let mut out: Vec<(PathBuf, u64)> = files
        .par_iter()
        .map(|path| (path.clone(), hash_file(path)))
        .collect();
    out.sort();
    out
}

/// First line "<n> files". Then one line per result: the hash as 16 lowercase hex
/// digits, two spaces, the path (`path.display()`). Lines joined by '\n', no trailing newline.
pub fn report(results: &[(PathBuf, u64)]) -> String {
    let mut lines = vec![format!("{} files", results.len())];
    for (path, hash) in results {
        lines.push(format!("{hash:016x}  {}", path.display()));
    }
    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc::RecvTimeoutError;
    use std::time::Duration;

    /// Runs `f` on a helper thread; fails instead of hanging if it takes too long.
    fn within<T: Send + 'static>(f: impl FnOnce() -> T + Send + 'static) -> T {
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let _ = tx.send(f());
        });
        match rx.recv_timeout(Duration::from_secs(10)) {
            Ok(v) => v,
            Err(RecvTimeoutError::Timeout) => {
                panic!("took over 10 s: did a channel never close (a Sender still alive)?")
            }
            Err(RecvTimeoutError::Disconnected) => panic!("the function panicked (see above)"),
        }
    }

    /// Creates a unique folder under the system temp dir. Returns it and the files in it, sorted.
    fn make_tree(tag: &str) -> (PathBuf, Vec<PathBuf>) {
        let dir = std::env::temp_dir().join(format!("oxide_conc_boss_mods_{tag}_{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(dir.join("mods/libs")).unwrap();
        fs::create_dir_all(dir.join("mods/pack")).unwrap();
        let mut files = Vec::new();
        let mut put = |rel: String, bytes: Vec<u8>| {
            let path = dir.join(&rel);
            fs::write(&path, bytes).unwrap();
            files.push(path);
        };
        put("mods/sodium.jar".into(), b"sodium 0.5.8".to_vec());
        put("mods/iris.jar".into(), b"iris 1.7".to_vec());
        put("mods/libs/fabric-api.jar".into(), b"fabric api".to_vec());
        put("mods/libs/cloth-config.jar".into(), b"cloth".to_vec());
        put("options.txt".into(), vec![0, 1, 2, 255]);
        for i in 0..20 {
            put(format!("mods/pack/addon_{i:02}.jar"), format!("addon number {i} ").repeat(i + 1).into_bytes());
        }
        files.sort();
        (dir, files)
    }

    fn expected(files: &[PathBuf]) -> Vec<(PathBuf, u64)> {
        let mut out: Vec<_> = files
            .iter()
            .map(|p| (p.clone(), fnv1a(&fs::read(p).unwrap())))
            .collect();
        out.sort();
        out
    }

    fn reversed(files: &[PathBuf]) -> Vec<PathBuf> {
        files.iter().rev().cloned().collect()
    }

    #[test]
    fn progress_counts_jars_across_workers() {
        let p = Progress::new();
        assert_eq!(p.done(), 0);
        for _ in 0..3 {
            p.inc();
        }
        assert_eq!(p.done(), 3);
        thread::scope(|s| {
            for _ in 0..4 {
                s.spawn(|| {
                    for _ in 0..250 {
                        p.inc();
                    }
                });
            }
        });
        assert_eq!(p.done(), 1003);
    }

    #[test]
    fn fnv1a_matches_the_published_values() {
        assert_eq!(fnv1a(b""), 0xcbf29ce484222325);
        assert_eq!(fnv1a(b"a"), 0xaf63dc4c8601ec8c);
        assert_eq!(fnv1a(b"foobar"), 0x85944171f73967e8);
    }

    #[test]
    fn walk_finds_every_jar_sorted() {
        let (dir, files) = make_tree("walk");
        assert_eq!(walk(&dir), files);
        assert!(walk(&dir.join("does-not-exist")).is_empty());
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn hash_mods_matches_a_one_by_one_run() {
        let (dir, files) = make_tree("pool");
        let input = reversed(&files);
        let got = within(move || hash_mods(input, 3));
        assert_eq!(got, expected(&files));
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn hash_mods_handles_odd_worker_counts() {
        let (dir, files) = make_tree("edges");
        assert!(within(|| hash_mods(Vec::new(), 4)).is_empty());

        let three: Vec<PathBuf> = files.iter().take(3).cloned().collect();
        let want = expected(&three);
        let t1 = three.clone();
        assert_eq!(within(move || hash_mods(t1, 1)), want, "one worker");
        let t2 = three.clone();
        assert_eq!(within(move || hash_mods(t2, 16)), want, "more workers than files");
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn hash_mods_tracked_counts_every_jar() {
        let (dir, files) = make_tree("tracked");
        let progress = Arc::new(Progress::new());
        let p = Arc::clone(&progress);
        let input = files.clone();
        let got = within(move || hash_mods_tracked(input, 4, &p));
        assert_eq!(progress.done(), files.len());
        assert_eq!(got, expected(&files));
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn hash_mods_rayon_matches_the_pool() {
        let (dir, files) = make_tree("rayon");
        let input = reversed(&files);
        assert_eq!(hash_mods_rayon(&input), expected(&files));
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn report_lists_hex_hashes_per_jar() {
        let results = vec![
            (PathBuf::from("mods/a.jar"), 0xff),
            (PathBuf::from("mods/libs/b.jar"), 0xaf63dc4c8601ec8c),
        ];
        let expected = "2 files\n00000000000000ff  mods/a.jar\naf63dc4c8601ec8c  mods/libs/b.jar";
        assert_eq!(report(&results), expected);
        assert_eq!(report(&[]), "0 files");
    }
}
