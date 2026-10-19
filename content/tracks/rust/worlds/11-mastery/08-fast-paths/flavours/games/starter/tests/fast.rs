use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;

use master_fast_paths::probe::{Counter, Name, clones};
use master_fast_paths::{chunk_areas, count_blocks, count_long_names, longest_name};

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
fn count_blocks_uses_one_operation_per_block() {
    let log = "stone dirt stone sand stone dirt";
    let mut counter = Counter::new();
    count_blocks(log, &mut counter);
    let ops = counter.ops();
    assert_eq!(counter.get("stone"), Some(3));
    assert_eq!(counter.get("dirt"), Some(2));
    assert_eq!(counter.get("sand"), Some(1));
    assert!(ops <= 6, "{ops} Counter operations for 6 blocks, expected at most 6");
}

#[test]
fn longest_name_clones_at_most_once() {
    let names: Vec<Name> = ["oak", "birch", "spruce", "jungle", "dark_oak", "acacia"]
        .iter()
        .map(|s| Name(s.to_string()))
        .collect();
    let before = clones();
    let best = longest_name(&names);
    let used = clones() - before;
    assert_eq!(best, Some(Name("dark_oak".to_string())));
    assert!(used <= 1, "{used} clones, expected at most 1");
    assert_eq!(longest_name(&[]), None);
}

#[test]
fn chunk_areas_allocate_once() {
    let (out, allocs) = allocations_in(|| chunk_areas(100));
    assert_eq!(out.len(), 100);
    assert_eq!(out[9], 81);
    assert!(allocs <= 1, "chunk_areas allocated {allocs} times, expected at most 1");
}

#[test]
fn count_long_names_does_not_allocate() {
    let log = "stone dirt gravel sand oak_log birch_log granite";
    let (n, allocs) = allocations_in(|| count_long_names(log, 7));
    assert_eq!(n, 3);
    assert_eq!(allocs, 0, "count_long_names allocated {allocs} times, expected 0");
}
