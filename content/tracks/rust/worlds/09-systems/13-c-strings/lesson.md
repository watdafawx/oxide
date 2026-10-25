Network functions take text too: a host name like `"localhost"` goes into C on its way to the operating system. But C stores text differently from Rust, and handing a Rust `&str` straight to C would let it read past the end. This quest crosses that border in both directions.

**How C stores text.** A C string is just bytes in a row, ending with a zero byte, written `\0` and called **NUL**. There is no length anywhere: `strlen` finds the end by walking until it meets the NUL. `c_char` is C's type for one of those bytes.

**Rust strings know their length.** A Rust string stores its length next to its bytes, and has no NUL at the end, so C would run off the end into whatever memory follows. A Rust string may even contain a zero byte in the middle, `"ab\0cd"`, which C would read as just `"ab"`.

**CString: a copy C can read.** `CString::new(text)` makes an owned copy with a NUL added at the end. It fails with an `Err` if the text has a NUL inside, since C could never see past it. `c.as_ptr()` gives C the address; the `CString` must stay alive until C is done, so keep it in a variable.

**CStr: borrowing C's text.** Going the other way, `CStr::from_ptr(ptr)` reads a C string up to its NUL. It is `unsafe`: you promise the pointer really leads to a NUL-terminated text. `to_string_lossy()` then gives a `Cow<str>` (world 06), patching any bytes that are not valid UTF-8, and `.into_owned()` turns it into a `String`.

```rust
let host = CString::new("localhost").unwrap();
assert_eq!(host.as_bytes_with_nul().len(), 10);   // 9 letters and the NUL

// SAFETY: `host` ends in a NUL and is still alive here.
let back = unsafe { CStr::from_ptr(host.as_ptr()) };
assert_eq!(back.to_str(), Ok("localhost"));
```

**End of the aside.** That is the floor under std: raw pointers, slices from parts, C calls and C strings, each wrapped once in a safe type. From the next quest on, you use those safe types to talk to a real server.

## Your turn

- In `c_strlen`, make a `CString` of `s` (give `None` if that fails) and let `strlen` count it.
- In `from_c`, borrow the C string behind `ptr` as a `CStr` and turn it into an owned `String`.
