Kinetik groups stats with dividers: a thin line, a dotted line, a small dot, or plain blank space. The bar now draws every `BarItem`, pills and dividers alike, so the three line dividers of the default order show up. Each style needs its own width.

**A divider is a box.** `divider` draws one: a 16-pixel line, a dashed border, a round dot, or nothing at all. `.flex_none()` stops the row from squeezing it. Its width comes from `divider_width`.

**Width by style.** Lines are 1 pixel wide, dotted lines too, a dot is 4, and blank space is 8: wide enough to read as a gap. A `match` with one arm per style, each giving back a length:

```rust
DividerStyle::Dot => px(4.),
```

**Two kinds in one list.** A pill and a divider are different elements. The starter's `render` turns each into `AnyElement` with `.into_any_element()`, so both fit in one `.children(..)`.

## Your turn

- In `divider_width`, replace the body with a `match` on `style`: Line and Dotted `px(1.)`, Dot `px(4.)`, Blank `px(8.)`.
- Save. Press Run: three thin lines split CPU, GPU, RAM and network.
