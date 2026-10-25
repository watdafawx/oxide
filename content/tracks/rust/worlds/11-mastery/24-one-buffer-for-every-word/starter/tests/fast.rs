use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;

use mastery_one_buffer::count_words;

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

/// Runs `f` and returns its result with the number of allocations it made.
fn allocations_in<R>(f: impl FnOnce() -> R) -> (R, usize) {
    let before = COUNT.with(Cell::get);
    let result = f();
    let after = COUNT.with(Cell::get);
    (result, after - before)
}

/// 300 words drawn from 5, in mixed case, with punctuation.
fn long_text() -> String {
    let words = ["Rust", "rust,", "FAST", "fast!", "Index", "word", "WORD;", "safe"];
    (0..300).map(|i| words[i % words.len()]).collect::<Vec<_>>().join(" ")
}

#[test]
fn counts_ignore_case_and_punctuation() {
    let counts = count_words("Rust, rust! RUST and Rust-fast");
    assert_eq!(counts["rust"], 4);
    assert_eq!(counts["and"], 1);
    assert_eq!(counts["fast"], 1);
    assert_eq!(counts.len(), 3);
}

#[test]
fn allocates_for_new_words_only() {
    let text = long_text();
    let (counts, allocs) = allocations_in(|| count_words(&text));
    assert_eq!(counts.len(), 5);
    assert_eq!(counts.values().sum::<usize>(), 300);
    assert!(
        allocs <= 20,
        "300 words, 5 of them different: {allocs} allocations, expected at most 20"
    );
}
