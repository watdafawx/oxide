The todo list now has a `planner` room that works out how long tasks take. The time per task is set once, at the top of the crate, and the planner needs to read it. Then the rest of the list wants to call the planner without typing long paths.

**Each room has its own names.** Inside `planner`, the bare name `MINUTES_PER_TASK` is not found, because it lives in the room above. Modules keep their names apart on purpose: two rooms can each have a function called `parse` without clashing, and the path says which one you mean.

**super goes up.** `super::MINUTES_PER_TASK` means "the one in the room one level up". A child room may use its parent's private items: privacy only stops code from looking *into* a room, never out of it.

**Other starting points.** `self::` starts in the current room, and `crate::` starts at the top of the crate, the `lib.rs` file. So `crate::planner::minutes_for` names the same function from anywhere in the crate.

**use makes a shortcut.** `use planner::minutes_for;` brings that name into the current room, so you can write `minutes_for(4)` instead of the whole path. You have used it already: `use std::fmt;`, `use anyhow::Context;`, and `use super::*;` at the top of every test module, where `*` means everything in the room above.

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

- In `planner::minutes_for`, multiply `tasks` by `MINUTES_PER_TASK`, reaching it with `super::`.
- Above `describe`, write `use planner::minutes_for;`.
- In `describe`, return `"<tasks> tasks take <minutes> minutes"`, calling `minutes_for` by its short name.
