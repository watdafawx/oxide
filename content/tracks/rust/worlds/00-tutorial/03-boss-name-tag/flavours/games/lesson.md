**Why now:** every player on your server gets a tag floating over their head: the name in capitals and their level. Two small functions, one calling the other.

A **boss** is a quest with several objectives. They are ordered: the first one is the easiest, and later ones usually build on it. Clear them top to bottom.

Two things you need:

```rust
let loud = "hi".to_uppercase();      // String: "HI"
let tag = format!("{loud} x{}", 3);  // "HI x3"  ({} takes the next argument)
```

Functions can call other functions. Build small pieces, then compose them.

## Your turn

- `loud_name(name)` returns the name in uppercase.
- `player_tag(name, level)` returns `[NAME] Lv N`, using `loud_name`.

Press Run to see the tags.
