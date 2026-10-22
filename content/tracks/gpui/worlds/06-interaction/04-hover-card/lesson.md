Kinetik's hover card shows the stat's last minute: a graph, the minimum, average and maximum, and the processes using the most. It appears when the mouse rests on a stat and goes when it leaves. GPUI calls that a tooltip, whatever it holds.

**Cards are views.** `HoverCard` holds a title, the summary line, the history and a process list, and implements `Render`. `card_for(bar, kind)` fills it from the bar: the summary is the `hover_text` you wrote in Sensors.

**Built when needed.** `.tooltip(..)` takes a closure that builds the card only when it is about to show, after the mouse has rested for half a second. It hands back an `AnyView`; `cx.new(..)` makes the card an entity and `.into()` turns that into the view:

```rust
.tooltip(move |_, cx| { let card = card.clone(); cx.new(|_| card).into() })
```

**Clone inside.** The closure may run each time you hover the pill, but it owns one `card`. Each run clones it, so every tooltip gets its own copy.

**Half a second.** GPUI waits 500 ms before showing a tooltip, so sweeping the mouse across the bar does not flash cards. The test moves its clock past that.

## Your turn

- In `interactive_pill`, add `.tooltip(move |_, cx| ..)` that clones `card` and gives back `cx.new(|_| card).into()`.
- Save. Press Run and rest the mouse on a pill.
