The next quest gives the bar commands that can come from a menu, the tray icon or a hotkey. GPUI delivers those the way it delivers key presses: to whatever has **focus**. So first, what focus is and how the bar gets it.

**One focused element.** In a window, one element at a time has the keyboard focus: the text box you are typing into, the list whose rows the arrow keys move. Key presses and commands start there.

**A handle for it.** GPUI names "something that can have focus" with a `FocusHandle`. `cx.focus_handle()` makes one. An element joins it with `.track_focus(&handle)`, and `window.focus(&handle)` gives it the focus.

**The bar's focus.** `focus_in` in `ui.rs` does the making and the focusing when the bar starts, and keeps the handle in `ui.focus`. `bar_layer` attaches it to the bar's root:

```rust
let bar = match &this.ui.focus {
    Some(focus) => bar.track_focus(focus),
    None => bar,
};
```

**Reading the match.** `ui.focus` is an `Option`, because a bar made in some tests never calls `focus_in`. Matching on `&this.ui.focus` lends the handle instead of moving it out of the bar. Both arms give back the `Div`: builder methods like `track_focus` take the element and return it, so `bar` is either the same div with focus attached, or unchanged.

**Up the tree.** A command starts at the focused element and travels up through its parents to the window's root. Any element on that path with a handler for it can answer. Web developers know this as event bubbling.

## Your turn

Answer the questions, then start the next quest.
