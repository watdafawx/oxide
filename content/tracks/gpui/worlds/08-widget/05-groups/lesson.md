The bar is a line; the widget has room. Kinetik's widget groups its stats under headings, in a fixed order, so you always find GPU temperature in the same place.

**Six headings.** `Group` lists them and `GROUPS` gives their order, top to bottom. The widget's `render` walks `GROUPS`, picks the stats of each, and skips a heading with none.

**Every kind has one.** `group_of` answers "which heading?". CPU usage and temperature are Processor; GPU usage and temperature are Graphics; download, upload and ping are Network:

```rust
StatKind::CpuUsage | StatKind::CpuTemp => Group::Processor,
```

**No catch-all.** List every kind; no `_` arm. When a new stat arrives, the compiler stops at this `match` until it has a heading.

**Model names later.** Kinetik puts your CPU and GPU model next to the headings. That is a sysinfo call away (`cpus()[0].brand()`), and left out here to keep the quest to one idea.

## Your turn

- In `group_of`, write the `match` with an arm per group, joining kinds with `|`.
- Save. Press Run: the widget shows headings.
