Sometimes a function must change the caller's value. Borrow it with **`&mut`**.

The caller opts in twice: the variable is `mut`, and the call says `&mut`.

```rust
fn shout(s: &mut String) {
    s.push_str("!!");
}

let mut word = String::from("hey");
shout(&mut word);
println!("{word}"); // hey!!
```

**The one-writer rule:** at any moment you may have *either* many `&` readers *or* exactly one `&mut` writer, never both. That prevents "someone changed it under me" bugs at compile time.

Two `&mut` to two *different* values (like two Vecs) are fine.

## Your turn

- In `src/lib.rs` fill in `add_exclaim`, `push_twice` and `move_last`.
- Each takes `&mut`: the tests already pass `&mut` for you.
