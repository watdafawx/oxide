Kinetik Stats puts a strip of numbers on your taskbar: CPU, GPU, RAM, network, ping. Before anything can be drawn, the code needs a word for each thing it can measure, and the short label the bar writes next to it.

**One type, nine kinds.** `StatKind` is an enum with one variant per thing the bar can show: `CpuUsage`, `CpuTemp`, `RamUsed` and six more. The real Kinetik has 44. Nine is enough to build the whole bar.

**Each kind has a label.** With icons switched off, Kinetik writes a short label instead, like `CPU` or `RAM`. Two kinds can share one: CPU usage and CPU temperature both say `CPU`, and the number next to it tells them apart.

**One arm per kind.** `label` is a `match` on `self`. Three arms are written, and the `_` arm catches the rest with `?` for now:

```rust
StatKind::GpuUsage => "GPU",
```

**Then drop the catch-all.** Once all nine arms are there, delete `_ => "?"`. If a tenth kind is added later, the compiler then points at this `match` until the new kind gets a label too.

## Your turn

- In `label`, add an arm for each missing kind: `GpuUsage` and `GpuTemp` say `"GPU"`, `NetDown` and `NetUp` say `"NET"`, `Ping` says `"PING"`, `Battery` says `"BAT"`.
- Delete the `_ => "?"` arm.
- Save. Press Run to see every label.
