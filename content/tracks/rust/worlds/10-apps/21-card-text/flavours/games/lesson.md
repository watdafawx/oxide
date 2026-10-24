The mod screen shows each mod as `[x] sodium` or `[ ] optifine`, and a header like "2 of 3 mods enabled". Before any window code, write that text as plain functions: the screen then only asks for it, and a test checks it in a millisecond. No new crate or idea today, just tools from earlier worlds.

**If picks a value.** From world 01, `if` is an expression, so it can choose a value directly: `let mark = if enabled { "[x]" } else { "[ ]" };`. Both branches give a `&str`, so `mark` is one. `format!("{mark} {name}")` then glues it to the name.

**Count what matches.** From world 03, `.iter().filter(..).count()` counts the items a closure says yes to. With a list of `(name, enabled)` pairs, the closure can unpack each pair in its parameter list, `|(_, e)|`, keeping the bool and skipping the name with `_`. `*e` reads the bool through the reference.

```rust
fn server_line(name: &str, online: bool) -> String {
    let state = if online { "UP  " } else { "DOWN" };
    format!("{state} {name}")
}

let servers = [("survival", true), ("creative", false), ("skyblock", true)];
let up = servers.iter().filter(|(_, online)| *online).count();   // 2
```

**Why plain functions.** This is the **view model** idea: the screen asks functions for its words and numbers, and the drawing code stays thin. Text bugs are the most common screen bugs, and here a normal unit test finds them without a window.

## Your turn

- In `card_line`, give `"[x] name"` for an enabled mod and `"[ ] name"` for a disabled one.
- In `pack_summary`, count the enabled mods and give `"2 of 3 mods enabled"`.
