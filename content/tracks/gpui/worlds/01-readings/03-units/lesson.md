`47`, `8589934592` and `3565158` are all plain numbers to the computer. On the bar they should read `47°C`, `8.0 GB` and `↓ 3.4 MB/s`. Before anything can be written that way, each kind has to say what its numbers mean.

**The word for it.** That meaning is the **unit**: percent, degrees Celsius, bytes, bytes per second, or milliseconds. `Unit` is a small enum with exactly those five variants.

**Each kind has one.** `StatKind::unit` answers "what are this kind's numbers in?". CPU usage is a percentage, RAM used is bytes, ping is milliseconds. The usage kinds and RAM are already written.

**Same shape as `label`.** It is a `match` with one arm per kind, like the `label` you wrote:

```rust
StatKind::CpuTemp => Unit::Celsius,
```

**Why not text.** Two kinds share each unit, and next quests write one formatter per unit. A new kind then only needs one arm here to be written correctly everywhere.

## Your turn

- In `unit`, add arms: `CpuTemp` and `GpuTemp` are `Unit::Celsius`, `NetDown` and `NetUp` are `Unit::BytesPerSec`, `Ping` is `Unit::Millis`.
- Delete the `_ => Unit::Percent` arm.
- Save.
