This is the server console itself. Everything from this world goes into one program: borrow the server to look (`list`), borrow it mutably to act (`tp`, `give`, `kick`), move items from the stash to a player, and slice each typed line into a command and its arguments.

**Who owns what.** The `Console` owns a `Server`, which owns every `Player` and the admin stash. Each player owns their name and inventory as `String`s. Drop the console and everything inside goes with it.

```rust
pub struct Console {
    pub server: Server,   // players: Vec<Player>, stash: Vec<String>
}
```

**Look with &self.** `list` and `player_name` only read, so they take `&self`. `player_name` lends out a `&str` borrowed from the server: one borrowed input, so no `'a` is needed.

**Act with &mut self.** `tp`, `give` and `kick` change the server, so they take `&mut self`. `tp` gives back `Result<&str, String>`: the `&str` borrows the player's name from the server, valid until the next change.

**Positions are Copy.** `BlockPos` derives `Copy`, as in "Copy your own type", so pass it around and use it twice freely. Walk the players with `for p in &mut self.server.players` and set `p.pos` on the right one.

**Moving an item.** `give` is "Take it out" again: find the item's position in the stash, `remove` it, and push the owned `String` onto the player's inventory. Find the player's position first, so you are not holding a borrow of the players while you change the stash.

**Two small tools.** `list.join("; ")` glues a `Vec` of text into one `String` with a separator between the pieces. `!list.is_empty()` is true when the list has something in it: `!` flips a `bool`.

**Split a command.** `line.split_once(' ')` gives `Some(("tp", "Steve 5 70 5"))`, or `None` when there is no space; split the rest again for the name. `parse_pos` is written for you. `for line in script` lends each line as a `&&str`, an arrow to an arrow, so write `*line` where a plain `&str` is wanted.

## Your turn

- Do the objectives top to bottom: `player_name`, `list`, `tp`, `give`, then `run`. `Console::demo()`, the types and `parse_pos` are done.
- `kick` has no objective of its own: the last objective covers it.
- Save after each one and watch another objective turn green. The exact replies are in the comment above each function, and in the tests.
