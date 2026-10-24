The chunk indexer's core should also run inside a tiny wasm module and on a hobby game console: no operating system, no heap, a few kilobytes of memory. Chunk ids wait there in a load queue that may never grow. Rust can build for such a place, if the crate promises not to need the standard library.

**Without std.** `#![no_std]` at the top of `lib.rs` drops `std`. What is left is **`core`**: `Option`, `Result`, numbers, arrays, slices, iterators, traits. Gone is everything that needs a heap (`Vec`, `String`, `Box`) or an operating system (files, threads, time). `core::cmp::max` is the same function as `std::cmp::max`; `std` simply passes on what `core` has.

**Tests still get std.** The test runner itself needs `std`, so the starter has `#[cfg(test)] extern crate std;`: in test builds only, std is linked back in, and the tests may use `Vec`. The real build stays `no_std`.

**A size in the type.** Without a heap, storage must be sized when the program is compiled. `[T; N]` is an array of exactly `N` items, and `const N: usize` in the angle brackets is a **const generic**: a number the type takes, as `T` is a type it takes. `LoadQueue<u32, 4>` and `LoadQueue<u32, 64>` are two types, each with its array inside.

```rust
#![no_std]

pub struct Hotbar<const N: usize> {
    slots: [u16; N],
    selected: usize,
}

let bar: Hotbar<9> = Hotbar { slots: [0; 9], selected: 0 };
```

**A queue that never moves.** A **ring buffer** is a queue in a fixed array. `head` is the slot of the oldest item and `len` how many are stored, so the next free slot is `(head + len) % N`. The `%` wraps an index past the end back to 0, so items are never shifted; only the two numbers change. A full queue cannot grow, so `enqueue` hands the chunk back in `Err`.

## Your turn

- `enqueue`: give the value back as `Err` when full; otherwise write it into the next free slot, wrapping, and count it.
- `dequeue`: `None` when empty; otherwise take the item at `head`, move `head` on by one, wrapping, and count one fewer.
