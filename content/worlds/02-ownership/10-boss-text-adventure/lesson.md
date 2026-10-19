Everything from this world in one program: a tiny text adventure where you borrow the world to look and borrow it mutably to act.

The game is a struct that owns the state:

```rust
pub struct Game {
    pub world: World,   // Vec<Room>, owns every room
    pub player: Player, // which room (index) + inventory
}
```

Rules of the fight:

- **Read-only methods** (`describe`, `room_name`) take `&self`.
- **Methods that change things** (`go`, `take`) take `&mut self`.
- `go` returns `Result<&str, String>`: the `&str` borrows from the game, so it's valid until the next change. Use it, then move on.
- No `.clone()` on rooms or the world. Cloning a `String` to move an item is fine.
- Rooms point at each other by **index** (`usize`), not by reference. Self-referencing structs are a fight you don't want yet.

Map you start with: **Cellar** (lamp) → north → **Hall** → east → **Library** (key, book).

Text formats are in the objective labels and the tests. Read the failing assertion for the exact string.

## Your turn

- Open `src/lib.rs`. `Game::demo()` and the types are done. Replace each `todo!()`, top to bottom, one objective at a time.
- `run` builds a fresh `Game::demo()` and plays each command: `look`, `go <dir>`, `take <item>`, anything else is "Unknown command: ...".
