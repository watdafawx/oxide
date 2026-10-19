**Why now:** a lot of networking and OS code is still C. To use it from the pinger you declare the function, hand over a NUL-terminated string, and promise Rust that the call is sound. The MOTD (the server's one-line message) is the string you will pass back and forth.

```rust
use std::ffi::{c_char, CString, CStr};

unsafe extern "C" {
    fn strlen(s: *const c_char) -> usize;
}

let motd = CString::new("A Minecraft Server").unwrap();   // adds the trailing NUL
let len = unsafe { strlen(motd.as_ptr()) };               // 18

let back = unsafe { CStr::from_ptr(motd.as_ptr()) };      // borrow it as a CStr
println!("{}", back.to_string_lossy());
```

- `unsafe extern "C" { fn ...; }` declares C functions. Calling one is `unsafe`: Rust cannot see its body.
- C strings end with a NUL byte and cannot contain one. `CString::new` returns `Err` if your text has an interior NUL.
- `.as_ptr()` borrows the buffer. The `CString` must stay alive until the call returns.
- `CStr::from_ptr` reads a C string you were given. `to_string_lossy` copes with invalid UTF-8.
- `strlen` counts **bytes**, not characters.

## Your turn

In `src/lib.rs`:

- `c_abs`: call C's `abs`.
- `c_motd_len`: C string length of a MOTD; `None` when it holds a NUL.
- `motd_from_c`: copy a C string pointer into a `String`.
