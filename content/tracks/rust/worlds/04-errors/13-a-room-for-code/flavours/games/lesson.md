The loader is growing: reading the manifest, resolving libraries, checking disk space. If every function sits side by side in one long file, anything can call anything, and a change in one corner breaks another. Rust lets you split code into rooms, and decide which doors are open.

**A room for code.** `mod pack { ... }` makes a **module**: a named room that holds functions, types and constants. Code outside reaches what is inside by its path, `pack::slots_left`, with the same `::` as `Type::new`.

**Private by default.** Everything in a module starts **private**: only code inside that room, and rooms nested in it, may use it. The tests live outside `pack`, so the starter does not build:

```text
error[E0603]: function `slots_left` is private
```

**pub opens the door.** `pub` in front of a `fn`, `struct`, `const` or `mod` lets code outside use it. Private is the default so that sharing is a decision: whatever stays private you can rename or rewrite later without breaking anyone who calls the room.

```rust
mod clock {
    const MINUTES_PER_HOUR: u32 = 60;   // private: only code in `clock`

    pub fn minutes(hours: u32) -> u32 {  // pub: anyone may call clock::minutes
        hours * MINUTES_PER_HOUR
    }
}
```

**Why rooms.** The pack's limit, `MAX_MODS`, stays private, so no other part of the launcher can change it by accident. The open doors, the `pub` items, are a short list: that list is how the rest of the launcher is allowed to use the pack.

## Your turn

- Put `pub` in front of `slots_left` inside `pack`.
- In `can_install`, call `pack::slots_left` and return whether it is above 0.
