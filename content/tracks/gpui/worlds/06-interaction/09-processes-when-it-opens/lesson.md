`top_five` picks the biggest processes, but the card still lists none. The list must be read when the card opens, not on every frame, and in tests it must come from a fixed list, not from whatever runs on your computer.

**A function in a field.** `bar.ui.top_processes` holds a function, not a list. Its type, `fn() -> Vec<ProcessUse>`, means "a function with no inputs that gives a list of processes". `main.rs` puts `real_top_processes` there; the tests put in a closure that always gives `game.exe` and `browser.exe`. The card does not care which.

**Call it with brackets.** `self.ui.top_processes()` would look for a *method* named `top_processes` and fail to compile. To call the function a field holds, put brackets around the field first, then the call's brackets:

```rust
let list = (self.ui.top_processes)();
```

**Copied out first.** The tooltip closure runs later, when `bar` is no longer lent to `interactive_pill`, so it cannot reach `bar.ui`. A function value is `Copy`, so the starter copies it into a local `top_processes` before the chain, and the `move` closure takes that copy along. Calling a local works the same way: `(top_processes)()`.

**A fresh list each time.** Inside the tooltip closure, clone the card into a `let mut` copy, set its `top` to what the function gives, and show that copy. Every hover asks again, so the list is always current.

**For the curious.** Rust can write "a copy of `card` with `top` changed" as one expression, called **struct update syntax**: `HoverCard { top: list, ..card.clone() }`. The `..` fills every field you did not name from the value after it. The starters of the next quests write it that way.

## Your turn

- In the tooltip closure in `interactive_pill`, make the copy of `card` mutable.
- Set its `top` to what calling `top_processes` gives, then show it as before.
- Save. Press Run and rest the mouse on a pill.
