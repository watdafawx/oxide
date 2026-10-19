**Why now:** if the mod manager's screen is a Svelte webview, everything native (downloads, files, the mod database) is a **Tauri command**: a Rust function the page calls by name. Commands tied to a window are hard to test. The trick: write the logic as plain functions on a state struct, make each `#[tauri::command]` a thin wrapper, and call the whole thing through a **mock app** in tests.

```rust
pub fn install_mod_inner(state: &mut AppState, id: String) -> Result<Vec<String>, String> {
    if state.installed.contains(&id) { return Err(format!("{id} is already installed")); }
    state.installed.push(id);
    Ok(state.installed.clone())
}

#[tauri::command]
fn install_mod(state: tauri::State<'_, Mutex<AppState>>, id: String) -> Result<Vec<String>, String> {
    install_mod_inner(&mut state.lock().unwrap(), id)       // thin wrapper
}
```

- State registered with `.manage(Mutex::new(AppState::default()))` reaches commands as `tauri::State<Mutex<..>>`.
- A command returning `Result<T, String>` becomes a resolved or rejected promise in JavaScript. `T` must be `Serialize`.
- The `_inner` functions need no Tauri at all: plain unit tests.
- `tauri::test::mock_builder()` builds an app with no real window, and `get_ipc_response` sends a command through the same path the webview uses.
- Keep command functions private in the crate root (Tauri generates a macro with the same name).

## Your turn

In `src/lib.rs`:

- `tier_for`: tier 1 at 0 MB, +1 per 100 MB.
- `add_download_inner`: add MB to the state, return `PackStats { mb, tier }`.
- `install_mod_inner`: record a mod id; blank or repeated ids are `Err`.
- The two `#[tauri::command]` wrappers.
