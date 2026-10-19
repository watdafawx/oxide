use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;

use master_slow_index::Index;
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
const WORDS_PER_LINE: usize = 4;

/// A deterministic document: `lines` lines of `WORDS_PER_LINE` words drawn from `w0..w299`.
fn document(lines: usize) -> String {
    let mut state: u64 = 12345;
    let mut out = String::new();
    for _ in 0..lines {
        for k in 0..WORDS_PER_LINE {
            state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            if k > 0 {
                out.push(' ');
            }
            out.push_str(&format!("w{}", (state >> 33) as usize % VOCAB));
        }
        out.push('\n');
    }
    out
}

/// Independent line lookup for checking answers.
fn lines_with(doc: &str, word: &str) -> Vec<usize> {
    doc.lines()
        .enumerate()
        .filter(|(_, l)| l.split(' ').any(|t| t == word))
        .map(|(i, _)| i + 1)
        .collect()
}

#[test]
fn search_finds_lines_once_each() {
    let index = Index::build("go go go\nstop\ngo\n");
    assert_eq!(index.search("go"), vec![1, 3]);
    assert_eq!(index.search("stop"), vec![2]);
    assert_eq!(index.search("nothing"), Vec::<usize>::new());
}

#[test]
fn words_ignore_case_and_punctuation() {
    let index = Index::build("Hello, world!\n\nHELLO again; world-wide\n");
    assert_eq!(index.search("hello"), vec![1, 3]);
    assert_eq!(index.search("WORLD"), vec![1, 3]);
    assert_eq!(index.search("wide"), vec![3]);
    assert_eq!(index.search(""), Vec::<usize>::new());
}

#[test]
fn top_words_orders_by_count_then_alphabet() {
    let index = Index::build("b a b\nc a b\nd\n");
    let top = index.top_words(3);
    assert_eq!(top, vec![("b".to_string(), 3), ("a".to_string(), 2), ("c".to_string(), 1)]);
    assert_eq!(index.top_words(100).len(), 4, "asking for more than exist returns all of them");
}

#[test]
fn search_ops_are_capped() {
    let doc = document(2_000);
    let index = Index::build(&doc);
    let (found, _, used) = measure(|| index.search("w7"));
    assert_eq!(found, lines_with(&doc, "w7"));
    assert!(used <= 20, "one search took {used} comparison ops, expected at most 20");
}

#[test]
fn build_ops_are_capped() {
    let doc = document(2_000);
    let tokens = 2_000 * WORDS_PER_LINE;
    let (_, _, used) = measure(|| Index::build(&doc));
    assert!(
        used <= tokens * 2,
        "build took {used} comparison ops for {tokens} words, expected at most {}",
        tokens * 2
    );
}

#[test]
fn build_allocations_are_capped() {
    let doc = document(2_000);
    let (_, allocs, _) = measure(|| Index::build(&doc));
    assert!(allocs <= 3_000, "build made {allocs} allocations, expected at most 3000");
}

#[test]
fn top_words_allocations_are_capped() {
    let index = Index::build(&document(2_000));
    let (top, allocs, _) = measure(|| index.top_words(10));
    assert_eq!(top.len(), 10);
    assert!(top.windows(2).all(|w| w[0].1 >= w[1].1), "most frequent first");
    assert!(allocs <= 14, "top_words(10) made {allocs} allocations, expected at most 14");
}

#[test]
fn big_document_stays_under_the_op_budget() {
    let lines = 50_000;
    let doc = document(lines);
    let tokens = lines * WORDS_PER_LINE;
    let budget = tokens * 2;
    let ((index, found, top), _, used) = measure(|| {
        let index = Index::build(&doc);
        let found: Vec<Vec<usize>> = (0..20).map(|k| index.search(&format!("w{k}"))).collect();
        let top = index.top_words(10);
        (index, found, top)
    });
    assert!(used <= budget, "{used} comparison ops for {lines} lines, budget is {budget}");
    for (k, got) in found.iter().enumerate() {
        assert_eq!(*got, lines_with(&doc, &format!("w{k}")), "search w{k}");
    }
    assert_eq!(top.len(), 10);
    let total: usize = top.iter().map(|(_, c)| c).sum();
    assert!(total > 0 && index.search("w0").len() > 100);
}
