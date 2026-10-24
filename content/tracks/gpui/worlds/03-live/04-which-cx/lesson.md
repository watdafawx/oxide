You have met `cx` four ways now: in `main.rs`, in a view's `render`, in a component's `render`, and inside `update`. The next quests add one more. They are all called `cx` because they all lead to the app, but each type says what you may do with it right now.

**`App`: the whole app.** `&mut App` is the app itself: open windows, make entities with `cx.new`, quit. `main.rs` gets one from `Application::new().run(..)`, and a component's `render` gets one, because a component has no state of its own to speak for.

**`Context<T>`: the app, for one entity.** `&mut Context<StatsBar>` is the app while you hold the bar: in the bar's `render`, inside `bar.update(..)`, in the closure of `cx.new`. It can do everything `App` can, plus things only an entity can do: `cx.notify()`, which needs to know whose state changed, and, next quest, `cx.spawn`.

**`Window`: this one window.** `render` also gets `window: &mut Window`, the window being drawn: its size, whether it is light or dark, which box has the keyboard. The bar ignores it until the Taskbar world, so the starter writes `_window`.

**`AsyncApp`: the app, later.** Code that runs later, in a spawned task, cannot hold `&mut App` while it waits, because other code runs in between. It gets an `AsyncApp` instead, a handle that asks for the app each time it is used. By then the app or the bar may be gone, so those calls return a `Result` that can fail.

**Test contexts.** Tests get a `TestAppContext`: an app with a fake clock and no screen. `add_window_view` gives a `VisualTestContext`, which adds a window, and that is where `debug_bounds` finds your named boxes.

**Why so many.** Each type lends exactly what is safe at that moment, which is Rust's borrowing rules applied to a whole app. When a call does not exist on your `cx`, the type is telling you that you are in the wrong place to make it.
