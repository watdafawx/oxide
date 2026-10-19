Two string types confuse everyone once. The rule of thumb fixes it.

- **`String`** owns its text. It can grow. It lives on the heap.
- **`&str`** borrows text that lives somewhere else (a literal, or inside a String).

**Parameter rule:** if the function only reads text, take `&str`. Callers can pass a literal *or* `&my_string`: Rust converts for you.

```rust
fn greet(name: &str) -> String {
    format!("Hello, {name}!")
}

let owned = String::from("Ferris");
greet("Ana");          // literal
greet(&owned);         // &String becomes &str automatically
let s: String = "hi".to_string(); // &str -> String (copies)
match owned.as_str() { // String -> &str, handy for match
    "Ferris" => println!("crab"),
    _ => println!("other"),
}
```

Return a `String` when the result is new text the caller will own.

## Your turn

- In `src/lib.rs` fill in `greet`, `into_owned` and `rank_owned`.
- Short quest: three one-liners.
