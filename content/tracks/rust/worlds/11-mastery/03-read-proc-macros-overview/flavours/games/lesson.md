**Why now:** `macro_rules!` is great for patterns. For the chunk indexer you want to write `#[derive(ChunkInfo)]` on a struct and get an impl for free, which needs real Rust code that **reads your code and writes new code**. That is a **proc-macro**.

A proc-macro is a Rust function the compiler runs on a stream of tokens. Three kinds:

| Kind | Written as | Does |
|---|---|---|
| **Derive** | `#[derive(ChunkInfo)]` | adds an `impl` for your struct or enum |
| **Attribute** | `#[count_calls] fn load() {}` | rewrites the item it is attached to |
| **Function-like** | `registry!(...)` | like `macro_rules!`, but with arbitrary Rust inside |

```rust
// in its own crate, with `[lib] proc-macro = true`
use proc_macro::TokenStream;

#[proc_macro_derive(ChunkInfo)]
pub fn derive_chunk_info(input: TokenStream) -> TokenStream {
    // parse `input` with syn, build the impl with quote!, return it
    TokenStream::new()
}
```

- The compiler **builds and runs** the macro before your crate exists, so it must be a separate crate.
- `syn` parses tokens into a syntax tree (`DeriveInput`, `ItemFn`). `quote!` writes tokens back, with `#name` splicing values in.
- Errors in a proc-macro show up as compile errors in the *user's* code. Return good messages.
- Next quests: write a derive, then an attribute macro.
