**Why now:** the pinger and every OS API underneath it speak C, so you need to call C functions and turn C strings into `String` safely.

Almost every OS API speaks **C**. Rust calls it through **FFI** (foreign function interface). No extra crate needed: the C runtime is already linked into every Rust program.

```rust
use std::ffi::{c_char, c_int, CStr, CString};

unsafe extern "C" {
    fn strlen(s: *const c_char) -> usize;
    fn abs(x: c_int) -> c_int;
}

fn len_of(s: &str) -> Option<usize> {
    let c = CString::new(s).ok()?;   // adds the trailing NUL
    // SAFETY: c is a valid NUL-terminated string that outlives the call.
    Some(unsafe { strlen(c.as_ptr()) })
}
```

- `unsafe extern "C" { ... }` declares C functions. Rust can't check a declaration, so every call is **unsafe**.
- C strings are bytes ending in a **NUL** (`\0`) with no length. Rust strings have a length and may contain NUL. **`CString`** (owned) and **`CStr`** (borrowed) convert between them.
- `CString::new` fails if the text has an interior NUL, so wrappers must return `Option` or `Result`.
- `CStr::from_ptr(p)` reads up to the NUL. It is unsafe: `p` must point to a valid NUL-terminated string.
- `CStr::from_ptr(p).to_string_lossy()` gives a `Cow<str>` (bad UTF-8 is patched up); `.into_owned()` turns the `Cow` into an owned `String`.
- Use the C types: `c_char`, `c_int`, not `i8` / `i32` guesses.

## Your turn

The `extern` block is already in `src/lib.rs`. Write the safe layer:

- `c_abs`: call C `abs`.
- `c_strlen`: `CString` + `strlen`, `None` on interior NUL.
- `from_c`: an `unsafe fn` that turns a `*const c_char` into a `String`.
