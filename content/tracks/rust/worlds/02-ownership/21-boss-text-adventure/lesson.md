Everything from this world goes into one program: a tiny text adventure. The game owns its rooms and the player. You borrow it to look, borrow it mutably to act, move items from room to inventory, and never clone a room.

**Who owns what.** `Game` owns a `World` (a `Vec<Room>`) and a `Player`. Each `Room` owns its name, description, exits and items as `String`s. Drop the game and everything inside goes with it.

```rust
pub struct Game {
    pub world: World,   // rooms: Vec<Room>, owns every room
    pub player: Player, // room: usize, inventory: Vec<String>
}
```

**Look with &self.** `describe` and `room_name` only read, so they take `&self`. `room_name` lends out a `&str` borrowed from the world: one borrowed input, so no `'a` is needed.

**Act with &mut self.** `go` and `take` change the game, so they take `&mut self`. `go` gives back `Result<&str, String>`: the `&str` borrows the new room's name from the game, valid until the next change.

**Rooms point by number.** An exit is a `(String, usize)`: a direction and the index of the room it leads to. Copy that `usize` out first, then change `self.player.room`. Rooms holding references to each other would be a lifetime puzzle you don't want yet.

**Moving an item.** `take` is "Take it out" again: find the position, `remove` it from the room, and push the owned `String` onto the inventory. The room and the player are different fields, so you may borrow both mutably at once.

**Two small tools.** `list.join(", ")` glues a `Vec` of text into one `String` with a separator between the pieces. `!list.is_empty()` is true when the list has something in it: `!` flips a `bool`.

**Split a command.** `line.split_once(' ')` gives `Some(("go", "north"))`, or `None` when there is no space. `for line in script` lends each line as a `&&str`, an arrow to an arrow, so write `*line` where a plain `&str` is wanted.

## Your turn

- Do the objectives top to bottom: `room_name`, `describe`, `go`, `take`, then `run`. `Game::demo()` and the types are done.
- Save after each one and watch another objective turn green.
- The exact text of each reply is in the comment above each function, and in the tests.
