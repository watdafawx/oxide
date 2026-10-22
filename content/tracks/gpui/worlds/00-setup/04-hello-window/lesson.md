The build is done; time to see something. Every GPUI app starts the same way: start the application, open a window, put a view in it. `main.rs` already does all three. You decide how big the window is.

**Start the app.** `Application::new().run(|cx| { ... })` starts GPUI and calls your closure once it is ready. `cx` is the app's **context**: the handle you open windows and read settings with.

**Open a window.** `cx.open_window(options, ...)` opens one. The options say where it goes, how big it is, and the title, which comes from your `app_title`.

**Close means quit.** `cx.on_window_closed(|cx| cx.quit())` ends the app when the window closes. Without it, the app keeps running with no window. `.detach()` keeps that rule alive for the whole run.

**Sizes are pixels.** GPUI sizes are not bare numbers. `px(360.)` is 360 pixels, and `size(width, height)` pairs two of them:

```rust
size(px(360.), px(120.))
```

## Your turn

- In `lib.rs`, change `bar_size` to 360 wide and 120 high.
- Save. Press Run: an empty window titled Kinetik Stats opens. Close it to stop the app.
