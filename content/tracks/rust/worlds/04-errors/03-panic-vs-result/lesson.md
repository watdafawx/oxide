Rust has two ways to fail. Picking the right one is a judgment call.

```rust
let n: i32 = input.parse()?;                       // expected failure: return it
let port: u16 = "8080".parse().expect("literal is valid"); // can't fail: document why
let first = list[0];                               // bug if empty: panics
```

- **`Result`** is for things that can go wrong in normal life: bad input, missing files, network down. The caller decides.
- **`panic!`** is for bugs: broken invariants that mean the code itself is wrong. It stops the thread.
- **`unwrap()`** panics with a generic message. Fine in tests and quick scripts.
- **`expect("why")`** panics with *your* message. Use it when failure is impossible and say why.
- **`?`** is the default in real code: pass the problem up.

Rule of thumb: if the user can cause it, return `Err`. If only a programmer mistake can cause it, panic.

## Your turn

Answer the two questions below.
