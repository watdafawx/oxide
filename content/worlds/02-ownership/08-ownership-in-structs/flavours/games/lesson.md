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

You do not write any lifetimes in this quest: both structs and the `new` functions are given. Your job is to produce a **view** of a `Player`. The next quest shows the one place you must write a lifetime by hand.

## Your turn

- In `src/lib.rs`, fill in `Player::view`. Everything else is written.
- Notice `PlayerRef<'_>` in its signature: it ties the view to `&self`.
