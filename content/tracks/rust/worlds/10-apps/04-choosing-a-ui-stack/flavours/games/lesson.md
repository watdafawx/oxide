**Why now:** the mod database is ready and needs a face. Before writing screens you pick a UI stack, because it decides how you structure state, how you test and how much you can reuse.

| Stack | Idea | Good for |
|---|---|---|
| **egui** | **Immediate mode**: every frame your code describes the whole UI from your state. | Tools and prototypes, tiny deps, easy to test without a window. |
| **GPUI** | **Retained mode**: view objects live on, and a view re-renders when notified. | Polished native apps (it powers the Zed editor), fine-grained updates. |
| **Tauri** | A system **webview** shows HTML/JS (Svelte, React); Rust runs native commands. | Reusing a web UI, rich styling, small installers. |

```rust
// egui: the UI is a function of state, run every frame
ui.label(format!("{} mods enabled", state.enabled));
if ui.button("Disable all").clicked() {
    state.disable_all();
}
```

- Whatever you pick, keep the **logic out of the drawing code**: a view model you can unit test.
- Immediate mode has no widget objects to keep in sync, so there is less state to get wrong.
- Retained mode redraws only what changed, and wins on huge, complex screens.
- Tauri splits the app in two languages. The seam is the commands (next quests).
- Pick by what you already have (web UI? go Tauri) and what you want to learn.
