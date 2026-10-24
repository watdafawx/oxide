use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;

use mastery_reserve_once::{join_words, squares};

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

#[test]
fn squares_allocate_once() {
    let (out, allocs) = allocations_in(|| squares(100));
    assert_eq!(out.len(), 100);
    assert_eq!(out[9], 81);
    assert!(allocs <= 1, "squares(100) allocated {allocs} times, expected at most 1");
}

#[test]
fn join_words_allocates_once() {
    let words = ["the", "quick", "brown", "fox", "jumps", "over", "the", "lazy", "dog"];
    let (joined, allocs) = allocations_in(|| join_words(&words));
    assert_eq!(joined, "the quick brown fox jumps over the lazy dog");
    assert!(allocs <= 1, "join_words allocated {allocs} times, expected at most 1");
}
