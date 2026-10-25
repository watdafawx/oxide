use conc_boss_hasher::*;
use std::panic;
use std::path::{Path, PathBuf};

fn main() {
    println!("== Mod folder hasher ==");
    panic::set_hook(Box::new(|_| {}));
    let result = panic::catch_unwind(|| {
        let dir = std::env::args().nth(1).unwrap_or_else(|| "src".to_string());
        let files = walk(Path::new(&dir));
        println!("Hashing {} files (jars and configs) under `{dir}` ...", files.len());

        let progress = Progress::new();
        let threaded = hash_mods_tracked(files.clone(), 4, &progress);
        println!("{}", report(&threaded));
        println!("Progress counter: {}/{}", progress.done(), files.len());

        let rayon = hash_mods_rayon(&files);
        let same: Vec<PathBuf> = threaded.iter().map(|(p, _)| p.clone()).collect();
        println!("rayon agrees with the worker pool: {}", rayon == threaded && same == files);
    });
    if result.is_err() {
        println!("Finish `Progress`, `fnv1a`, `walk`, `hash_mods_tracked`, `hash_mods_rayon` and `report` to hash your mods folder.");
    }
}
