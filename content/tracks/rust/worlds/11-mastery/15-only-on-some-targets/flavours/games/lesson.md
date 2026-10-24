The chunk indexer should also power a map viewer that runs in a web browser. On a desktop it caches into `.minecraft/saves/`; in a browser there are no folders, only the browser's own storage. So `save_location` needs two versions, and right now both are in the file and the build stops with `error[E0428]: the name save_location is defined multiple times`.

**What a target is.** A **target** is the kind of machine a build is for: its processor and its operating system. Your tests build for the computer you are on, perhaps `x86_64` Linux or Windows. A browser runs **WebAssembly**, a portable format for code, and Rust builds it for the target `wasm32-unknown-unknown`, whose processor family, or `target_arch`, is `wasm32`.

**Compile it only if.** `#[cfg(condition)]` above an item keeps it only when the condition is true for this build; otherwise the compiler drops it before checking anything, as if you had deleted it. You have seen one in every starter: `#[cfg(test)]` keeps the test module in test builds and drops it from the real program. **cfg** is short for configuration.

```rust
#[cfg(target_os = "windows")]
pub fn line_end() -> &'static str {
    "\r\n"
}

#[cfg(not(target_os = "windows"))]
pub fn line_end() -> &'static str {
    "\n"
}
```

**Exactly one survives.** `not(...)` turns a condition around, so of the two `line_end`s above exactly one is kept on any machine, and the name is defined once. Other conditions you will meet: `target_os = "linux"`, `target_arch = "wasm32"`, `test`, and `feature = "..."` for a crate's optional features.

## Your turn

- Mark the `"browser storage"` version so it is only compiled for `target_arch = "wasm32"`.
- Mark the `".minecraft/saves/"` version so it is compiled for every other target.
