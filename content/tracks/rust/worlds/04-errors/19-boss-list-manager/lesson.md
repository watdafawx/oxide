Time to put the world together: a todo list that remembers. It reads its tasks from a file, changes them on command and writes them back, with your own error type where callers need to match and anyhow where the app only reports.

**Two kinds of errors.** `toggle` and `parse_command` return **`TodoError`**, a thiserror enum, because callers match on it: the tests check for `NotFound` and `BadCommand`. `load`, `save` and `run` return **`anyhow::Result`**, because the top of the app only shows the message. `?` turns a `TodoError` into an `anyhow::Error` on the way up.

```rust
// inside: precise, matchable
#[derive(Debug, thiserror::Error)]
enum TodoError {
    #[error("no todo with id {0}")]
    NotFound(u32),
}

// outside: one type for anything, with context
fn run(store: &mut Store, cmd: Command) -> anyhow::Result<String> {
    store.toggle(9)?;   // a TodoError becomes an anyhow::Error
    // ...
}
```

**The file.** Each todo is one line, `id|done|text`, such as `2|1|buy eggs` for todo 2, done. `format_line` and `parse_line` are given, so `save` and `load` only deal with files and errors.

**First start.** A file that does not exist yet is an empty todo list, as in the missing-file quest. Any other read error goes back with context naming the path.

**Which line broke.** When `parse_line` fails, add the context `line <n>`, so the message points at the broken line. `text.lines().enumerate()` hands you `(i, line)` pairs with `i` counting from 0, so the line number is `i + 1`.

**Change it in place.** `for todo in &mut self.todos` lends you each todo mutably, so you can change its fields. `!` in front of a `bool` flips it: `todo.done = !todo.done;`.

**Commands.** `split_verb(input)` is given: it trims the input and cuts off the first word, so `"done 3"` becomes `("done", "3")`. A `match` on the verb picks the command.

## Your turn

- Fill every `todo!()`, working down the objectives: the first one is one line.
- An order that works: `new`, `add`, `toggle`, `parse_command`, `save`, `load`, `run`.
- When all eight are green, the program in `main.rs` is a todo list you can use: it saves to `todos.txt`.
