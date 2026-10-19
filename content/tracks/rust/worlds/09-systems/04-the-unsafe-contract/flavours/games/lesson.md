**Why now:** the next quests poke at memory the way a network or C library does: raw pointers into a packet buffer, a call into C. The compiler can no longer check those for you, so you need to know exactly what `unsafe` promises.

`unsafe` is not "turn off the checks". It unlocks **five** extra operations and nothing else: dereference a raw pointer, call an `unsafe fn` (including FFI), access a mutable `static`, implement an unsafe trait, and access union fields. Borrowck and types still apply to everything in the block.

```rust
/// # Safety
/// `ptr` must point to a live, aligned i32.
pub unsafe fn read_count(ptr: *const i32) -> i32 {
    // SAFETY: the caller guarantees `ptr` is valid.
    unsafe { *ptr }
}

pub fn first_count(counts: &[i32]) -> Option<i32> {
    let ptr = counts.first()?;                 // a reference is always valid
    // SAFETY: `ptr` comes from a live slice element.
    Some(unsafe { read_count(ptr) })
}
```

- `unsafe fn` says "the caller must uphold these rules". Write them in a `# Safety` section.
- A **safe** function with an `unsafe` block inside says "I checked, you cannot misuse this". That promise is yours, not the caller's.
- Write a `// SAFETY:` comment at every unsafe block explaining why it holds.
- Undefined behavior (null deref, dangling pointer, data race) can break code far away from the bug.
