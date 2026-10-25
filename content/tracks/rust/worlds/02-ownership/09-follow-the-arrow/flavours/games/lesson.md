The `xp` command adds experience to a player, and the leaderboard counts who is high level. A `&mut u32` is an arrow pointing at a number somewhere else. Write `xp += 50` and you are asking Rust to add 50 to the arrow, not to the number it points at. Rust says no.

**Try the obvious thing.** With `xp: &mut u32`, the line `xp += n;` stops the build. The help line underneath already names the fix:

```text
error[E0368]: binary assignment operation `+=` cannot be applied to type `&mut u32`
```

**Follow the arrow.** `*xp` means "the value `xp` points at". `*xp += 50` adds to the number itself, in the caller's variable. Following a reference like this is called **dereferencing**, and `*` is the **dereference** operator.

**Same when reading.** `for l in &board.levels` lends each level as a `&u32`. To compare it with a plain number, follow it first: `*l >= level`.

```rust
fn heal(health: &mut u32) {
    *health += 4;
}

fn count_starving(server: &Hunger) -> usize {
    let mut n = 0;
    for food in &server.food {
        if *food == 0 {
            n += 1;
        }
    }
    n
}
```

**Dots follow for you.** Methods and fields follow the arrow on their own: `roster.players.len()` works on a `&Roster`, and `name.push_str` on a `&mut String`. That is why you needed no `*` until now.

**An arrow to an arrow.** A reference can point at another reference, written `&&u32`, and `**r` follows both. You will meet it when looping over borrowed lists in the next world; the fix is always one more `*`.

## Your turn

- In `add_xp`, add `n` to the number `xp` points at.
- In `count_at_least`, count the levels that are `level` or higher. Follow each `l` with `*` to compare it.
