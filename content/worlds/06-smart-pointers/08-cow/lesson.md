Many functions *usually* return their input unchanged and only sometimes need to edit it. Returning a `String` always allocates. **`Cow<str>`** ("clone on write") holds either a borrowed `&str` or an owned `String`.

```rust
use std::borrow::Cow;

fn no_commas(s: &str) -> Cow<str> {
    if s.contains(',') {
        Cow::Owned(s.replace(',', ""))   // had to change: allocate
    } else {
        Cow::Borrowed(s)                 // untouched: free
    }
}

let a = no_commas("hello");   // Borrowed
let b = no_commas("a,b");     // Owned("ab")
println!("{a} {b}");          // Cow<str> prints like a string
```

- `Cow` derefs to `&str`, so callers use it like a normal string.
- `matches!(c, Cow::Borrowed(_))` tells you whether anything was allocated.
- `.into_owned()` gives a `String` when you need one.

## Your turn

In `src/lib.rs`, fill in `normalise`: replace every tab with one space. Allocate **only** if the input contains a tab.
