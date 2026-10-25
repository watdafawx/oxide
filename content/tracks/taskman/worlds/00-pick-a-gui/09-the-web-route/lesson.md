The third route skips Rust GUI libraries altogether: the screen is a web page, and Rust does the work behind it. Oxide, the app you are reading this in, is built this way, with Tauri and Svelte (a JavaScript framework for the page).

**A browser inside the window.** Tauri opens a window with a **web view** in it: the browser engine Windows 11 already has, called WebView2 (the same one as Edge; Tauri's installer adds it where it is missing). The page is ordinary HTML, CSS and JavaScript, packed into your exe.

**Rust behind a command.** The page cannot read the process list itself. It calls a Rust function marked `#[tauri::command]`, and the answer comes back as JSON. Rust can also push events to the page, like "here are this second's numbers".

```rust
#[tauri::command]
fn processes() -> Vec<ProcessRow> { /* ask Windows */ }
```

**What it buys.** CSS makes any look you can imagine, the browser's developer tools show every box, and the web has a component for everything: tables, charts, menus. A designer who knows CSS can work on it without touching Rust.

**What it costs.** WebView2 is a browser: it starts several processes of its own, and they need memory before your page shows a single row. Every refresh crosses from Rust to JavaScript as JSON. And you now work in two languages.

**Rust in the page.** Leptos and Dioxus let you write the page in Rust too. Leptos compiles it to WebAssembly (code a browser can run, like a small exe for the page) that runs inside Tauri's web view; Dioxus runs your Rust as a normal program and sends each change to a web view. One language again, but the web view and its memory stay. (Dioxus is also building a renderer without a web view, called Blitz; it is still early.)

**Where it shines.** An app whose look matters most, a team that knows the web, or a screen that must also open in a normal browser, like a dashboard you check from your phone.
