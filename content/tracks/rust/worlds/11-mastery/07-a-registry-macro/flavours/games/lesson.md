`block!("stone" => 1)` makes one `Block`. The chunk indexer's registry is a list of them, and `registry!["stone" => 1, "dirt" => 2]` should give the whole `Vec<Block>` in one go, in the shape of the data.

**Two captures per round.** A repetition can hold more than one capture. `$( $name:expr => $id:expr ),*` matches name, `=>`, id, then a comma and the next round. In the template, `$name` and `$id` repeat together, so round 2's name stays next to round 2's id.

**A macro inside a template.** The template may call another macro, and that call is expanded in turn. `$( $crate::block!($name => $id) ),*` writes one `block!` call per pair, separated by commas, and each one becomes a `Block`. Through `$crate::`, the call works from any module, as in the last quest.

```rust
#[macro_export]
macro_rules! total_items {
    ($($name:expr => $count:expr),* $(,)?) => {
        0 $( + $count )*
    };
}

assert_eq!(total_items!["iron_ingot" => 64, "coal" => 32,], 96);   // 0 + 64 + 32
```

**Only what you use.** `total_items!` captures every `$name` and never pastes it. That is allowed: a capture the template ignores is simply dropped.

## Your turn

- In `registry!`, replace the stub with a `vec![ ]` that holds one `$crate::block!` call per pair.
