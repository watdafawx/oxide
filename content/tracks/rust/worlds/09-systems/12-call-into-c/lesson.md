The aside's last two quests answer one question: what is under `TcpStream`? The operating system's network functions are written in C, and Rust's standard library calls them for you. Here you make such a call yourself, on a much smaller function.

**What C is.** **C** is a programming language from 1972, and most operating systems are written in it: Linux, the core of macOS, and much of Windows. Their public doors, "open a file", "open a network connection", are C functions. Any language that wants to use the system has to be able to call C.

**The C library is already there.** Every program on your computer, Rust ones included, is linked with the **C standard library**, a set of small C functions like `abs` (distance from 0) and `strlen` (length of a text). So you can call them with no crate at all.

**Declare it.** Rust has to be told a C function exists and what it takes. An `unsafe extern "C" { ... }` block lists declarations: a name and types, no body. Calling code in another language this way is called **FFI**, the foreign function interface. `c_int` from `std::ffi` is C's `int` type, which is an `i32` on every common computer.

**Every call is unsafe.** Rust can't see a C function's body, so it can't check that the declaration is true or that the function keeps any rules. So each call sits in an `unsafe` block, and a safe Rust function around it is where you promise it's fine.

```rust
use std::ffi::c_longlong;

unsafe extern "C" {
    fn llabs(x: c_longlong) -> c_longlong;   // abs for C's 64-bit integers
}

pub fn distance(x: i64) -> i64 {
    // SAFETY: llabs has no rules beyond taking a valid number.
    unsafe { llabs(x) }   // c_longlong is i64
}
```

**For the curious.** `std::net::TcpStream::connect` ends in exactly such a call. On Linux and macOS it is the C function `connect`; on Windows, the same function from Winsock, Windows' C network library. std declares it, checks everything, and hands you a safe method.

## Your turn

- In `c_abs`, call the C function `abs` with `n` and give back what it returns.
- Put the call in an `unsafe` block with a `// SAFETY:` comment.
