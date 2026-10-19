use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;

use master_counting_allocations::{dedup_chunk_ids, join_block_names};

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
fn join_block_names_allocates_once() {
    let names = ["stone", "dirt", "gravel", "sand", "oak_log", "birch_log", "granite", "diorite", "andesite"];
    let (joined, allocs) = allocations_in(|| join_block_names(&names));
    assert_eq!(joined, "stone dirt gravel sand oak_log birch_log granite diorite andesite");
    assert!(allocs <= 1, "join_block_names allocated {allocs} times, expected at most 1");
}

#[test]
fn dedup_chunk_ids_allocates_once() {
    let input: Vec<i32> = (1..=50).flat_map(|n| [n, n]).collect();
    let (unique, allocs) = allocations_in(|| dedup_chunk_ids(&input));
    assert_eq!(unique, (1..=50).collect::<Vec<i32>>());
    assert!(allocs <= 1, "dedup_chunk_ids allocated {allocs} times, expected at most 1");
}
