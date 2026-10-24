The given `for_each_mod` runs one step for every mod id, and you hand it the step as a closure, as in world 03. But this step must ask the mod server for the jar name, so it has to `.await`. A plain closure can't, so you need one that can.

**Try the obvious thing.** Pass `|id| { let file = jar_name(id).await; .. }`. Rust stops with `error[E0728]: await is only allowed inside async functions and blocks`. The body of a plain closure is plain code.

**An async closure.** Writing `async` in front, `async |id| { .. }`, makes a closure whose body is async: every call gives back a future, and `.await` works inside. A function that takes one writes `impl AsyncFn(u32) -> String`, the async twin of `impl Fn` from world 03.

**Move what it uses.** Like `move |..|`, `async move |id| { .. }` moves the variables it captures into the closure, so it owns them. That lets the closure outlive the function that made it, for example when you return it.

```rust
let mirror = String::from("cdn.modrinth.example");
let fetch = async move |id: u32| {
    download(&mirror, id).await // the closure owns `mirror`
};
let first = fetch(1).await;
let second = fetch(2).await; // call it again: a new future each time
```

**Block or closure.** `async move { .. }` from quest 07 is one future, built right now, with no arguments. `async move |id| { .. }` is a recipe for futures: it takes arguments and builds a new future at every call. Libraries that call your code later with values of their own take the closure, for example GPUI's `cx.spawn(async move |this, cx| { .. })`.

**The older spelling.** Before Rust 1.85 people wrote `|id| async move { .. }`: a plain closure that returns an async block. You will meet it in older code and in quest 17. It works as long as the block doesn't need to borrow from the closure.

## Your turn

- In `tagged`, call `for_each_mod` with an async closure that awaits `jar_name(id)` and adds `"[<loader>] "` in front.
- In `tagger`, return an `async move` closure with the same body, so it owns `loader`.
