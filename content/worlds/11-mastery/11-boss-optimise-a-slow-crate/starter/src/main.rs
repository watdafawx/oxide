use master_slow_index::probe::ops;
use master_slow_index::*;
use std::panic;
use std::time::Instant;

const TEXT: &str = "The quick brown fox jumps over the lazy dog.
The dog barks; the fox runs away!
A quick fox is a happy fox.
Dogs and foxes: the oldest rivalry in the book.";

fn main() {
    println!("== Text index ==");
    panic::set_hook(Box::new(|_| {}));
    let result = panic::catch_unwind(|| {
        let big: String = (0..20_000).map(|_| format!("{TEXT}\n")).collect();

        let (o0, t0) = (ops(), Instant::now());
        let index = Index::build(&big);
        println!("build: {} lines indexed, {} ops, {:?}", big.lines().count(), ops() - o0, t0.elapsed());

        let (o0, t0) = (ops(), Instant::now());
        let lines = index.search("fox");
        println!("search \"fox\": {} lines (first {:?}), {} ops, {:?}", lines.len(), &lines[..lines.len().min(5)], ops() - o0, t0.elapsed());

        let (o0, t0) = (ops(), Instant::now());
        let top = index.top_words(5);
        println!("top_words(5): {} ops, {:?}", ops() - o0, t0.elapsed());
        for (word, count) in top {
            println!("  {word}: {count}");
        }
    });
    if result.is_err() {
        println!("The index panicked: check `build`, `search` and `top_words`.");
    }
}
