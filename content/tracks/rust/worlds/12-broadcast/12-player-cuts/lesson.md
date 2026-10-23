Apollo, ESL's image tool, crops, trims and squares player photos for the HUD in batches. Its library, Jimp, is plain JavaScript. In Rust the same work runs natively, on every core.

**Find the edges.** A player photo is a cut-out on a transparent background. The useful part is the smallest box holding every pixel whose alpha is above 0.

**Crop without copying twice.** `imageops::crop_imm` gives a view of the box; `.to_image()` makes it a new image:

```rust
imageops::crop_imm(img, left, top, right - left + 1, bottom - top + 1).to_image()
```

**Nothing there.** A fully transparent image has no box: `left` is still bigger than `right`. Give back the image unchanged.

**Every core.** `cut_folder` uses rayon: `.par_iter()` instead of `.iter()` splits the files across all cores. Each photo is independent work with no waiting, exactly what rayon is for.

## Your turn

- Write `trim_transparent`: find the box of visible pixels, then crop to it.
- Keep a fully transparent image as it is. Save.
