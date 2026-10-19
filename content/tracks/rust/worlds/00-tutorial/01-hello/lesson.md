Every quest works the same way: Oxide copies a small project into your quests folder, you edit it in your editor, and **every time you save** the tests run and the objectives on the right tick off.

Rust functions look like this:

```rust
fn double(n: i32) -> i32 {
    n * 2          // no semicolon = this is the return value
}
```

- `fn` starts a function, `-> i32` is the return type.
- `&str` is borrowed text, `String` is owned text. `format!("Hello, {name}!")` builds a `String`.
- `todo!()` compiles anywhere and panics when run. It marks the spot you fill in.

## Your turn

- Open `src/lib.rs`.
- Replace the `todo!()` in `greeting` so it returns `Hello, <name>!`.
- Save. Watch the objective go green.
