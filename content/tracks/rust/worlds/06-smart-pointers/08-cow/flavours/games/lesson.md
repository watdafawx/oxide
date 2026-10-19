**Why now:** your machine graph handles thousands of item names, and nearly all of them are already written as `mod:item` (`minecraft:iron_ingot`). A few arrive bare (`cogwheel`) and need the mod id put in front. Returning a `String` for every name allocates for nothing. **`Cow<str>`** ("clone on write") holds either a borrowed `&str` or an owned `String`, so only the bare names allocate.

```rust
use std::borrow::Cow;

fn with_ns(name: &str) -> Cow<str> {
    if name.contains(':') {
        Cow::Borrowed(name)                       // fine as it is: free
    } else {
        Cow::Owned(format!("minecraft:{name}"))   // had to change: allocate
    }
}

let a = with_ns("minecraft:stone");  // Borrowed
let b = with_ns("stone");            // Owned("minecraft:stone")
println!("{a} {b}");                 // Cow<str> prints like a string
```

- `Cow` derefs to `&str`, so callers use it like a normal string.
- `matches!(c, Cow::Borrowed(_))` tells you whether anything was allocated.
- `.into_owned()` gives a `String` when you need one.

## Your turn

In `src/lib.rs`, fill in `qualified_name(name, mod_id)`: if `name` already contains a `:` return it untouched, otherwise return `"<mod_id>:<name>"`. Allocate **only** in the second case.
