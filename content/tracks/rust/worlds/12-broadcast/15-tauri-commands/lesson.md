rl-hud is an Electron app: the React dashboard plus a whole Chromium and Node.js, about 100 MB to install. Tauri keeps the same React pages but uses Windows' own WebView2, with Rust behind it. The installer is about 10 MB.

**Pages call commands.** A page calls `invoke("hud_state")`; Tauri runs the Rust function marked `#[tauri::command]` and sends its return value back as JSON.

**Managed state.** `build_app` gives Tauri one `Mutex<HudState>` with `.manage(..)`. Any command can ask for it by type as `tauri::State`:

```rust
pub fn hud_state(hud: tauri::State<'_, SharedHud>) -> HudState {
```

**Poisoned locks.** If a thread panicked while holding the lock, `lock()` gives an error. The data is still fine for showing a score, so take it with `into_inner()` instead of crashing the app.

**No window needed.** Tests build the app on Tauri's mock runtime (`features = ["test"]`) and call commands the way a page would, as in the Apps world.

## Your turn

- In `hud_state`, lock the managed state and give back a clone.
- Save.
