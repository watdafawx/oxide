use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;

use master_fast_paths::probe::{Counter, Name, clones};
use master_fast_paths::{count_long_words, longest, squares, word_count};

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

fn allocations_in<R>(f: impl FnOnce() -> R) -> (R, usize) {
    let before = COUNT.with(Cell::get);
    let result = f();
    (result, COUNT.with(Cell::get) - before)
}

#[test]
fn word_count_uses_one_operation_per_word() {
    let text = "a b a c a b";
    let mut counter = Counter::new();
    word_count(text, &mut counter);
    let ops = counter.ops();
    assert_eq!(counter.get("a"), Some(3));
    assert_eq!(counter.get("b"), Some(2));
    assert_eq!(counter.get("c"), Some(1));
    assert!(ops <= 6, "{ops} Counter operations for 6 words, expected at most 6");
}

#[test]
fn longest_clones_at_most_once() {
    let names: Vec<Name> = ["ann", "bob1", "carol", "dorian", "eleanora", "frank"]
        .iter()
        .map(|s| Name(s.to_string()))
        .collect();
    let before = clones();
    let best = longest(&names);
    let used = clones() - before;
    assert_eq!(best, Some(Name("eleanora".to_string())));
    assert!(used <= 1, "{used} clones, expected at most 1");
    assert_eq!(longest(&[]), None);
}

#[test]
fn squares_allocate_once() {
    let (out, allocs) = allocations_in(|| squares(100));
    assert_eq!(out.len(), 100);
    assert_eq!(out[9], 81);
    assert!(allocs <= 1, "squares allocated {allocs} times, expected at most 1");
}

#[test]
fn count_long_words_does_not_allocate() {
    let text = "the quick brown fox jumps over the lazy dog";
    let (n, allocs) = allocations_in(|| count_long_words(text, 5));
    assert_eq!(n, 3);
    assert_eq!(allocs, 0, "count_long_words allocated {allocs} times, expected 0");
}
