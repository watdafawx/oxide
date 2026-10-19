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

A `&mut Vec` works the same way: `push` adds to the end, `pop` removes the last item and returns an `Option`.

```rust
fn grow(v: &mut Vec<i32>) {
    v.push(7);                  // changes the caller's Vec
    let last = v.pop();         // Some(7), or None if the Vec was empty
}
```

**The one-writer rule:** at any moment you may have *either* many `&` readers *or* exactly one `&mut` writer, never both. That prevents "someone changed it under me" bugs at compile time.

Two `&mut` to two *different* values (like two Vecs) are fine.

## Your turn

- In `src/lib.rs` fill in `add_exclaim`, `push_twice` and `move_last`.
- Each takes `&mut`: the tests already pass `&mut` for you.
