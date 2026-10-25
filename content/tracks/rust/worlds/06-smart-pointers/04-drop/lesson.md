While the outliner saves, it writes a temporary file, and that file must be deleted afterwards, also when a save stops halfway. "Remember to clean up" fails the one time someone forgets. Rust can run the cleanup for you, at the moment the value dies.

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
    println!("reading");
}                      // "lamp off" prints here, at the closing brace
```

**Last in, first out.** Two values in one scope are dropped in reverse order: the one made last goes first. That is the safe order, because a later value may depend on an earlier one, like a file opened inside a locked folder.

**Never call it yourself.** Writing `lamp.drop()` is an error (`E0040`). Rust calls it exactly once, and calling it by hand would run the cleanup twice. The next quest shows how to end a value early.

## Your turn

- In `drop` for `TempFile`, push `"delete <name>"` onto `self.log`, with the file's name in place of `<name>`.
- The test makes a file in an inner block and reads the log after the closing brace.
