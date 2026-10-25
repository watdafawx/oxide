Kinetik draws the same pill in three places: the bar, the desktop widget and the game overlay. A loose function works, but a named element that carries its own data and its own drawing is easier to move around. GPUI calls that a component.

**The word for it.** A **component** is a struct that becomes an element. `PillView` holds one `Pill`. `#[derive(IntoElement)]` writes the code that lets you pass it to `.child(..)` or `.children(..)` like a `div()`.

**Render once.** A component implements `RenderOnce`. Its `render` takes `self` by value: GPUI calls it once while it builds the frame, and the struct is used up. Because `self` is yours, its fields move out: `self.pill.text` goes into `.child(..)` with no clone.

**View or component.** A view (`Render`, `&mut self`) is kept by GPUI between frames and has state that changes: the bar. A component is a throw-away value that a view's `render` builds each frame from data it hands over: a pill. If the thing has nothing of its own to remember, make it a component.

**Two kinds of cx.** The bar's `render` gets `cx: &mut Context<Self>`, the app plus the means to speak for this view, such as telling GPUI it changed. A component's gets `cx: &mut App`, the whole app and nothing more, because it is not a view and has no state to report. The Live world lays out the whole family.

**Use it.** The starter's bar already builds one `PillView { pill: pill.clone() }` per pill, so for now every pill is an empty box. The clone is cheap next to drawing, and the component owns its copy.

**The shape.** A component for a blank gap of a given width, to show the shape:

```rust
#[derive(IntoElement)]
struct Gap { width: Pixels }

impl RenderOnce for Gap {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div().w(self.width)
    }
}
```

## Your turn

- In `PillView::render`, after `.gap_1()`, draw what `pill_element` draws: the named label box, the named value box and the text colour, all from `self.pill`.
- Delete `pill_element`, which nothing uses any more.
- Save. Press Run: the full bar is back, drawn by `PillView`.
