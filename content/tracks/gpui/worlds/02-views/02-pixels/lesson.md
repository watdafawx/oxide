On the real taskbar Kinetik's bar is exactly as tall as the space it gets: 32 pixels here, never "however tall the text makes it". A fixed height also stops the bar jumping when the font changes.

**Content decides, by default.** A box with no height is as tall as what is inside it: one line of text, 26 pixels in tests. Change the text size and the bar changes with it.

**Say the height.** `.h(..)` sets a box's height. Its argument is a length in pixels, written with `px` like the window size was:

```rust
.h(px(32.))
```

**Centre the text.** A 32-pixel box with 26-pixel text inside leaves the text at the top. The starter already adds `.items_center()`, which centres a flex row's children from top to bottom.

## Your turn

- In `render`, add `.h(px(32.))` to the outer `div()`.
- Save. Press Run: the bar is a fixed strip with the text in the middle.
