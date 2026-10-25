The loader now has a `resolver` room that works out how much disk the libraries need. The size per library is set once, at the top of the crate, and the resolver needs to read it. Then the rest of the launcher wants to call the resolver without typing long paths.

**Each room has its own names.** Inside `resolver`, the bare name `MB_PER_LIBRARY` is not found, because it lives in the room above. Modules keep their names apart on purpose: two rooms can each have a function called `parse` without clashing, and the path says which one you mean.

**super goes up.** `super::MB_PER_LIBRARY` means "the one in the room one level up". A child room may use its parent's private items: privacy only stops code from looking *into* a room, never out of it.

**Other starting points.** `self::` starts in the current room, and `crate::` starts at the top of the crate, the `lib.rs` file. So `crate::resolver::library_mb` names the same function from anywhere in the crate.

**use makes a shortcut.** `use resolver::library_mb;` brings that name into the current room, so you can write `library_mb(4)` instead of the whole path. You have used it already: `use std::fmt;`, `use anyhow::Context;`, and `use super::*;` at the top of every test module, where `*` means everything in the room above.

```rust
const BREAK_MINUTES: u32 = 10;

mod clock {
    pub fn with_break(minutes: u32) -> u32 {
        minutes + super::BREAK_MINUTES
    }
}

use clock::with_break;
```

## Your turn

- In `resolver::library_mb`, multiply `libraries` by `MB_PER_LIBRARY`, reaching it with `super::`.
- Above `describe`, write `use resolver::library_mb;`.
- In `describe`, return `"<libraries> libraries need <mb> MB"`, calling `library_mb` by its short name.
