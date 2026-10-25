Kinetik's Dashboard settings page has a layout editor that switches cards on and off. Nobody with a desktop PC wants a Battery card. The cards follow the bar's Arrange order already; switching one off is new.

**Hidden in Settings.** `Settings::dashboard.hidden` lists the stats whose cards are off. It is saved with everything else, and the dashboard watches the Settings global, so switching a card off redraws the window at once.

**Everything but.** The cards to show are every kind in `ALL` that is not in `hidden`. `filter` keeps the items its closure says `true` for, and `collect()` gathers them into the `Vec` the function returns. Here is the same shape keeping only the percentage stats:

```rust
ALL.into_iter().filter(|kind| kind.unit() == Unit::Percent).collect()
```

**Is it in the list.** `hidden.contains(kind)` is `true` when the list holds that kind. `filter` hands its closure a reference to each item, and `contains` wants a reference too, so `kind` goes in as it is. Put `!` in front to keep the ones that are not hidden.

**One place.** `render` asks `visible_cards` for its list, so the window and, in the boss, anything else that shows cards agree on which ones are on.

**Nothing is lost.** A hidden card keeps its history: the bar still reads the stat, so switching the card back on shows the last minute at once.

## Your turn

- In `visible_cards`, keep every kind in `ALL` that `hidden` does not contain, and collect them.
- Save.
