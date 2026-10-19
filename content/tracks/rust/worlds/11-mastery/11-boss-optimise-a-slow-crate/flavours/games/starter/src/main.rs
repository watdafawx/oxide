use master_slow_index::probe::ops;
use master_slow_index::*;
use std::panic;
use std::time::Instant;

const LOG: &str = "stone dirt grass oak_log stone
Stone Dirt; gravel, stone, coal_ore!
grass grass flower dirt
Stone and diamond_ore: deep, dark, stone.";

fn main() {
    println!("== Chunk index ==");
    panic::set_hook(Box::new(|_| {}));
    let result = panic::catch_unwind(|| {
        let big: String = (0..20_000).map(|_| format!("{LOG}\n")).collect();

        let (o0, t0) = (ops(), Instant::now());
        let index = ChunkIndex::build(&big);
        println!("build: {} chunks indexed, {} ops, {:?}", big.lines().count(), ops() - o0, t0.elapsed());

        let (o0, t0) = (ops(), Instant::now());
        let chunks = index.chunks_with("stone");
        println!("chunks_with \"stone\": {} chunks (first {:?}), {} ops, {:?}", chunks.len(), &chunks[..chunks.len().min(5)], ops() - o0, t0.elapsed());

        let (o0, t0) = (ops(), Instant::now());
        let top = index.top_blocks(5);
        println!("top_blocks(5): {} ops, {:?}", ops() - o0, t0.elapsed());
        for (block, count) in top {
            println!("  {block}: {count}");
        }
    });
    if result.is_err() {
        println!("The index panicked: check `build`, `chunks_with` and `top_blocks`.");
    }
}
