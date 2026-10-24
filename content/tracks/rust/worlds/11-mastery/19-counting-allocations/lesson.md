Before making the text index faster, you want a number for "how much work does this do?" that is the same on every computer. Seconds are not: they change with the machine and with whatever else is running. The number of **allocations**, the times the program asks for heap memory, is exact. This quest builds the meter that reads it.

**What an allocation is.** World 02 showed that a `String`'s letters and a `Vec`'s items live on the heap, in memory asked for while the program runs. Each request goes to the **allocator**, the part of the program that hands out heap memory, and each one costs time: finding a free block, and later giving it back.

**Stand in front of it.** `#[global_allocator]` on a `static` makes that value the allocator for the whole program. The starter's `meter.rs` puts a `Counting` allocator there: it adds 1 to a counter, then forwards the request to `System`, the normal allocator. Its `unsafe impl` is the promise from world 09: the compiler cannot check an allocator, so the code vows to pass every request on unchanged.

**A counter per thread.** Tests run side by side on several threads, and each allocates on its own. `thread_local!` gives every thread its own `COUNT`, so one test never sees another's allocations. It is a `Cell<usize>` (world 06), which can be changed through a shared reference; `allocations()` reads it.

**Read, run, read.** A meter gives a running total, so measuring one piece of code means reading it before and after and taking the difference. `allocations_in` takes the code as a closure, `impl FnOnce() -> R`: a closure that may be called **once**, which is all a measurement needs, and that returns some `R`, handed back with the count.

```rust
let before = steps_taken();
walk_to_the_shop();
let walked = steps_taken() - before;
```

## Your turn

- In `allocations_in`, read `allocations()`, run `f` and keep its result, read `allocations()` again.
- Return the result together with the difference of the two readings.
