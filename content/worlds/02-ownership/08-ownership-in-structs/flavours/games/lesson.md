**Why now:** the server's player list needs to *keep* players (they own their names), but a quick status screen only wants to *peek* at them without copying every name. A struct field can own its data (`String`) or borrow it (`&str`). The choice decides how long everything lives.

```rust
struct Player {
    name: String,          // owns the text
}

struct PlayerRef<'a> {
    name: &'a str,         // points at text owned elsewhere
}
```

- **Owned** is simple: the struct carries its text wherever it goes. Pick this by default.
- **Borrowed** is cheap (no copy) but the struct can't outlive the text it points at. Rust needs a name for "how long", so a **lifetime parameter** `'a` appears. Read `PlayerRef<'a>` as "a PlayerRef that is valid for as long as `'a`".
- The `'a` doesn't change how long anything lives. It only lets the compiler check the promise.

If the lifetime syntax feels like too much, use `String` and move on. The next quest shows the one place you must write it by hand.

## Your turn

- In `src/lib.rs`, fill in `Player::new`, `PlayerRef::new` and `Player::view`.
- The lifetimes are already written. Notice `PlayerRef<'_>` in `view`: it ties the view to `&self`.
