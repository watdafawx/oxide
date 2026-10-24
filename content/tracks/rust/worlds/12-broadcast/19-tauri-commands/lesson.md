rl-hud is an Electron app: the React dashboard plus a whole bundled Chromium browser and Node.js, about 100 MB to install. Tauri keeps the same React pages but shows them in the WebView2 browser engine that Windows already has, with Rust behind them. The installer is about 10 MB.

**Pages call commands.** A page calls `invoke("hud_state")`; Tauri runs the Rust function marked `#[tauri::command]` and sends its return value back to the page as JSON.

**Managed state.** `build_app` (already written) hands Tauri one `Mutex<HudState>` with `.manage(..)`; `SharedHud` is the short name for that type. Any command can ask for it by type with a parameter `hud: tauri::State<'_, SharedHud>`, and Tauri fills it in.

**Lock, then copy out.** `hud.lock()` locks the mutex the way it did in the Concurrency world. Give the page a `.clone()` of the state, so the lock is released as soon as the function returns instead of while the page reads it.

**Poisoned locks.** If a thread panicked while holding the lock, `lock()` gives an `Err` ("poisoned"). The data is still fine for showing a score, so take it anyway with `into_inner()` instead of crashing the app. The same pattern on another mutex:

```rust
let goals = goal_log.lock().unwrap_or_else(|poisoned| poisoned.into_inner()).len();
```

**No window needed.** Tests build the app on Tauri's mock runtime (`features = ["test"]`) and call commands the way a page would, as in the Apps world.

## Your turn

- In `hud_state`, lock the managed state, taking it even if the lock is poisoned.
- Give back a clone of the `HudState` inside. Save.
