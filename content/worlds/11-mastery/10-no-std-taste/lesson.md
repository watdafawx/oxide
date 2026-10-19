**Why now:** the chunk indexer's queue of pending chunks must live in a fixed array with no heap, which is the `no_std` ring buffer you build here.

`#![no_std]` drops the standard library. No `Vec`, no `String`, no heap, no OS. What is left is **`core`**: `Option`, `Result`, slices, arrays, iterators, `Copy`. That is all a microcontroller or a kernel has, and a wasm module is happiest with it too.

- `core::` replaces `std::` for what exists in both (`core::cmp`, `core::fmt`).
- Storage has to be known at compile time. A **const generic** `[T; N]` fixes the size in the type.
- Tests still want `std`. Add `#[cfg(test)] extern crate std;` so the test build, and only the test build, can use `Vec` and `assert_eq!` with `std`.

```rust
#![no_std]

pub struct ChunkQueue<const N: usize> {
    items: [u8; N],
    len: usize,
}
```

A **ring buffer** is a queue in a fixed array. `head` marks the oldest item, new items go at `(head + len) % N`, and both indices wrap around the end, so nothing ever moves.

## Your turn

In `src/lib.rs`, fill in the `RingBuffer` methods:

- `len` and `is_full`.
- `push`: `Err(value)` when full, otherwise store it at the tail.
- `pop`: oldest item first, `None` when empty.
