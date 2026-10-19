WebAssembly lets Rust run in a browser. Your crate can serve both worlds: plain `cargo test` on your machine, and a `.wasm` file for JavaScript.

Three pieces make that work:

- **`crate-type = ["cdylib", "rlib"]`** in `[lib]`: `cdylib` is what `wasm-pack` turns into a `.wasm`, `rlib` keeps normal Rust tests and dependents working.
- **`wasm-bindgen`** generates the JS glue. It is declared only for the wasm target in `Cargo.toml`, so native builds never compile it.
- **`#[cfg_attr(target_arch = "wasm32", ...)]`** applies an attribute only on wasm. Everywhere else the line vanishes.

```rust
#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}
```

Keep the logic in plain Rust and the wasm attribute as a thin tag on top. Then your tests need no browser.

## Your turn

In `src/lib.rs`:

- Implement `fib(n)` with a loop. It must handle `fib(90)`.
- Add the `cfg_attr` export line above it (the TODO says where).

Look at `Cargo.toml` too: that is where the wasm-only dependency lives.
