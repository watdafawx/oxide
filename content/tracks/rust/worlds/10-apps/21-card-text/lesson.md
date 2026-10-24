The quest screen shows each objective as `[x] Count goes up` or `[ ] Count goes down`, and a header like "2 of 3 quests done". Before any window code, write that text as plain functions: the screen then only asks for it, and a test checks it in a millisecond. No new crate or idea today, just tools from earlier worlds.

**If picks a value.** From world 01, `if` is an expression, so it can choose a value directly: `let mark = if done { "[x]" } else { "[ ]" };`. Both branches give a `&str`, so `mark` is one. `format!("{mark} {title}")` then glues it to the title.

**Count what matches.** From world 03, `.iter().filter(..).count()` counts the items a closure says yes to. With a list of `(title, done)` pairs, the closure can unpack each pair in its parameter list, `|(_, d)|`, keeping the bool and skipping the title with `_`. `*d` reads the bool through the reference.

```rust
fn lamp_line(room: &str, on: bool) -> String {
    let state = if on { "on " } else { "off" };
    format!("{state} {room}")
}

let lamps = [("hall", true), ("kitchen", false), ("attic", true)];
let lit = lamps.iter().filter(|(_, on)| *on).count();   // 2
```

**Why plain functions.** This is the **view model** idea: the screen asks functions for its words and numbers, and the drawing code stays thin. Text bugs are the most common screen bugs, and here a normal unit test finds them without a window.

## Your turn

- In `quest_line`, give `"[x] title"` for a finished quest and `"[ ] title"` for an open one.
- In `progress_summary`, count the finished quests and give `"2 of 3 quests done"`.
