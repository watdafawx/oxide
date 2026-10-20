**Why now:** the chunk indexer should also run in a browser map viewer, so its core crate must build for wasm without giving up plain `cargo test`.

WebAssembly lets Rust run in a browser. One crate can serve both: `cargo test` on your machine and a `.wasm` file for JavaScript.

Three pieces make that work:

- **`crate-type = ["cdylib", "rlib"]`** in `[lib]`: `cdylib` is what `wasm-pack` turns into a `.wasm`, `rlib` keeps normal Rust tests and dependents working.
- **`wasm-bindgen`** generates the JS glue. It is declared only for the wasm target in `Cargo.toml`, so native builds never compile it.
- **`#[cfg_attr(target_arch = "wasm32", ...)]`** applies an attribute only on wasm. Everywhere else the line vanishes.

```rust
#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
pub fn chunk_index(x: u32, z: u32) -> u32 {
    x + z * 16
}
```

The function body is ordinary Rust. A sequence where each number comes from the previous ones (Fibonacci: 0, 1, 1, 2, 3, 5, ...) needs no recursion, just two running numbers that you update together in a loop. You can assign a tuple at once, which updates both from their *old* values:

```rust
let (mut x, mut y) = (1u64, 2u64);
for _ in 0..3 {
    (x, y) = (y, x + y);      // x takes y's old value, y takes the sum
}
```

Keep the logic in plain Rust and the wasm attribute as a thin tag on top. Then your tests need no browser.

## Your turn

In `src/lib.rs`:

- Implement `fib(n)` with a loop. It must handle `fib(90)`.
- Add the `cfg_attr` export line above it (the TODO says where).

Look at `Cargo.toml` too: that is where the wasm-only dependency lives.
