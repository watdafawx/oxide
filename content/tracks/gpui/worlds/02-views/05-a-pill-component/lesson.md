Kinetik draws the same pill in three places: the bar, the desktop widget and the game overlay. A loose function works, but a named element that carries its own data and its own drawing is easier to move around. GPUI calls that a component.

**The word for it.** A **component** is a struct that becomes an element. `PillView` holds one `Pill`. `#[derive(IntoElement)]` lets you pass it to `.child(..)` like a `div()`.

**Render once.** A component implements `RenderOnce`. Its `render` takes `self` by value: GPUI calls it once per frame, and the struct is used up. The view `StatsBar` keeps its state with `&mut self`; a component keeps none.

**Name the parts.** The value moves around later: it gets a fixed width, then a second line. Give the label and the value their own boxes and names, so tests and later quests can find each:

```rust
.child(div().debug_selector(move || format!("value-{kind:?}")).child(self.pill.text))
```

**Use it.** In the bar, each pill becomes `PillView { pill: pill.clone() }`. The clone is cheap next to drawing, and the component owns its copy.

## Your turn

- In `PillView::render`, add the label box, then the value box, then the text colour.
- In `StatsBar::render`, make `.children(..)` build a `PillView` per pill instead of calling `pill_element`, and delete `pill_element`.
- Save.
