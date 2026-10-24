The console reads player names all day without keeping them, and the chat log keeps lines long after the typed text is gone. A function that only reads a name should accept any text: a literal in the code, or a `String` someone owns. Rust has a type for exactly that, borrowed text, and you have been writing it since the first tutorial quest.

**Two kinds of text.** A `String` owns its letters on the heap and can grow. A **`&str`** (say "string slice") is borrowed text: a pointer to letters someone else owns, plus how many there are.

**Literals are &str.** `"Steve"` in your code is a `&str`. Its letters are baked into the program file itself and stay put for as long as the server runs, so anyone may borrow them.

**The rule for parameters.** If a function only reads text, take `&str`. Callers can pass a literal, or lend a `String` with `&joined`: Rust turns a `&String` into a `&str` for you, because a String already holds the pointer and the length a `&str` needs.

```rust
fn shout_name(name: &str) -> String {
    format!("** {name} **")
}

let joined = String::from("Alex");
let a = shout_name("Steve");     // a literal
let b = shout_name(&joined);     // a borrowed String works too
```

**The rule for results.** When a function makes new text, give back a `String`, so the caller becomes its owner. `.to_string()` turns borrowed text into a fresh `String` with its own copy of the letters: that is how the log keeps a line.

**For the curious.** A literal's full type is `&'static str`. The `'static` means "valid until the program ends"; the tick mark comes back, explained, in the last quests of this world.

## Your turn

- In `welcome`, build `"Welcome, <name>!"`.
- In `chat_line`, give back an owned copy of `text`.
