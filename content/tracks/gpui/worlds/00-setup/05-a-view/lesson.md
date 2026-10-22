The window is empty. Kinetik's bar is a row of small boxes, one per stat, each holding a label and a number. Start with one: a box that says `CPU 12%`.

**The word for it.** What fills a window is a **view**: a struct that GPUI asks "what do you look like right now?". `StatsBar` is that struct, and `main.rs` now puts one in the window.

**Render answers.** A view implements `Render`. Its one function, `render`, gives back a description of the window's contents. GPUI calls it whenever the view needs drawing.

**Boxes in boxes.** The building block is `div()`, an empty box. `.child(...)` puts something in it: text, or another box. Everything on Kinetik's bar is boxes in boxes.

**A name for tests.** `.debug_selector(|| "pill-cpu".into())` gives a box a name that tests can find. It changes nothing on screen, and it is gone in release builds:

```rust
div().child(div().debug_selector(|| "pill-cpu".into()).child("CPU 12%"))
```

## Your turn

- In `render`, replace `div()` with the line above: an outer box with one named box inside it.
- Save. Press Run: the window now says CPU 12%.
