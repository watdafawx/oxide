The last quest of every world is a **boss**: a bigger job with several objectives. This one builds the tag floating over each player on your server: the name in capitals and their level.

**Stages.** A boss has more than one objective, and they are in order. The first one is the easiest and the later ones build on it. Clear them from the top.

**Capital letters.** Text has a method that gives you the capital-letter version. You write the method after a dot, and it hands back the new text. Try it on a word:

```rust
let loud = "hi".to_uppercase();      // "HI"
let tag = format!("{loud} x{}", 3);  // "HI x3"
```

**Two ways into the hole.** `{player}` style works for a plain variable. To put a function call's answer into the text, leave the braces empty, `{}`, and add the call after the string: `format!("({})", double(4))` gives `(8)`.

**A function calling a function.** The second function can use the first one. Build the small piece (`loud_name`), then reuse it inside the bigger one (`player_tag`). That is how every program in this course grows.

## Your turn

- In `loud_name`, return the name in capitals.
- In `player_tag`, call `loud_name` and build `[NAME] Lv N`, for example `[FERRIS] Lv 3`.
- Save after each one. Press Run to see the tags printed.
