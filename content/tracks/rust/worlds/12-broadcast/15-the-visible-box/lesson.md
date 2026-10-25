Apollo, ESL's image tool, trims and squares player photos for the HUD in batches. A player photo is a cut-out on a transparent background, often with a lot of empty space around the player. Before cutting, the tool needs the box that holds the player.

**Four numbers per pixel.** An `RgbaImage` stores each pixel as four bytes: red, green, blue and **alpha**. Alpha is how solid the pixel is, from 0 (fully **transparent**: you see straight through it) to 255 (fully solid). `pixel[3]` is a pixel's alpha, since counting starts at 0.

**Every pixel, with its place.** `img.enumerate_pixels()` walks the image row by row and gives `(x, y, pixel)` for each one: its column, its row, and its four bytes. `x` grows to the right, `y` grows downwards from the top.

**The box.** The visible box is the smallest left, top, right and bottom that hold every pixel with alpha above 0. Each visible pixel can only push an edge outwards: left down to its `x`, right up to its `x`, and the same for top and bottom with `y`.

**Running smallest and largest.** Keep the best so far and update it per item with `min` and `max`. Start the smallest at the biggest possible number, `u32::MAX`, so the first real value always replaces it; start the largest at 0. The same shape on goal times:

```rust
let (mut first, mut last) = (u32::MAX, 0);
for goal in &goals {
    first = first.min(goal.goal_time);
    last = last.max(goal.goal_time);
}
```

**Nothing there.** If no pixel was visible, nothing replaced the starting values, so `left` is still `u32::MAX`, bigger than `right`. That is how you know to give back `None`.

## Your turn

- In `visible_box`, walk every pixel and widen the four edges around each one with alpha above 0.
- Give back `None` when nothing was visible, otherwise `Some((left, top, right, bottom))`. Save.
