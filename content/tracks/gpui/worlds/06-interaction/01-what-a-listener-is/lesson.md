This world makes the bar answer the mouse: a click opens Task Manager, resting on a stat shows a card, a right click opens a menu. Each of those is a **listener**, a closure GPUI keeps and runs later, when the thing happens. They all have the same shape, so read it once before writing any.

**Something to run later.** `.on_click(..)` runs nothing while the bar is drawn. It stores a closure, and GPUI calls it when a click arrives: in a second, in an hour, or never. A closure is a small function written in place, from [Rust world 03](/quest/rust/03-collections/09-a-rule-without-a-name).

**Four inputs, one order.** `cx.listener(..)` makes a listener from a closure with four inputs, always in this order: `this`, the `StatsBar` you may change; the event, which says what happened (for a click, a `ClickEvent`); the `window` it happened in; and `cx`, the bar's `Context`, which you need for `cx.notify()`. From one handler to the next, only the event changes.

```rust
cx.listener(|this, event: &ClickEvent, window, cx| {
    // this: &mut StatsBar       event: what happened
    // window: &mut Window       cx: &mut Context<StatsBar>
})
```

**Skip with `_`.** A handler that does not need an input names it `_`. A click on a pill needs only the bar and `cx`, so it is written `|this, _, _, cx|`. GPUI knows the event's type from the method you hand the listener to.

**Why a wrapper.** `interactive_pill` runs while the bar is being drawn, and it borrows the bar to do that. The click comes much later, after that borrow has ended. `cx.listener` remembers which bar it belongs to and hands it back to you as `this` when the click arrives, so the closure never has to hold on to the bar itself.

**`move` for the rest.** The closure is kept after the function that made it returns, so it cannot borrow that function's local variables, such as the pill's `kind`. `move` makes it take its own copy. `StatKind` is `Copy`, so the copy is free and `kind` stays usable for the next listener on the same pill; a `String` would have to be cloned first.

**Under the hood.** GPUI builds every element again on every frame: the `div` that `render` returns is thrown away once it is drawn. Listeners are attached again each frame too, which is one more reason they must not borrow anything from the frame that made them.

## Your turn

Answer the questions, then start the next quest.
