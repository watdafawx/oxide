GPUI is the second route for the task manager. It comes from the Zed code editor, which shows thousands of lines and stays fast. It asks a little more of you than egui does: a struct GPUI keeps, and a call when it changes.

**A view is kept.** In GPUI a screen is a struct, a **view**, and GPUI owns it for as long as the window is open. The list of processes lives in the view, not in a function's locals.

**It describes itself.** A view implements `Render`: a `render` function that returns what it holds, built from boxes called `div()`. A box can sit its children side by side with `.flex()`, take a gap, a colour, a size. GPUI lays the boxes out and the graphics card draws them.

**It says when it changed.** GPUI does not call `render` again by itself. When your code changes a field, it calls `cx.notify()`: "I changed, ask me again". Forget it when new numbers arrive on a timer, and the window keeps showing the old list until something else makes GPUI redraw.

**A click is a callback.** Where egui answers "was it clicked?", GPUI takes a closure: `.on_click(..)` on a box, and GPUI runs the closure when the click lands. The closure gets the view back, changes it, and notifies.

```rust
div().id("header-name").child("Name").on_click(cx.listener(|list, _, _, cx| {
    list.sort_by_name();
    cx.notify();
}))
```

**Reading the closure.** `cx` is GPUI's context, your way to talk to GPUI inside `render` and callbacks. `cx.listener` wraps the closure so its first argument is the view (`list`); the two `_` are the click event and the window, unused here. The `.id(..)` is needed for clicks; a later quest shows why.

**Long lists.** `uniform_list` draws only the rows that are on screen, so 400 processes cost what 30 do. GPUI also has keyboard shortcuts built in: an action like "select next row" bound to a key.

**The cost.** GPUI is younger than egui. It has fewer ready-made widgets (you build your own buttons and tables), its API still changes between versions, and the first build takes about 2 minutes and 2 GB.
