Rust ships a handful of traits that nearly every type wants. Some you get for one line, some you write.

```rust
use std::fmt;

#[derive(Debug, Clone, PartialEq, Default)]   // the compiler writes these impls
struct Item { name: String, qty: u32 }

impl fmt::Display for Item {                  // you write this one: it is the user-facing text
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} x{}", self.name, self.qty)
    }
}

impl From<&str> for Item {                    // gives you .into() for free
    fn from(name: &str) -> Self { Item { name: name.to_string(), qty: 1 } }
}

let a: Item = "sword".into();                 // From
let b = a.clone();                            // Clone
assert_eq!(a, b);                             // PartialEq
println!("{a} / {a:?}");                      // Display / Debug
let none = Item::default();                   // "" and 0
```

- **`derive`** works when every field already has the trait.
- **`Debug`** is for programmers, **`Display`** is for users.
- Implement **`From<X>`** and you get `.into()` automatically.

## Your turn

- Open `src/lib.rs`.
- Add the `derive` line above `struct Player`.
- Write the four helpers, `Display for Player` and `From<&str> for Player`.
