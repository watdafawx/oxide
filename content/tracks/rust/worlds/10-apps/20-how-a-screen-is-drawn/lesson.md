The engine is done: a database, JSON and an API. Oxide also needs a face, a window with an XP bar, buttons and a quest list. Before you write a screen it helps to know how a screen works at all, because the two common answers lead to very different code.

**Frames.** A screen is a picture the computer redraws many times a second, usually 60. Each redraw is a **frame**. Between frames the program checks for **events** (a click, a key, the mouse moving), changes its state, and draws the next frame. That loop, events then draw, then again, is the **event loop**, and every app with a window has one.

**Widgets.** The pieces you see, a button, a label, a progress bar, a text box, are **widgets**. A UI **toolkit** is the library that knows how to draw widgets, tell which one was clicked, and lay them out in rows and columns.

**Retained mode.** Most toolkits, and web pages, keep a tree of widget objects alive between frames. You create a label once, keep a handle to it, and call `label.set_text("Level 3")` when the XP changes. That is **retained mode**: the toolkit retains the widgets. Fast for huge screens, but now there are two copies of the truth, your data and the widget tree, and keeping them in step is a classic source of bugs.

**Immediate mode.** **egui** does the opposite. Every frame it calls one function of yours, and that function describes the whole screen from your state, from scratch. There is no label object to update: next frame you just draw the label with the new number. A button is a function call that draws the button and tells you, right there, whether it was clicked this frame.

```rust
fn ui(&mut self, ui: &mut egui::Ui) {
    ui.label(format!("Level {}", self.level));    // drawn fresh every frame
    if ui.button("Claim reward").clicked() {       // true on the frame it was clicked
        self.level += 1;
    }
}
```

**Easy to test.** Because the screen is only "your state in, widgets out", a test can run that function with no window and no graphics card, find a widget by its text, click it, and check your state. The egui quests here do exactly that.

**A webview.** **Tauri** takes a third path: the window shows a **webview**, a browser engine without the address bar, and the screen is an ordinary web page in HTML, CSS and JavaScript. The page can't open files or a database (web pages are kept in a sandbox on purpose), so it asks Rust to, by name. Oxide itself is built this way: a Svelte web page in front, Rust behind. Rule of thumb: a small tool or a debug panel, egui; a web UI you already have, Tauri.

## Your turn

Answer the three questions.
