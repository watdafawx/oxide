**Why now:** a block registry lists as many blocks as you like, so the macro that builds it must accept any number of arguments.

You can take any number of arguments in `macro_rules!` with **repetition**, like `vec![1, 2, 3]` does.

- `$( $x:expr ),*` matches zero or more expressions separated by commas. Use `+` for one or more.
- In the expansion, `$( ... ),*` repeats once per match. Whatever you put inside runs for each `$x`.
- `$(,)?` at the end accepts an optional trailing comma.
- A macro may call itself. Peel off the first argument and recurse on the `$rest`. Put the base case (one argument) first.

```rust
macro_rules! total_blocks {
    ($x:expr) => { $x };
    ($x:expr, $($rest:expr),+) => { $x + total_blocks!($($rest),+) };
}

assert_eq!(total_blocks!(4, 2, 3), 9);
```

## Your turn

In `src/lib.rs`:

- `vec_of_strings!`: turn every argument into a `String`; allow zero arguments and a trailing comma.
- `hashmap!`: build a `HashMap` and insert the one `$k => $v` pair (the pattern matches the literal `=>`).
- `max!`: add the recursive arm so it works for any number of arguments.
