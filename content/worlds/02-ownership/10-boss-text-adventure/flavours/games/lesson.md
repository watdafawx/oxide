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
- Each command line is **sliced**, not copied: `line.split_whitespace().collect::<Vec<&str>>()` and match on `words.as_slice()` with patterns like `["kick", name]`.

You start with **Steve** at `0,64,0` (empty inventory), **Alex** at `10,70,-3` (holding a `compass`) and a stash with a `diamond` and a `torch`.

Exact text formats are in the doc comment above each function in `src/lib.rs`, and in the tests.

## Your turn

- Open `src/lib.rs`. `Console::demo()` and the types are done. Replace each `todo!()`, top to bottom, one objective at a time.
- `kick` has no objective of its own: `run_plays_a_whole_console_script` covers it at the end.
- `run` builds a fresh `Console::demo()` and plays each line: `list`, `tp <name> <x> <y> <z>`, `give <name> <item>`, `kick <name>`, anything else is "Unknown command: ...".

Press Run to play it.
