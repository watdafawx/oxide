With DLSS or FSR frame generation on, the capture shows 118 FPS, but every second frame was made up. Your mouse and keyboard still act at the speed of the game's own frames, so Kinetik shows that number too: base FPS.

**Same time, fewer frames.** Base FPS divides by the same seconds as FPS: the whole run, every frame's time added up. Only the count changes: just the frames whose `generated` is `false`.

**Seconds from frames.** The frames are `Frame`s, not plain numbers, so pick the field first: `frames.iter().map(|f| f.ms)` gives the frame times, and `.sum::<f64>()` adds them. The `::<f64>` tells `sum` the type, since nothing else here does. As in FPS, no time gives `0.0`.

**Filter, then count.** `filter` keeps the frames its closure says yes to, and `count()` counts them, as a `usize`. Here the same pair counts the frames that took longer than 20 ms; yours keeps the game's own frames with `!f.generated`, read "not generated":

```rust
let slow = frames.iter().filter(|f| f.ms > 20.0).count();
```

**What it tells you.** FPS 118, base 59: frame generation doubles the picture, and the game feels like 59.

**Live, for now.** The live presents in the next quest carry no frame type, so the live overlay shows base FPS equal to FPS. Kinetik reads the real split from NVIDIA Reflex; in a capture it is a column.

## Your turn

- In `base_fps`, work out the seconds from every frame's time, and give back `0.0` for none.
- Give back the count of the frames that are not generated, as an `f64`, over those seconds.
- Save.
