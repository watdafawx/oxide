This quest and the four after it are an aside: the status checker never needs them, and the boss contains no `unsafe` at all. But the `&[u8]` slices you just read and the network connections coming next are built on code the compiler cannot check. This is a short look at that floor, so that "a safe wrapper" means something when you meet one.

**What the compiler checks.** In safe Rust, the compiler proves that every reference points at a live value, that there is never a writer next to a reader, and that every index is checked. That is why none of your programs so far could read freed memory.

**What it cannot check.** Some memory comes from outside the program: the operating system hands over a buffer of received bytes, or a library written in another language returns an address. The compiler never saw that code, so it cannot prove anything about it. Someone has to check it by hand.

**`unsafe`: I checked this.** An `unsafe { ... }` block is how you tell the compiler "I checked this part myself". It unlocks a few extra operations and nothing else. This world uses three: follow a **raw pointer**, call a function marked `unsafe fn`, and call a C function. (Two more exist, changing a global `static mut` and promising a type is thread-safe with `unsafe impl`; you won't need them here.) The borrow checker and the types still apply inside the block.

**Who is on the hook.** An `unsafe fn` says "the caller must keep these rules", and lists them in a `# Safety` section of its doc comment. An `unsafe` block says "I, the author, have checked the rules here", in a `// SAFETY:` comment above it.

```rust
/// # Safety
/// `ptr` must point to a live i32.
unsafe fn read(ptr: *const i32) -> i32 {
    // SAFETY: the caller promised it in the doc above.
    unsafe { *ptr }
}

fn first(counts: &[i32]) -> Option<i32> {
    let item = counts.first()?;        // a reference is always valid
    // SAFETY: `item` points at an element of a live slice.
    Some(unsafe { read(item) })
}
```

**A safe wrapper.** `first` is a normal function with an `unsafe` block inside. Its promise is that no call to it, with any slice, can break the rules. If one could, the bug is in `first`, not in the caller. `Vec`, `String` and `TcpStream` are all safe wrappers like this.

**Undefined behaviour.** Break an unsafe rule and you get **undefined behaviour** (UB): not a clean crash, but the compiler optimizing on a promise that was false. The symptom can show up anywhere, much later, which is why the unsafe parts stay tiny.

## Your turn

Answer the questions, then start the next quest.
