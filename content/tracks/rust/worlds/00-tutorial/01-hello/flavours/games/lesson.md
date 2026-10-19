**Why now:** a server plugin's first job is talking to players. When someone joins, your code builds one chat line from their name. That is a function that takes text and returns text.

Every quest works the same way: Oxide copies a small project into your quests folder, you edit it in your editor, and **every time you save** the tests run and the objectives on the right tick off.

Rust functions look like this:

```rust
fn double(n: i32) -> i32 {
    n * 2          // no semicolon = this is the return value
}
```

- `fn` starts a function, `-> i32` is the return type.
- A player name arrives as `&str` (borrowed text). The chat line you build is a `String` (owned text). `format!("Welcome, {player}!")` builds a `String`.
- `todo!()` compiles anywhere and panics when run. It marks the spot you fill in.

## Your turn

- Open `src/lib.rs`.
- Replace the `todo!()` in `join_message` so it returns `Welcome to the server, <player>!`.
- Save. Watch the objective go green.
