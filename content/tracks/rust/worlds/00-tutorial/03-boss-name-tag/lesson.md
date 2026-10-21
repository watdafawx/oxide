The last quest of every world is a **boss**: a bigger job with several objectives. This one builds the finished name tag, the capitals and the level together.

**Stages.** A boss has more than one objective, and they are in order. The first one is the easiest and the later ones build on it. Clear them from the top.

**Capital letters.** Text has a method that gives you the capital-letter version. You write the method after a dot, and it hands back the new text. Try it on a word:

```rust
let loud = "hi".to_uppercase();      // "HI"
let tag = format!("{loud} x{}", 3);  // "HI x3"
```

**Two ways into the hole.** `{name}` works for a plain variable. To put a function call's answer into the text, leave the braces empty, `{}`, and add the call after the string: `format!("({})", double(4))` gives `(8)`.

**A function calling a function.** The second function can use the first one. Build the small piece (`shout`), then reuse it inside the bigger one (`name_tag`). That is how every program in this course grows.

## Your turn

- In `shout`, return the name in capitals.
- In `name_tag`, call `shout` and build `[NAME] Lv N`, for example `[FERRIS] Lv 3`.
- Save after each one. Press Run to see the tags printed.
