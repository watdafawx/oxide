`item!("apples" => 3)` makes one `Item`. The shop's opening stock is a list of them, and `stock!["apples" => 3, "pears" => 2]` should give the whole `Vec<Item>` in one go, in the shape of the data.

**Two captures per round.** A repetition can hold more than one capture. `$( $name:expr => $count:expr ),*` matches name, `=>`, count, then a comma and the next round. In the template, `$name` and `$count` repeat together, so round 2's name stays next to round 2's count.

**A macro inside a template.** The template may call another macro, and that call is expanded in turn. `$( $crate::item!($name => $count) ),*` writes one `item!` call per pair, separated by commas, and each one becomes an `Item`. Through `$crate::`, the call works from any module, as in the last quest.

```rust
#[macro_export]
macro_rules! total_count {
    ($($name:expr => $count:expr),* $(,)?) => {
        0 $( + $count )*
    };
}

assert_eq!(total_count!["apples" => 3, "pears" => 2,], 5);   // 0 + 3 + 2
```

**Only what you use.** `total_count!` captures every `$name` and never pastes it. That is allowed: a capture the template ignores is simply dropped.

## Your turn

- In `stock!`, replace the stub with a `vec![ ]` that holds one `$crate::item!` call per pair.
