Oxide's screen is a web page in a Tauri webview. When you finish a quest, the page runs `await invoke("award_xp", { amount: 50 })` and shows the new level. The page can't touch the progress file itself, so Rust has to answer, and this quest writes the Rust side.

**The page can't, Rust can.** A web page runs in a **sandbox**: it can't open files, start programs or reach the database, so a page from anywhere can't harm your computer. Tauri's way through is a **command**: a Rust function the page calls by name with `invoke`, getting the result back as a JavaScript value. Everything native goes through commands.

**Mark a command.** `#[tauri::command]` above a function makes it callable from the page. Arguments arrive by name from the page's object (`{ amount: 50 }` fills the parameter `amount`) and the return value goes back, both through serde. Returning `Result<T, String>` makes an `Err` reject the page's promise, so the page can show the message.

**Shared state.** `.manage(Mutex::new(AppState::default()))` hands Tauri one value at start-up. A command asks for it with a parameter `state: tauri::State<'_, Mutex<AppState>>` and locks it like the database in quest 18; the `'_` is a lifetime Rust works out on its own. The page never sees this parameter; only `amount` comes from it.

**Register them.** `.invoke_handler(tauri::generate_handler![award_xp, start_quest])` builds the table from command names to functions, like a router's routes. A command you forget to list here doesn't exist as far as the page knows: `invoke` fails with "command not found".

```rust
#[tauri::command]
fn set_goal(state: tauri::State<'_, Mutex<AppState>>, quests: u32) -> u32 {
    set_goal_inner(&mut state.lock().unwrap(), quests)
}

pub fn build_app<R: tauri::Runtime>(builder: tauri::Builder<R>) -> tauri::Builder<R> {
    builder
        .manage(Mutex::new(AppState::default()))
        .invoke_handler(tauri::generate_handler![set_goal])
}
```

**Thin wrappers, real tests.** The logic lives in plain `_inner` functions, already written, which any unit test can call. Each command is one line around one of them. `build_app` is generic over `R: tauri::Runtime` (world 05), so the tests can hand it `tauri::test::mock_builder()`: an app with no real window, where the tests' `invoke` helper sends requests exactly as the page would.

**Small print.** Keep command functions private (no `pub`): `#[tauri::command]` also makes a hidden macro with the same name, and a `pub` one at the crate root clashes with it. The first build compiles Tauri's large dependency tree, three to six minutes, and on Linux it needs the system's webview libraries (GTK and WebKit) installed.

## Your turn

- Write the commands `award_xp` (takes `amount: u32`) and `start_quest` (takes `id: String`), each one line around its `_inner` function.
- In `build_app`, register the shared `AppState` and both commands on `builder`.
