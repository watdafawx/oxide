`macro_rules!` matches patterns. A **procedural macro** is a Rust function that receives your code as tokens and returns new tokens. That is how `serde`, `clap` and `tokio::main` work.

Three kinds:

- **Derive**: `#[derive(Hello)]` on a struct or enum. Adds new items (usually an `impl`) and never changes the original.
- **Attribute**: `#[timed]` on an item. Gets the item *and* may rewrite it completely.
- **Function-like**: `sql!("SELECT ...")`. Looks like `macro_rules!`, parses anything you like.

```rust
// in the proc-macro crate
use proc_macro::TokenStream;

#[proc_macro_derive(Hello)]
pub fn derive_hello(input: TokenStream) -> TokenStream {
    // parse `input` with `syn`, build the answer with `quote!`
    TokenStream::new()
}
```

Two crates help: **`syn`** parses tokens into a syntax tree (`DeriveInput`, `ItemFn`), **`quote`** turns Rust-looking text back into tokens.

The macro must live in its **own crate** (`[lib] proc-macro = true`). The compiler builds that crate first and *runs* it while compiling yours, so it cannot be part of the code it transforms.

Next two quests: write a derive, then an attribute.
