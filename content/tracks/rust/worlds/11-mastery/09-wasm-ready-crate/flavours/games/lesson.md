**Why now:** the chunk indexer should also run in a web page that previews a world. A browser runs **WebAssembly**, and Rust compiles to it. The trick is making the same crate build natively (for tests) and for the web, without dragging browser-only crates into `cargo test`.

```rust
// The slime farm doubles every round: 1, 2, 4, 8, ...
#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
pub fn doubling(rounds: u32) -> u64 {
    let mut x = 1u64;
    for _ in 0..rounds {
        x *= 2;
    }
    x
}

// two running numbers, updated together from their OLD values:
let (mut p, mut q) = (1u64, 2u64);
for _ in 0..3 {
    (p, q) = (q, p + q);
}
```

```toml
[lib]
crate-type = ["cdylib", "rlib"]     # cdylib for wasm, rlib for tests and other crates

[target.'cfg(target_arch = "wasm32")'.dependencies]
wasm-bindgen = "0.2"                 # compiled only when targeting wasm
```

- `cfg_attr(condition, attribute)` applies an attribute **only** when the condition holds. Here: only on `wasm32` is `#[wasm_bindgen]` added, so native builds never see it.
- `#[wasm_bindgen]` generates the JavaScript glue that lets the browser call your function.
- The target-specific dependency table keeps `wasm-bindgen` out of native builds entirely.
- Keep the **logic pure** (numbers in, numbers out). Anything that touches files or threads does not belong in the wasm-facing API.
- `u64` here is `BigInt` on the JavaScript side.
- The tests run natively and also check the source for the attribute.

## Your turn

In `src/lib.rs`: implement `slimes_after` by iterating, and put the wasm-only `cfg_attr` on it.
