**Why now:** a chat line you type is temporary, but the log has to keep it, and the console has to read player names without taking them. Two string types, one rule of thumb:

- **`String`** owns its text. It can grow. It lives on the heap. The log stores these.
- **`&str`** borrows text that lives somewhere else (a literal, or inside a String). Function parameters that only read take these.

**Parameter rule:** if the function only reads text, take `&str`. Callers can pass a literal *or* `&my_string`: Rust converts for you.

```rust
fn welcome(name: &str) -> String {
    format!("Welcome, {name}!")
}

let joined = String::from("Ferris");
welcome("Ana");        // literal
welcome(&joined);      // &String becomes &str automatically
let kept: String = "brb".to_string(); // &str -> String (copies)
match joined.as_str() { // String -> &str, handy for match
    "Ferris" => println!("crab"),
    _ => println!("other"),
}
```

Return a `String` when the result is new text the caller will own.

## Your turn

- In `src/lib.rs`, fill in `welcome`, `chat_line` and `permission_level`.
- Short quest: three one-liners.
