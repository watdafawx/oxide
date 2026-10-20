**Why now:** this is the server console itself. Everything from this world in one program: **borrow** the server to look (`list`), borrow it **mutably** to act (`tp`, `give`, `kick`), and slice each typed line into a command and its arguments.

The console is a small state machine: it owns the server, and every command line moves it to a new state.

```rust
pub struct Console {
    pub server: Server,   // owns every Player and the stash
}
pub struct Server {
    pub players: Vec<Player>,
    pub stash: Vec<String>,
}
```

Rules of the fight:

- **Read-only methods** (`list`, `player_name`) take `&self`.
- **Methods that change things** (`tp`, `give`, `kick`) take `&mut self`.
- `tp` returns `Result<&str, String>`: the `&str` borrows from the server, so it's valid until the next change. Use it, then move on.
- No `.clone()` on players or the server. Moving a `String` item out of the stash is fine.
- `BlockPos` is `Copy`: pass it around freely.
- Each command line is **sliced**, not copied: `line.split_whitespace().collect::<Vec<&str>>()` and match on `words.as_slice()` with **slice patterns** like `["kick", name]` (matches a 2-word slice whose first word is `kick` and binds the second to `name`) or `["tp", name, x, y, z]`.
- Coordinates arrive as text: `x.parse::<i32>()` gives a `Result`. A coordinate that is not a number means `Unknown command`.

You start with **Steve** at `0,64,0` (empty inventory), **Alex** at `10,70,-3` (holding a `compass`) and a stash with a `diamond` and a `torch`.

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

## Your turn

- Open `src/lib.rs`. `Console::demo()` and the types are done. Replace each `todo!()`, top to bottom, one objective at a time.
- `kick` has no objective of its own: `run_plays_a_whole_console_script` covers it at the end.
- `run` builds a fresh `Console::demo()` and plays each line: `list`, `tp <name> <x> <y> <z>`, `give <name> <item>`, `kick <name>`, anything else is "Unknown command: ...".

Press Run to play it.
