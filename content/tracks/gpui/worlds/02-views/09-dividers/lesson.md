Kinetik groups stats with dividers: a thin line, a dotted line, a small dot, or plain blank space. The bar now draws every `BarItem`, pills and dividers alike, so the three line dividers of the default order show up. Each style needs its own width.

**Dividers are boxes.** `divider` draws one: a 16-pixel line, a dashed border, a round dot, or nothing at all. `.flex_none()` stops the row from squeezing it. Its width comes from `divider_width`.

**Width by style.** Lines are 1 pixel wide, dotted lines too, a dot is 4, and blank space is 8: wide enough to read as a gap. That is a `match` with one arm per style, each giving back a length. `divider` itself picks heights the same way, a dot short and the rest 16 pixels tall:

```rust
let height = match style {
    DividerStyle::Dot => px(4.),
    _ => px(16.),
};
```

**Width, not padding.** The space between items comes from the bar's `.gap_2()`; a divider's width is only its mark. A blank 8-pixel divider plus two gaps reads as a clear break.

**One type per list.** An iterator gives one type, but a pill is a `PillView` and a divider is a `Div`. `.into_any_element()` turns either into an **`AnyElement`**: an element whose exact type is forgotten, the way `Box<dyn Trait>` forgets a type in the Traits world. The starter's `render` does that in each `match` arm, so both fit in one `.children(..)`.

**Every name once.** `render` numbers the items with `.enumerate()`, and `divider(i, ..)` names each divider `divider-<i>`. The three line dividers look the same, so the number is what keeps their names apart for a test.

## Your turn

- In `divider_width`, replace the body with a `match` on `style` that gives each style its width.
- Save. Press Run: three thin lines split CPU, GPU, RAM and network.
