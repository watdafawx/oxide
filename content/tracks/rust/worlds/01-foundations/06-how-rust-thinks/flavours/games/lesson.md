**Why now:** the next quests turn your calculator into real features: recipe lookup, parsing recipe lines, bad-input handling. Three Rust ideas decide how all of that looks. Read once, then they click.

**1. The compiler is your first tester.**
Types, mutability and (later) ownership are checked **before** the program runs. Mix a stack count with an item name and Rust refuses to build. A compile error is Rust saying "this would have been a bug". Read it: the message usually tells you the fix.

**2. There is no null.**
A value is never secretly "missing". Looking up `"diamond_pickaxe"` in your recipe book might find nothing, so the return type says so: `Option<T>` is either `Some(value)` or `None`. You must handle `None` before you can use the value.

**3. There are no exceptions.**
A function that can fail returns `Result<T, E>`: either `Ok(value)` or `Err(reason)`. Parsing the line `"stick x4 = plank x2"` can go wrong, so the failure is part of the signature and callers cannot forget it.

```rust
let recipe: Option<u32> = None;               // "maybe a crafted count"
let parsed: Result<u32, String> = Ok(4);      // "a count or an error message"
```

Why bother? Whole families of crashes (null pointers, uncaught errors, type mix-ups) become compile errors you fix in seconds instead of bugs you hunt for hours on a live server.

## Your turn

- Answer the two questions. No code this time.
