With DLSS or FSR frame generation on, the capture shows 118 FPS, but every second frame was made up. Input still moves at the speed of the game's own frames, so Kinetik shows that too: base FPS.

**Same time, fewer frames.** Base FPS divides by the same seconds as FPS: the whole run. Only the count changes: frames that are not `generated`.

**Filter, then count.** `filter(|f| !f.generated)` keeps the game's own frames, and `count()` counts them:

```rust
frames.iter().filter(|f| !f.generated).count() as f64 / seconds
```

**What it tells you.** FPS 118, base 59: frame generation doubles the picture, and the game feels like 59. Kinetik gets this live from NVIDIA Reflex; in a capture it is a column.

**No column, no base.** Live presents carry no frame type here, so the live overlay shows base FPS equal to FPS; Kinetik reads the real split from NVIDIA Reflex.

## Your turn

- In `base_fps`, work out the seconds from every frame's time, `0.0` for none.
- Give back the count of frames that are not generated, over those seconds.
- Save.
