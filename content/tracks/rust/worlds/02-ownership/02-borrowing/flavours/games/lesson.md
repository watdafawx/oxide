**Why now:** the `list` command only needs to *look* at who is online. If it took the player list by value, the server would lose its own list after one `list`. A **reference** lets a function look at a value without taking it.

`&x` borrows `x`. The owner keeps ownership; the function gets a temporary read-only view.

```rust
struct Roster {
    players: Vec<String>,
}

fn count(roster: &Roster) -> usize {
    roster.players.len()
}

let roster = Roster { players: vec!["Steve".to_string()] };
let n = count(&roster);                        // lend it
println!("{n} {}", roster.players.len());      // still ours
```

- The type `&Roster` means "a borrowed Roster".
- Looping over a borrowed list works the same way: `for name in &roster.players { ... }` looks at each name without taking the list.
- Borrow as many times at once as you like, as long as nobody changes it.
- No `.clone()` needed: nothing is copied.

## Your turn

- In `src/lib.rs`, fill in `count_players` and `total_chars`. Both already take `&Roster`.
- Save: the third objective passes once both work and the roster is still usable.
