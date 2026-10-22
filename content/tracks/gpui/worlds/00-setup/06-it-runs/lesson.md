You pressed Run and a window said CPU 12%. Before the bar grows, it helps to know what happened in between, because every later quest hooks into the same steps.

**Build, then start.** Run builds your project and starts the program, which begins at `main`. `main` hands control to GPUI with `Application::new().run(...)`.

**GPUI keeps going.** `run` does not return while the app is alive. It waits for things to happen: a mouse move, a resize, a timer. That waiting is the app's **event loop**.

**Render when needed.** When the window first opens, and whenever something changes, GPUI calls your view's `render` and draws what it describes. Nothing else touches the screen.

**Closing ends it.** `main.rs` registered `cx.on_window_closed(|cx| cx.quit())`. Closing the window runs that closure, `quit` ends the event loop, and the program exits. Kinetik itself never quits this way: its bar has no close button, and you leave from the right-click menu.

**What comes next.** The next world builds the model behind the bar: nine kinds of stat, how each is written, when one turns red. It is plain Rust with no window, so its quests build in seconds.
