On the real taskbar Kinetik's bar is exactly as tall as the space it gets: 32 pixels here, never "however tall the text makes it". A fixed height also stops the bar jumping when the font changes.

**Content decides, by default.** A box with no height is as tall as what is inside it: one line of text, 26 pixels in tests. Change the text size and the bar changes with it.

**Say the height.** `.h(..)` sets a box's height and `.w(..)` its width. Each takes a length in pixels, written with `px` like the window size was. The dot in `px(32.)` makes `32.` a decimal number, `32.0`, because GPUI measures pixels as `f32`. A box 360 pixels wide:

```rust
div().w(px(360.))
```

**Centre the text.** A 32-pixel box with 26-pixel text inside leaves the text at the top. The starter already adds `.items_center()`, which centres a flex row's children from top to bottom.

**Why not the window.** Here the window is taller than the bar; on the taskbar it will be exactly the bar. The bar's own height keeps it right in both.

## Your turn

- In `render`, give the outer `div()` a height of 32 pixels.
- Save. Press Run: the bar is a fixed strip with the text in the middle.
