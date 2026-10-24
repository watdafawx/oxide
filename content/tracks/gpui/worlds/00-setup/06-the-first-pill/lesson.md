The window is blank because `render` gives back an empty `div()`. Kinetik's bar is a row of small boxes, one per stat, each holding a label and a number. Start with one: inside that empty box, a box that says `CPU 12%`.

**Build by chaining.** `div()` makes an empty box. Each call after it, like `.child(..)`, takes the box, changes one thing and hands the same box back, so the next call can go on: `div().child("RAM 8.0 GB")`. This shape is called a **builder**, and every look the bar gets later (a row, a height, a colour) is one more call in the chain.

**Boxes in boxes.** `.child(..)` takes anything that can become an element: a piece of text, or another box. So a box can hold a box. The outer `div()` is the bar, and each pill is a `div()` inside it.

**A name for tests.** A test cannot look at the screen, so it finds boxes by name. `.debug_selector(..)` gives a box a name, such as `pill-ram`. It changes nothing you can see.

**Why a closure.** `debug_selector` takes a closure, a small recipe for the name, not the name itself. GPUI runs it only when its test tools are switched on (the `test-support` feature from the first build); without them the call does nothing and the name is never built. The closure has to give back a `String`, and `.into()` turns the text `"pill-ram"` into one:

```rust
div().debug_selector(|| "pill-ram".into()).child("RAM 8.0 GB")
```

**Order inside the chain.** The calls above say "a box, named pill-ram, holding RAM 8.0 GB". Swapping the two calls gives the same box: each one changes a different part of it.

## Your turn

- In `render`, keep the outer `div()` and give it one child: a box named `pill-cpu` that holds the text `CPU 12%`.
- Save. Press Run: the window now says CPU 12%.
