#![allow(unused_imports, unused_variables)] // TODO: delete this line when you are done
use rayon::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, mpsc};
use std::thread;

/// Counts finished files. Shared between threads by `&`.
#[derive(Default)]
pub struct Progress {
    done: AtomicUsize,
}

impl Progress {
    /// A counter at zero.
    pub fn new() -> Self {
        // TODO: done: AtomicUsize::new(0)
        todo!()
    }

    /// One more file finished. Callable from many threads at once.
    pub fn inc(&self) {
        // TODO: fetch_add(1, Ordering::Relaxed)
        todo!()
    }

    /// How many files are finished so far.
    pub fn done(&self) -> usize {
        // TODO: load
        todo!()
    }
}

/// FNV-1a, 64 bit. offset basis 0xcbf29ce484222325, prime 0x100000001b3.
pub fn fnv1a(data: &[u8]) -> u64 {
    // TODO: for each byte: hash ^= byte; hash = hash.wrapping_mul(prime)
    todo!()
}

/// Every file (not directory) below `dir`, at any depth, sorted by path.
/// A directory that cannot be read gives an empty list.
pub fn walk(dir: &Path) -> Vec<PathBuf> {
    // TODO: fs::read_dir; recurse into directories; sort at the end
    todo!()
}

/// Given: reads one file and hashes its bytes.
pub fn hash_file(path: &Path) -> u64 {
    fnv1a(&fs::read(path).expect("file should be readable"))
}

/// Hashes all `files` with a pool of `workers` threads (at least 1) and bumps
/// `progress` once per finished file. Returns (path, hash) sorted by path.
pub fn hash_all_tracked(
    files: Vec<PathBuf>,
    workers: usize,
    progress: &Progress,
) -> Vec<(PathBuf, u64)> {
    // TODO: job channel + Arc<Mutex<Receiver>> shared by the workers, result channel,
    // thread::scope, drop the senders you no longer need, collect, sort
    todo!()
}

/// Same as `hash_all_tracked` without a progress counter.
pub fn hash_all(files: Vec<PathBuf>, workers: usize) -> Vec<(PathBuf, u64)> {
    // TODO: call hash_all_tracked with a throwaway &Progress::new()
    todo!()
}

/// Same result as `hash_all`, using rayon. Sorted by path.
pub fn hash_all_rayon(files: &[PathBuf]) -> Vec<(PathBuf, u64)> {
    // TODO: files.par_iter().map(...).collect(), then sort
    todo!()
}

/// First line "<n> files". Then one line per result: the hash as 16 lowercase hex
/// digits, two spaces, the path (`path.display()`). Lines joined by '\n', no trailing newline.
pub fn report(results: &[(PathBuf, u64)]) -> String {
    // TODO: format!("{hash:016x}  {}", path.display()) per result
    todo!()
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
        let dir = std::env::temp_dir().join(format!("oxide_conc_boss_{tag}_{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(dir.join("sub/deeper")).unwrap();
        fs::create_dir_all(dir.join("bulk")).unwrap();
        let mut files = Vec::new();
        let mut put = |rel: String, bytes: Vec<u8>| {
            let path = dir.join(&rel);
            fs::write(&path, bytes).unwrap();
            files.push(path);
        };
        put("a.txt".into(), b"alpha".to_vec());
        put("b.txt".into(), b"bravo".to_vec());
        put("sub/c.txt".into(), b"charlie".to_vec());
        put("sub/deeper/d.txt".into(), b"delta".to_vec());
        put("sub/deeper/e.bin".into(), vec![0, 1, 2, 255]);
        for i in 0..20 {
            put(format!("bulk/file_{i:02}.txt"), format!("file number {i} ").repeat(i + 1).into_bytes());
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
    fn progress_counts_across_threads() {
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
    fn fnv1a_matches_known_vectors() {
        assert_eq!(fnv1a(b""), 0xcbf29ce484222325);
        assert_eq!(fnv1a(b"a"), 0xaf63dc4c8601ec8c);
        assert_eq!(fnv1a(b"foobar"), 0x85944171f73967e8);
    }

    #[test]
    fn walk_lists_every_file_sorted() {
        let (dir, files) = make_tree("walk");
        assert_eq!(walk(&dir), files);
        assert!(walk(&dir.join("does-not-exist")).is_empty());
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn hash_all_matches_a_sequential_run() {
        let (dir, files) = make_tree("pool");
        let input = reversed(&files);
        let got = within(move || hash_all(input, 3));
        assert_eq!(got, expected(&files));
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn hash_all_handles_edge_cases() {
        let (dir, files) = make_tree("edges");
        assert!(within(|| hash_all(Vec::new(), 4)).is_empty());

        let three: Vec<PathBuf> = files.iter().take(3).cloned().collect();
        let want = expected(&three);
        let t1 = three.clone();
        assert_eq!(within(move || hash_all(t1, 1)), want, "one worker");
        let t2 = three.clone();
        assert_eq!(within(move || hash_all(t2, 16)), want, "more workers than files");
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn hash_all_tracked_counts_every_file() {
        let (dir, files) = make_tree("tracked");
        let progress = Arc::new(Progress::new());
        let p = Arc::clone(&progress);
        let input = files.clone();
        let got = within(move || hash_all_tracked(input, 4, &p));
        assert_eq!(progress.done(), files.len());
        assert_eq!(got, expected(&files));
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn hash_all_rayon_matches_the_pool() {
        let (dir, files) = make_tree("rayon");
        let input = reversed(&files);
        assert_eq!(hash_all_rayon(&input), expected(&files));
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn report_lists_hex_hashes() {
        let results = vec![
            (PathBuf::from("a.txt"), 0xff),
            (PathBuf::from("dir/b.txt"), 0xaf63dc4c8601ec8c),
        ];
        let expected = "2 files\n00000000000000ff  a.txt\naf63dc4c8601ec8c  dir/b.txt";
        assert_eq!(report(&results), expected);
        assert_eq!(report(&[]), "0 files");
    }
}
