**Why now:** a game-adjacent tool often runs where there is no operating system and no heap (a microcontroller, a kernel, a tiny wasm module). The load queue that holds chunk ids waiting to be read must work there too. `#![no_std]` removes everything that needs an allocator and leaves **`core`**: no `Vec`, no `String`, no `HashMap`. A **ring buffer** is the classic answer: a plain array plus two indices that wrap around.

```rust
#![no_std]

pub struct LoadQueue<T, const N: usize> {
    buf: [T; N],     // fixed size, lives inline
    head: usize,     // index of the oldest item
    len: usize,      // how many are stored
}

// the next free slot wraps past the end back to 0:
let tail = (self.head + self.len) % N;
```

- `const N: usize` is a **const generic**: the capacity is part of the type, known at compile time.
- `#![no_std]` links only `core`. Tests may still use `std` via `#[cfg(test)] extern crate std;`.
- A full queue cannot grow, so `enqueue` returns `Result<(), T>` and hands the value back.
- `% N` makes indices wrap: after the last slot comes the first. That is how a small array carries an endless stream.
- `T: Copy + Default` keeps the example simple: the array is filled with defaults and copied out.

## Your turn

In `src/lib.rs`: write `enqueue` and `dequeue` for the `LoadQueue`; `new`, `len`, `is_full` are given.
