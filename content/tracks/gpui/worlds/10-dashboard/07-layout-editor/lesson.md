Kinetik's Dashboard page has a layout editor that "reorders cards and their stats and switches them on or off". The order follows the bar's Arrange list here; switching off is new.

**Hidden in Settings.** `Settings::dashboard.hidden` lists the stats whose cards are off. It is saved with everything else, and the dashboard watches the global, so a change redraws at once.

**Everything but.** The cards to show are every kind in `ALL` that is not in `hidden`. A `filter` with `contains`:

```rust
ALL.into_iter().filter(|kind| !hidden.contains(kind)).collect()
```

**One place.** `render` asks `visible_cards` for its list, so the window and, in the boss, anything else that shows cards agree.

**Nothing is lost.** A hidden card keeps its history: the bar reads the stat anyway when it is on the bar, so switching the card back on shows the last minute at once.

## Your turn

- In `visible_cards`, give back every kind in `ALL` that `hidden` does not contain.
- Save.
