The next quest builds an on/off switch, `Toggle`, used once per stat on the Stats page. Its click handler looks different from every handler so far: no `cx.listener`, no `this`, and a `cx` of another type. That follows from which context a closure gets, so here are the three you have met, side by side.

**`App`.** `&mut App` is the context of the whole app. It can open windows, read and change globals, and make entities, but it belongs to no view, so it cannot say "redraw this one". `Application::run`'s closure and `open_settings_window` get it.

**`Context<T>`.** `&mut Context<StatsBar>` is an `App` that also knows which view it belongs to. That is why it has `cx.notify()` and `cx.listener(..)`, and why it works anywhere an `&mut App` is wanted. A view's `render` and the closures `cx.listener` makes get one.

**`Window`.** `&mut Window` is one window: its focus, its size, painting, `activate_window`. It is not a context, so it comes as its own input, next to `cx`.

**A component has no `this`.** `Toggle` is a component, not a view: it is built fresh each frame from its fields and keeps nothing between frames, so there is no `Context<Toggle>` to make a `cx.listener` from and no `this` to hand back. Its `render` gets `&mut App`, and its `on_click` takes a plain closure that gets the event, the window and `&mut App`.

**That is enough.** The toggle's job is to change the `Settings` global, and `&mut App` can do that. The bar and the Settings window both watch the global, so both redraw. Who decides *what* a click changes is the page that builds the toggle: it passes in a closure, stored in the field `on_toggle`.

**Boxed, for a field.** Every closure has its own type, so a struct field cannot name it. `ToggleHandler` is a **type alias**, a short name for a long type, and the type is a `Box<dyn Fn(..)>`: a box that can hold any closure with those inputs, the way `Box<dyn Sensor>` holds any sensor:

```rust
pub type ToggleHandler = Box<dyn Fn(bool, &mut Window, &mut App) + 'static>;
```

**`'static`, no borrows.** The toggle keeps the closure until some later click. `+ 'static` promises it borrows nothing that could be gone by then: it may own values (with `move`), but not hold references to locals. It does not mean "lives forever"; `'static` was named in [Rust world 02](/quest/rust/02-ownership/12-owned-or-borrowed-text).

## Your turn

Answer the questions, then start the next quest.
