A function that only reads a name should accept any text: a literal typed into the code, or a `String` someone owns. Rust has a type for exactly that, borrowed text, and you have been writing it since the first tutorial quest.

**Two kinds of text.** A `String` owns its letters on the heap and can grow. A **`&str`** (say "string slice") is borrowed text: a pointer to letters someone else owns, plus how many there are.

**Literals are &str.** `"Ana"` in your code is a `&str`. Its letters are baked into the program file itself and stay put for as long as the program runs, so anyone may borrow them.

**The rule for parameters.** If a function only reads text, take `&str`. Callers can pass a literal, or lend a `String` with `&owned`: Rust turns a `&String` into a `&str` for you, because a String already holds the pointer and the length a `&str` needs.

```rust
fn label(item: &str) -> String {
    format!("[{item}]")
}

let owned = String::from("jam");
let a = label("bread");    // a literal
let b = label(&owned);     // a borrowed String works too
```

**The rule for results.** When a function makes new text, give back a `String`, so the caller becomes its owner. `.to_string()` turns borrowed text into a fresh `String` with its own copy of the letters.

**For the curious.** A literal's full type is `&'static str`. The `'static` means "valid until the program ends"; the tick mark comes back, explained, in the last quests of this world.

## Your turn

- In `greet`, build `"Hello, <name>!"`.
- In `into_owned`, give back an owned copy of `text`.
