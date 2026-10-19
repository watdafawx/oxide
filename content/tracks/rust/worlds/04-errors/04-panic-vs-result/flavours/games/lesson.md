**Why now:** your manifest loader is about to meet two kinds of trouble. A player forgot an optional mod: that is normal life, so report it and carry on. Your own built-in loader table has a hole in it: that is a bug, so crash loudly. Rust has a tool for each.

```rust
let version: u32 = line_version.parse()?;                  // player typed it: return the error
let fabric = loaders.iter().find(|l| l.name == "fabric")
    .expect("built-in loader table always has fabric");    // can't fail: document why
let first = manifest_lines[0];                             // bug if the manifest is empty: panics
```

- **`Result`** is for things that go wrong in normal life: missing mod file, bad version, network down. The caller decides.
- **`panic!`** is for bugs: broken invariants that mean the code itself is wrong. It stops the thread.
- **`unwrap()`** panics with a generic message. Fine in tests and quick scripts.
- **`expect("why")`** panics with *your* message. Use it when failure is impossible and say why.
- **`?`** is the default in real code: pass the problem up.

Rule of thumb: if the player can cause it, return `Err`. If only a programmer mistake can cause it, panic.

## Your turn

Answer the two questions below.
