`vec![1, 2, 3]` takes any number of arguments. You can do that in `macro_rules!` with **repetition**.

- `$( $x:expr ),*` matches zero or more expressions separated by commas. Use `+` for one or more.
- In the expansion, `$( ... ),*` repeats once per match. Whatever you put inside runs for each `$x`.
- `$(,)?` at the end accepts an optional trailing comma.
- A macro may call itself. Peel off the first argument and recurse on the `$rest`. Put the base case (one argument) first.

```rust
macro_rules! sum {
    ($x:expr) => { $x };
    ($x:expr, $($rest:expr),+) => { $x + sum!($($rest),+) };
}

assert_eq!(sum!(1, 2, 3), 6);
```

## Your turn

In `src/lib.rs`:

- `vec_of_strings!`: turn every argument into a `String`; allow zero arguments and a trailing comma.
- `max!`: add the recursive arm so it works for any number of arguments.
