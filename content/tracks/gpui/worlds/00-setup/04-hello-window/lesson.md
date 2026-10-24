The build is done; time to see something. Every GPUI app starts the same way: start the application, open a window, put something in it. `main.rs` already does all three. You decide how big the window is.

**Start the app.** `Application::new().run(|cx| { ... })` starts GPUI and calls your closure once it is ready. `cx` is the app's **context**, a handle to the running app: you open windows and read settings through it. Its type here is `App`; you will meet a few more kinds of `cx`, each a handle to part of the app.

**Open a window.** `cx.open_window(options, ...)` opens one. `WindowOptions` says where it goes, how big it is and its title; `..Default::default()` fills every field you did not name with its usual value. `Bounds::centered` puts a window of `bar_size()` in the middle of the screen.

**Close means quit.** `cx.on_window_closed(|cx| cx.quit())` ends the app when the window closes; without it, the app keeps running with no window. The call hands back a **subscription**, and dropping a subscription cancels it. `.detach()` lets it go without cancelling it, so the rule lasts for the whole run.

**Sizes are pixels.** GPUI sizes are not bare numbers. `px(..)` turns a number into `Pixels`, and `size(width, height)` pairs two of them into a `Size<Pixels>`. A box 200 wide and 50 high:

```rust
size(px(200.), px(50.))
```

**Why the dot.** `px` takes an `f32`, because layout works in fractions of a pixel on high-DPI screens. `200.` is the short way to write the decimal number `200.0`; a bare `200` is an integer, and Rust does not turn it into a float for you.

## Your turn

- In `lib.rs`, make `bar_size` give back a size 360 wide and 120 high.
- Save. Press Run: an empty window titled Kinetik Stats opens. Close it to stop the app.
