**Why now:** Oxide itself is a Tauri app, so its mod and XP logic reaches the UI as `#[tauri::command]` wrappers around plain, testable functions.

A Tauri app is a web UI that calls Rust **commands**. The trick to keeping this testable: put the logic in plain functions and make each command a **thin wrapper**.

- `#[tauri::command]` turns a function into something the UI can `invoke("name", { args })`. Arguments and the return value go through serde.
- Shared state is registered once with `.manage(Mutex::new(AppState::default()))` and injected as a parameter: `state: tauri::State<'_, Mutex<AppState>>`.
- Return `Result<T, String>`: `Err` becomes a rejected promise in JS.
- `tauri::State` can only be built by Tauri, so test the *plain* function (`fn bump_inner(&mut AppState)`) in ordinary unit tests.
- To prove the wiring too, use `tauri::test::mock_builder()`: a fake runtime with no window and no webview, where `get_ipc_response` sends an invoke request and hands back the reply.

```rust
#[tauri::command]
fn bump(state: tauri::State<'_, Mutex<AppState>>, by: u32) -> u32 {
    bump_inner(&mut state.lock().unwrap(), by)
}

fn builder<R: tauri::Runtime>(b: tauri::Builder<R>) -> tauri::Builder<R> {
    b.manage(Mutex::new(AppState::default()))
        .invoke_handler(tauri::generate_handler![bump])
}
```

This lib has no window and no `tauri.conf.json`, so once tauri itself is built (the first build compiles a large dependency tree, about 3-6 min) it tests in seconds. `build_app` and the mock test are done for you.

## Your turn

In `src/lib.rs`:

- `level_for` (one level per 100 XP, starting at 1), `award_xp_inner` and `start_quest_inner`: plain functions.
- `award_xp` and `start_quest`: the two commands, each one line around the matching `_inner` function.
