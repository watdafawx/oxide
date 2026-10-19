Three Rust-friendly ways to put a window on screen. They differ in **where the UI lives** and **how it updates**. Pick by what you already have and how big the app is.

| | egui | GPUI | Tauri |
|---|---|---|---|
| Style | **immediate mode** | **retained mode** | web view + Rust backend |
| UI is | a function `ui()` run every frame | views (structs) that return elements | HTML / CSS / JS |
| State | plain fields in your app struct | fields in views and models, `cx.notify()` to redraw | JS store, plus Rust state behind commands |
| Great at | tools, debuggers, prototypes, game overlays | big native apps with lots of lists and text (Zed, Hopper) | reusing web skills, rich styling, installers |
| Cost | simple widgets, you redraw all of it | steeper learning curve, young ecosystem | two languages, a webview per window |

**Immediate mode** means there is no widget tree to keep in sync. Your state is the truth, and each frame you say "if a button, here; label with this text". A click is returned from the call: `if ui.button("+").clicked() { self.n += 1 }`. That also makes it easy to test.

```rust
fn ui(&mut self, ui: &mut egui::Ui) {
    if ui.button("+1").clicked() {
        self.count += 1;
    }
    ui.label(format!("count: {}", self.count));
}
```

**Retained mode** keeps the views between frames and only repaints what changed. More bookkeeping, less work per frame when the screen is huge.

**Tauri** is not a toolkit: it ships your web UI in the system webview and gives it `#[tauri::command]` functions to call. Oxide itself is Tauri + Svelte.

Rule of thumb: a small tool, egui. A big native app you will live in for years, GPUI. A web UI already exists, or designers need CSS, Tauri.

## Your turn

Answer the two questions below.
