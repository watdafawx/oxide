use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;

use master_slow_index::ChunkIndex;
use master_slow_index::probe::ops;

thread_local! {
    static COUNT: Cell<usize> = const { Cell::new(0) };
}

/// Forwards to the system allocator and counts every allocation on the current thread.
struct Counting;

unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        COUNT.with(|c| c.set(c.get() + 1));
        unsafe { System.alloc(layout) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) }
    }
}

#[global_allocator]
static ALLOCATOR: Counting = Counting;

/// Runs `f`, returns its result, the allocations it made and the comparison ops it did.
fn measure<R>(f: impl FnOnce() -> R) -> (R, usize, usize) {
    let (a0, o0) = (COUNT.with(Cell::get), ops());
    let result = f();
    (result, COUNT.with(Cell::get) - a0, ops() - o0)
}

const VOCAB: usize = 300;
const BLOCKS_PER_CHUNK: usize = 4;

/// A deterministic world: `chunks` lines of `BLOCKS_PER_CHUNK` blocks drawn from `b0..b299`.
fn world(chunks: usize) -> String {
    let mut state: u64 = 12345;
    let mut out = String::new();
    for _ in 0..chunks {
        for k in 0..BLOCKS_PER_CHUNK {
            state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            if k > 0 {
                out.push(' ');
            }
            out.push_str(&format!("b{}", (state >> 33) as usize % VOCAB));
        }
        out.push('\n');
    }
    out
}

/// Independent chunk lookup for checking answers.
fn chunks_containing(log: &str, block: &str) -> Vec<usize> {
    log.lines()
        .enumerate()
        .filter(|(_, l)| l.split(' ').any(|t| t == block))
        .map(|(i, _)| i + 1)
        .collect()
}

#[test]
fn chunks_with_finds_chunks_once_each() {
    let index = ChunkIndex::build("stone stone stone\ndirt\nstone\n");
    assert_eq!(index.chunks_with("stone"), vec![1, 3]);
    assert_eq!(index.chunks_with("dirt"), vec![2]);
    assert_eq!(index.chunks_with("bedrock"), Vec::<usize>::new());
}

#[test]
fn block_names_ignore_case_and_punctuation() {
    let index = ChunkIndex::build("Oak, birch!\n\nOAK again; dark-oak\n");
    assert_eq!(index.chunks_with("oak"), vec![1, 3]);
    assert_eq!(index.chunks_with("BIRCH"), vec![1]);
    assert_eq!(index.chunks_with("dark"), vec![3]);
    assert_eq!(index.chunks_with(""), Vec::<usize>::new());
}

#[test]
fn top_blocks_order_by_count_then_alphabet() {
    let index = ChunkIndex::build("b a b\nc a b\nd\n");
    let top = index.top_blocks(3);
    assert_eq!(top, vec![("b".to_string(), 3), ("a".to_string(), 2), ("c".to_string(), 1)]);
    assert_eq!(index.top_blocks(100).len(), 4, "asking for more than exist returns all of them");
}

#[test]
fn chunks_with_ops_are_capped() {
    let log = world(2_000);
    let index = ChunkIndex::build(&log);
    let (found, _, used) = measure(|| index.chunks_with("b7"));
    assert_eq!(found, chunks_containing(&log, "b7"));
    assert!(used <= 20, "one lookup took {used} comparison ops, expected at most 20");
}

#[test]
fn build_ops_are_capped() {
    let log = world(2_000);
    let tokens = 2_000 * BLOCKS_PER_CHUNK;
    let (_, _, used) = measure(|| ChunkIndex::build(&log));
    assert!(
        used <= tokens * 2,
        "build took {used} comparison ops for {tokens} block names, expected at most {}",
        tokens * 2
    );
}

#[test]
fn build_allocations_are_capped() {
    let log = world(2_000);
    let (_, allocs, _) = measure(|| ChunkIndex::build(&log));
    assert!(allocs <= 3_000, "build made {allocs} allocations, expected at most 3000");
}

#[test]
fn top_blocks_allocations_are_capped() {
    let index = ChunkIndex::build(&world(2_000));
    let (top, allocs, _) = measure(|| index.top_blocks(10));
    assert_eq!(top.len(), 10);
    assert!(top.windows(2).all(|w| w[0].1 >= w[1].1), "most frequent first");
    assert!(allocs <= 14, "top_blocks(10) made {allocs} allocations, expected at most 14");
}

#[test]
fn big_world_stays_under_the_op_budget() {
    let chunks = 50_000;
    let log = world(chunks);
    let tokens = chunks * BLOCKS_PER_CHUNK;
    let budget = tokens * 2;
    let ((index, found, top), _, used) = measure(|| {
        let index = ChunkIndex::build(&log);
        let found: Vec<Vec<usize>> = (0..20).map(|k| index.chunks_with(&format!("b{k}"))).collect();
        let top = index.top_blocks(10);
        (index, found, top)
    });
    assert!(used <= budget, "{used} comparison ops for {chunks} chunks, budget is {budget}");
    for (k, got) in found.iter().enumerate() {
        assert_eq!(*got, chunks_containing(&log, &format!("b{k}")), "chunks_with b{k}");
    }
    assert_eq!(top.len(), 10);
    let total: usize = top.iter().map(|(_, c)| c).sum();
    assert!(total > 0 && index.chunks_with("b0").len() > 100);
}
