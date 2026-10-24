The `list` command only needs to look at who is online. If it took the player list by value, the server would lose its own list after one `list`; cloning every name each time would be waste. What you want is to lend the list: let the command look, then the server carries on with it.

**A borrow.** `&roster` makes a **reference**: a pointer to `roster` that does not own it. Handing a reference to a function is called **borrowing**. The function reads through it, and at its `}` nothing is dropped, because it never owned anything.

**Say it in the type.** A parameter of type `&Roster` means "a borrowed Roster". Inside, `roster.players` reads the field through the reference, just as if you held the roster yourself.

**You have done this.** The `&` in `&self` in the crafting calculator is the same thing: a method that borrows the value it is called on. So is `for name in &roster.players`, which walks a list without taking it.

```rust
fn longest_name(banlist: &Banlist) -> usize {
    let mut longest = 0;
    for name in &banlist.names {
        if name.len() > longest {
            longest = name.len();
        }
    }
    longest
}

let banlist = Banlist { names: vec!["griefer".to_string(), "bot42".to_string()] };
let n = longest_name(&banlist);       // lend it
let still_ours = banlist.names.len(); // and keep it
```

**Many readers.** You may lend the same value to as many readers at once as you like, because nobody can change it while it is lent. The tab list, the `list` command and the web map can all look at once.

**vec! and the bang.** `vec![a, b]` makes a `Vec` holding `a` and `b`. Names ending in `!`, like `format!` and `vec!`, are **macros**: shorthand that Rust expands into longer code before compiling.

## Your turn

- In `count_players`, give back the length of `roster.players`.
- In `total_chars`, loop over `&roster.players` and add each name's `len()` to a `mut` total.
