Still in the aside. When the operating system hands your pinger a buffer with a server's bytes, it hands over an address, not a Rust reference. Here you follow such an address yourself, once, to see what the safe types do for you.

**A pointer is an address.** World 06 showed it with `Box`: memory is a long row of numbered cells, and a **pointer** is the number of the cell where a value starts. If this is new, Programming 101 draws it: [Numbered boxes](/quest/basics/07-how-the-computer-holds-things/01-numbered-boxes).

**References are checked pointers.** A `&i32` is such an address too, plus a promise the compiler checked: it points at a live `i32` and the borrow rules hold. A **raw pointer**, `*const i32` to read or `*mut i32` to write, is the bare address with no promise at all.

**Making one is safe, following it is not.** `let p: *mut i32 = &mut players;` just copies an address, which can't hurt anyone. Following it with `*p` reads or writes the memory there, so it must sit inside `unsafe { ... }`.

**What can go wrong.** A raw pointer can be **null** (address 0, pointing at nothing), **dangling** (pointing at memory already freed) or **unaligned** (an `i32` must start at an address divisible by 4). Rust checks none of these. `ptr.is_null()` is a safe check for the first one; the other two are what the `# Safety` docs ask the caller to promise.

```rust
/// # Safety
/// `ptr` must point to a live, writable i32.
pub unsafe fn kick_everyone(ptr: *mut i32) {
    // SAFETY: the caller promised `ptr` is valid for writes.
    unsafe { *ptr = 0; }
}

let mut online = 12;
unsafe { kick_everyone(&mut online) };   // a &mut turns into a *mut by itself
assert_eq!(online, 0);
```

## Your turn

The `# Safety` docs are already written.

- In `add_player`, add 1 to the count behind `ptr`, in an `unsafe` block with a `// SAFETY:` comment.
- In `player_count_or_none`, give `None` for a null pointer and `Some` of the count otherwise.
