The next quests all lean on three ideas. Read once, then they click.

**1. The compiler is your first tester.**
Types, mutability and (later) ownership are checked **before** the program runs. A compile error is Rust saying "this would have been a bug". Read it: the message usually tells you the fix.

**2. There is no null.**
A value is never secretly "missing". If something can be absent, its type says so: `Option<T>` is either `Some(value)` or `None`. You must handle `None` before you can use the value.

**3. There are no exceptions.**
A function that can fail returns `Result<T, E>`: either `Ok(value)` or `Err(reason)`. The failure is part of the signature, so callers can't forget it.

```rust
let n: Option<i32> = None;          // "maybe a number"
let r: Result<i32, String> = Ok(3); // "a number or an error message"
```

Why bother? Whole families of crashes (null pointers, uncaught errors, type mix-ups) become compile errors you fix in seconds instead of bugs you hunt for hours.

## Your turn

- Answer the two questions. No code this time.
