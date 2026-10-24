When you remove an inserter from the factory, it should write "power down" to the log, every time, without you remembering to. "Remember to clean up" fails the one time someone forgets. Rust can run the cleanup for you, at the moment the value dies.

**When a value dies.** A **scope** is the stretch of code between a `{` and its `}`. A variable lives until the end of the scope it was made in. At that closing brace its owner is gone, and Rust **drops** the value: it runs the value's cleanup code, if it has any, and frees its memory.

**The trait for it.** The cleanup code is the `Drop` trait: one method, `drop(&mut self)`, which Rust calls by itself. A `Box` uses it to free its heap value, and a `File` uses it to close the file.

```rust
struct Lamp;

impl Drop for Lamp {
    fn drop(&mut self) {
        println!("lamp off");
    }
}

{
    let _lamp = Lamp;
    println!("working");
}                      // "lamp off" prints here, at the closing brace
```

**Last in, first out.** Two values in one scope are dropped in reverse order: the machine placed last powers down first. That is the safe order, because a later machine may depend on an earlier one, like an inserter feeding a furnace.

**Never call it yourself.** Writing `lamp.drop()` is an error (`E0040`). Rust calls it exactly once, and calling it by hand would run the cleanup twice. The next quest shows how to remove a machine early.

## Your turn

- In `drop` for `Inserter`, push `"power down <name>"` onto `self.log`, with the inserter's name in place of `<name>`.
- The test places an inserter in an inner block and reads the log after the closing brace.
