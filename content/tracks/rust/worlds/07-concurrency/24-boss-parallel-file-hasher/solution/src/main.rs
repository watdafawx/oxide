use conc_boss_hasher::*;
use std::panic;
use std::path::{Path, PathBuf};

fn main() {
    println!("== Parallel file hasher ==");
    panic::set_hook(Box::new(|_| {}));
    let result = panic::catch_unwind(|| {
        let dir = std::env::args().nth(1).unwrap_or_else(|| "src".to_string());
        let files = walk(Path::new(&dir));
        println!("Hashing {} files under `{dir}` ...", files.len());

        let progress = Progress::new();
        let threaded = hash_all_tracked(files.clone(), 4, &progress);
        println!("{}", report(&threaded));
        println!("Progress counter: {}/{}", progress.done(), files.len());

        let rayon = hash_all_rayon(&files);
        let same: Vec<PathBuf> = threaded.iter().map(|(p, _)| p.clone()).collect();
        println!("rayon agrees with the worker pool: {}", rayon == threaded && same == files);
    });
    if result.is_err() {
        println!("Finish `Progress`, `fnv1a`, `walk`, `hash_all_tracked`, `hash_all_rayon` and `report` to hash files.");
    }
}
