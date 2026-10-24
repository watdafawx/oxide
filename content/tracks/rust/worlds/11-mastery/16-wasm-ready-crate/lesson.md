The web page should count words as you type, with the same `word_count` the desktop app uses, built for the browser. A JavaScript page cannot call a Rust function directly, so the function needs a tag that makes it callable, and that tag must not get in the way of `cargo test` on your machine.

**An attribute only sometimes.** `#[cfg_attr(condition, attribute)]` adds the attribute only when the condition holds, and is an empty line otherwise. With `target_arch = "wasm32"` as the condition, the browser build gets the tag and the native build never sees it. It is the cousin of last quest's `#[cfg]`, which drops a whole item.

```rust
#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
pub fn line_count(text: &str) -> u32 {
    text.lines().count() as u32
}
```

**What the tag does.** `#[wasm_bindgen]` is an attribute macro from the wasm-bindgen crate. Next to your function it writes **glue**, the small pieces of Rust and JavaScript that pass values across: it turns a JavaScript string into a `&str` on the way in, and a `u32` into a JavaScript number on the way out. Plain types like numbers and text cross easily; anything that opens files or threads does not belong in a browser-facing function.

**Two crate types.** In `Cargo.toml`, `crate-type = ["cdylib", "rlib"]` builds the library twice. An **rlib** is a normal Rust library, the kind tests and other crates link. A **cdylib** is a library in the plain format other languages load; the `wasm-pack` tool turns it into a `.wasm` file plus the JavaScript glue.

**A dependency for one target.** The table `[target.'cfg(target_arch = "wasm32")'.dependencies]` lists wasm-bindgen for browser builds only, so `cargo test` never compiles it. That is also why the attribute names it by full path: on your machine the crate is not there, and the line that mentions it vanishes.

## Your turn

- Write `word_count`: the number of whitespace-separated words, as a `u32`.
- Add the `cfg_attr` line that applies `wasm_bindgen::prelude::wasm_bindgen` only for `wasm32`, right above `pub fn word_count`.
