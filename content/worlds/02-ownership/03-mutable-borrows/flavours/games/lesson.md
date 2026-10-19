**Why now:** `kick` and `rename` change the server's data, so the command needs write access to the player list. Borrow it with **`&mut`**.

The caller opts in twice: the variable is `mut`, and the call says `&mut`.

```rust
fn rename(name: &mut String) {
    name.push_str("_afk");
}

let mut player = String::from("Steve");
rename(&mut player);
println!("{player}"); // Steve_afk
```

**The one-writer rule:** at any moment you may have *either* many `&` readers *or* exactly one `&mut` writer, never both. That prevents "someone changed the list under me" bugs at compile time.

Two `&mut` to two *different* values (like the online list and the kicked list) are fine.

## Your turn

- In `src/lib.rs`, fill in `tag_as_op`, `queue_twice` and `kick_last`.
- Each takes `&mut`: the tests already pass `&mut` for you.
