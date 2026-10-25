Time to put the world together: the launcher's mod list screen. Add a mod, switch it on or off, list what you have, and find it all still there tomorrow. Your own error type goes where callers need to match, and anyhow where the launcher only reports.

**Two kinds of errors.** `toggle` and `parse_command` return **`ModError`**, a thiserror enum, because callers match on it: the tests check for `NotFound` and `BadCommand`. `load`, `save` and `run` return **`anyhow::Result`**, because the top of the app only shows the message. `?` turns a `ModError` into an `anyhow::Error` on the way up.

```rust
// inside: precise, matchable
#[derive(Debug, thiserror::Error)]
enum ModError {
    #[error("no mod with id {0}")]
    NotFound(u32),
}

// outside: one type for anything, with context
fn run(list: &mut ModList, cmd: Command) -> anyhow::Result<String> {
    list.toggle(9)?;   // a ModError becomes an anyhow::Error
    // ...
}
```

**The file.** Each mod is one line, `id|enabled|name`, such as `2|0|lithium` for mod 2, switched off. `format_line` and `parse_line` are given, so `save` and `load` only deal with files and errors.

**First start.** A file that does not exist yet is an empty mod list, as in the missing-file quest. Any other read error goes back with context naming the path.

**Which line broke.** When `parse_line` fails, add the context `line <n>`, so the message points at the broken line. `text.lines().enumerate()` hands you `(i, line)` pairs with `i` counting from 0, so the line number is `i + 1`.

**Change it in place.** `for entry in &mut self.mods` lends you each mod mutably, so you can change its fields. `!` in front of a `bool` flips it: `entry.enabled = !entry.enabled;`.

**Commands.** `split_verb(input)` is given: it trims the input and cuts off the first word, so `"toggle 3"` becomes `("toggle", "3")`. A `match` on the verb picks the command.

## Your turn

- Fill every `todo!()`, working down the objectives: the first one is one line.
- An order that works: `new`, `add`, `toggle`, `parse_command`, `save`, `load`, `run`.
- When all eight are green, the program in `main.rs` is a mod list you can use: it saves to `mods.txt`.
