use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;

use mastery_one_buffer::tally_blocks;

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

/// A scan log of 300 block names drawn from 5, in mixed case, with punctuation.
fn long_log() -> String {
    let names = ["Stone", "stone,", "DIRT", "dirt!", "Gravel", "sand", "SAND;", "granite"];
    (0..300).map(|i| names[i % names.len()]).collect::<Vec<_>>().join(" ")
}

#[test]
fn tally_ignores_case_and_punctuation() {
    let tally = tally_blocks("Stone, stone! STONE and Stone-dirt");
    assert_eq!(tally["stone"], 4);
    assert_eq!(tally["and"], 1);
    assert_eq!(tally["dirt"], 1);
    assert_eq!(tally.len(), 3);
}

#[test]
fn allocates_for_new_names_only() {
    let log = long_log();
    let (tally, allocs) = allocations_in(|| tally_blocks(&log));
    assert_eq!(tally.len(), 5);
    assert_eq!(tally.values().sum::<usize>(), 300);
    assert!(
        allocs <= 20,
        "300 block names, 5 of them different: {allocs} allocations, expected at most 20"
    );
}
