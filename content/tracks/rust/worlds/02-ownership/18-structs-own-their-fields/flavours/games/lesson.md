When someone joins, the server makes a player record and keeps it until they leave, long after the login line it read the name from is gone. Try to give the record a borrowed `&str` field for the name and Rust stops you.

**Try it.** The starter's `Player` has `name: &str`. Rust refuses to build:

```text
error[E0106]: missing lifetime specifier
```

**Why it stops you.** A `&str` field would point at text owned by someone else. If that text is dropped first, the player holds a dangling pointer. Rust has to know how long the borrowed text lives, and the struct does not say. (The help line offers a way to say it; that is the next quest.)

**Own it instead.** Make the field a `String`. The player owns their name, carries it wherever they go, and drops it when the record is dropped. Owning is the simple default: reach for it first.

```rust
struct Warp {
    label: String,     // the warp owns its label
    y: i32,
}

fn new_warp(label: &str, height: i32) -> Warp {
    Warp { label: label.to_string(), y: height }
}
```

**Ownership nests.** A struct owns its fields, and a `Vec` owns its items. Drop the server's player list and every player and every name in it is dropped with it, in one go.

## Your turn

- Save and read the error on `Player`.
- Change the `name` field to a `String`.
- In `Player::new`, store an owned copy of `name`.
