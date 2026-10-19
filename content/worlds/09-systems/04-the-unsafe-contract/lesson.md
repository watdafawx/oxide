**Why now:** the pinger talks to the OS and to C through `unsafe`, and the boss crate must stay safe for its callers; this quest is the rulebook.

Rust's guarantees (no use-after-free, no data races) come from the compiler checking your code. **`unsafe` is how you tell the compiler: "I checked this part myself."**

`unsafe` unlocks exactly five things:

1. Dereference a **raw pointer** (`*const T`, `*mut T`).
2. Call an **`unsafe fn`**, including C functions.
3. Access or modify a `static mut`.
4. Implement an **`unsafe trait`** (`Send`, `Sync`).
5. Access fields of a `union`.

Everything else is still checked. The borrow checker still runs inside an unsafe block.

```rust
/// # Safety
/// `ptr` must be non-null, aligned, and point to a live i32.
unsafe fn read(ptr: *const i32) -> i32 {
    // SAFETY: the caller promised in the doc above.
    unsafe { *ptr }
}

fn first(v: &[i32]) -> Option<i32> {
    if v.is_empty() { return None; }
    // SAFETY: v is non-empty, so as_ptr() points at a live i32.
    Some(unsafe { read(v.as_ptr()) })
}
```

- An **`unsafe fn`** says: "the caller must uphold these rules" (written in a `# Safety` doc section).
- An **`unsafe` block** says: "I, the author, have checked those rules are met here" (written in a `// SAFETY:` comment).
- A **safe function** wrapping unsafe code must make misuse impossible. If any safe call can cause undefined behavior, the bug is in the wrapper.

Undefined behavior (UB) is not a crash. It means the compiler may assume it never happens and optimize on that, so the symptom shows up anywhere, later.

Answer the two questions below, then continue.
