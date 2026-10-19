**Why now:** the chunk indexer touches millions of ids, so a function that allocates on every call is slow without looking slow. Before you optimise anything, learn to **count**: a test that installs a counting allocator tells you exactly how many times a function asked for memory.

```rust
use std::alloc::{GlobalAlloc, Layout, System};

struct Counting;

unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        COUNT.with(|c| c.set(c.get() + 1));         // one more allocation
        unsafe { System.alloc(layout) }              // then do the real thing
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) }
    }
}

#[global_allocator]
static ALLOCATOR: Counting = Counting;
```

- `#[global_allocator]` replaces the allocator for the whole program (here: the test binary). The test file `tests/allocations.rs` has it ready. Read it, it is 30 lines.
- Count on **this thread only** (`thread_local!`) so other tests running in parallel do not disturb the number.
- Common reducers: `Vec::with_capacity(n)` when you know the size, `join` instead of `+=` in a loop, passing `&str` instead of `String`.
- Counting beats timing: a count is exact and the same on every machine.

## Your turn

In `src/lib.rs`:

- `join_block_names`: join names with one space, allocating once.
- `dedup_chunk_ids`: drop consecutive duplicates from a sorted slice, allocating once.
