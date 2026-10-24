The same text tools should run on a small sensor board: no operating system, no heap, a few kilobytes of memory. Its keypresses arrive faster than they are handled, so they wait in a queue that may never grow. Rust can build for such a place, if the crate promises not to need the standard library.

**Without std.** `#![no_std]` at the top of `lib.rs` drops `std`. What is left is **`core`**: `Option`, `Result`, numbers, arrays, slices, iterators, traits. Gone is everything that needs a heap (`Vec`, `String`, `Box`) or an operating system (files, threads, time). `core::cmp::max` is the same function as `std::cmp::max`; `std` simply passes on what `core` has.

**Tests still get std.** The test runner itself needs `std`, so the starter has `#[cfg(test)] extern crate std;`: in test builds only, std is linked back in, and the tests may use `Vec`. The real build stays `no_std`.

**A size in the type.** Without a heap, storage must be sized when the program is compiled. `[T; N]` is an array of exactly `N` items, and `const N: usize` in the angle brackets is a **const generic**: a number the type takes, as `T` is a type it takes. `RingBuffer<u8, 4>` and `RingBuffer<u8, 64>` are two types, each with its array inside.

```rust
#![no_std]

pub struct Mailbox<const N: usize> {
    letters: [u8; N],
    count: usize,
}

let small: Mailbox<4> = Mailbox { letters: [0; 4], count: 0 };
```

**A queue that never moves.** A **ring buffer** is a queue in a fixed array. `head` is the slot of the oldest item and `len` how many are stored, so the next free slot is `(head + len) % N`. The `%` wraps an index past the end back to 0, so items are never shifted; only the two numbers change. A full buffer cannot grow, so `push` hands the value back in `Err`.

## Your turn

- `push`: give the value back as `Err` when full; otherwise write it into the next free slot, wrapping, and count it.
- `pop`: `None` when empty; otherwise take the item at `head`, move `head` on by one, wrapping, and count one fewer.
