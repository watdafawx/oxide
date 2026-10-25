use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;

use mastery_borrow_dont_copy::count_long_words;

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
fn count_long_words_does_not_allocate() {
    let text = "the quick brown fox jumps over the lazy dog";
    let (n, allocs) = allocations_in(|| count_long_words(text, 5));
    assert_eq!(n, 3);
    assert_eq!(allocs, 0, "count_long_words allocated {allocs} times, expected 0");
}
