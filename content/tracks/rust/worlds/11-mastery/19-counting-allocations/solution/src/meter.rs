//! The allocation meter. Nothing to edit here.
//!
//! `Counting` stands in for the program's allocator: every request for memory goes through it,
//! it adds 1 to this thread's counter, then asks the system allocator for the real memory.
use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;

thread_local! {
    // One counter per thread, so tests running side by side never mix their numbers.
    static COUNT: Cell<usize> = const { Cell::new(0) };
}

struct Counting;

// `unsafe impl`: the compiler cannot check an allocator, so we promise to forward every request
// to `System` unchanged (world 09 has the unsafe rules).
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

/// How many allocations this thread has made since it started.
pub fn allocations() -> usize {
    COUNT.with(Cell::get)
}
