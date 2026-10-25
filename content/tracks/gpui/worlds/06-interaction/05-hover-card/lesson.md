Kinetik's hover card shows the stat's last minute: a graph, the minimum, average and maximum, and the processes using the most. It appears when the mouse rests on a stat and goes when it leaves. GPUI calls that a tooltip, whatever it holds.

**Cards are views.** `HoverCard` holds a title, the summary line, the history and a process list, and implements `Render`. `card_for(bar, kind)` fills it from the bar: the summary is the `hover_text` you wrote in Sensors.

**Built when needed.** `.tooltip(..)` takes a closure that builds the card only when it is about to show. It must give back an `AnyView`, a view of any type. `cx.new(|_| card)` makes the card an entity, as in Live, and `.into()` turns that entity into the `AnyView`.

**Not a listener.** The tooltip's closure gets the window and `cx`, but no `this`: it only builds something to show and changes nothing on the bar, so it is a plain closure, not a `cx.listener`. A tooltip that always showed the same empty card would be:

```rust
.tooltip(|_, cx| cx.new(|_| HoverCard { title: "CPU".into(), summary: String::new(), points: vec![], top: vec![] }).into())
```

**Clone inside.** Your closure takes `card` with `move`, and GPUI may call it on every hover. `cx.new(|_| card)` would use the card up on the first call, leaving nothing for the second, so the compiler refuses it. Clone it first: each call gets its own copy.

**Half a second.** GPUI waits 500 ms before showing a tooltip, so sweeping the mouse across the bar does not flash cards. The test moves its clock past that.

## Your turn

- In `interactive_pill`, add `.tooltip(move |_, cx| { .. })`.
- Inside, clone `card`, and give back `cx.new(..)` of the copy turned into a view.
- Save. Press Run and rest the mouse on a pill.
