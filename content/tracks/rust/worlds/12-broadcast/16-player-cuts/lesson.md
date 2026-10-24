With the visible box found, a player cut is a crop, a square and a resize. `square`, `cut` and `cut_folder` are already written; the missing piece is the crop, and `cut_folder` then runs it over a whole folder on every core.

**Crop without copying twice.** `imageops::crop_imm(img, x, y, width, height)` gives a view of that part of the image, without copying anything yet. `.to_image()` then makes it a new image of its own. Cutting a 64 x 64 corner off a team banner:

```rust
let corner = imageops::crop_imm(&banner, 0, 0, 64, 64).to_image();
```

**Both edges count.** The box's `right` and `bottom` are the last visible pixels, included. A box from 20 to 49 is 30 pixels wide, not 29, so the width is `right - left + 1`, and the same for the height.

**Nothing there.** For a fully transparent image `visible_box` gives `None`; there is no box to crop to, so give back the image unchanged with `img.clone()`. A `match` on the `Option` handles both: the `Some` arm unpacks the four edges.

**Every core.** `cut_folder` uses rayon: `.par_iter()` instead of `.iter()` splits the files across all cores. The closure then runs on several threads at once, so it must be safe to run that way, the `Send` and `Sync` rules from the Concurrency world. Each photo is independent work with no waiting, exactly what rayon is for. In JavaScript, Apollo's library Jimp runs on one thread; here every core cuts.

## Your turn

- In `trim_transparent`, match on `visible_box(img)`: crop to the box, with width and height that include both edges.
- A fully transparent image stays as it is. Save.
