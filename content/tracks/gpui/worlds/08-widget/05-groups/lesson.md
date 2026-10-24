The bar is one line; the widget has room. Kinetik's widget groups its stats under headings, in a fixed order, so GPU temperature is always in the same place when you glance at it.

**Six headings.** `Group` lists them: Processor, Graphics, Memory, Network, Storage, Battery. `GROUPS` gives their order, top to bottom. The widget's `render` walks `GROUPS`, picks the stats of each, and skips a heading with none.

**Every kind has one.** `group_of` answers "which heading does this stat go under?". CPU usage and temperature are Processor; GPU usage and temperature are Graphics; RAM is Memory; download, upload and ping are Network; free disk is Storage; the battery is Battery.

**Two kinds, one arm.** Several kinds share a heading, and a `match` arm can list them joined with `|`, read as "or". It is the same `|` the Taskbar world used for light and dark. Here is a different `match` on the widget's style, two styles per arm:

```rust
match style {
    WidgetStyle::Bars | WidgetStyle::Both => true,
    WidgetStyle::Graphs | WidgetStyle::Minimal => false,
}
```

**No catch-all.** List every kind; write no `_ =>` arm. Then, when a new stat is added to `StatKind`, the compiler stops at this `match` until the new stat has a heading, instead of quietly filing it somewhere.

**For the curious.** Kinetik puts your CPU and GPU model next to the headings. That is one sysinfo call away (`cpus()[0].brand()`), and left out here to keep the quest to one idea.

## Your turn

- In `group_of`, write a `match` on `kind` with one arm per group, joining the kinds of a group with `|`.
- Save. Press Run: the widget shows headings.
