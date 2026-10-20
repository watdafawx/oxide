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
- Build the exit and item lists with `Vec<String>::join(", ")`: it glues the pieces with a separator.
- Rooms point at each other by **index** (`usize`), not by reference. Self-referencing structs are a fight you don't want yet.

Map you start with: **Cellar** (lamp) → north → **Hall** → east → **Library** (key, book).

Exact text formats are in the doc comment above each function in `src/lib.rs`, and in the tests.

## Toolbox you have not met yet

Skim these Vec moves; the hints name them again when you need one.

```rust
let mut names: Vec<String> = vec!["Steve".to_string(), "Alex".to_string()];
names.is_empty();                                  // false
names.iter().position(|n| n == "Alex");            // Some(1): the index of the first match, or None
let gone = names.remove(1);                        // takes index 1 out and hands you the owned String
for n in names.iter_mut() { n.push('!'); }         // like iter(), but each n is a &mut: you may change it
let first = names.iter_mut().find(|n| n.len() > 3); // Option<&mut String>
```

- `!x.is_empty()` means "is not empty".
- `for &line in script` loops over a `&[&str]` and gives you each `&str` itself.
- `entry.split_once(' ')` cuts text at the first space into `Some((before, after))`, or `None`.
- A closure can pick tuples apart: `exits.iter().find(|(dir, _)| dir == want)` looks at the first part of each `(String, usize)` pair.

## Your turn

- Open `src/lib.rs`. `Game::demo()` and the types are done. Replace each `todo!()`, top to bottom, one objective at a time.
- `run` builds a fresh `Game::demo()` and plays each command: `look`, `go <dir>`, `take <item>`, anything else is "Unknown command: ...".

Press Run to play it.
