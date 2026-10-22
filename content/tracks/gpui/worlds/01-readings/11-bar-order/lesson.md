Kinetik's Arrange page is one list: every stat and divider on the bar, left to right, and you drag rows to reorder it. In the model that list is a `Vec`, and each entry is either a stat or a divider.

**One of two things.** A place on the bar holds a stat or a divider, never both. That is an enum with data: `Slot::Stat(StatKind::Ping)` or `Slot::Divider(DividerStyle::Line)`. `DividerStyle` has Kinetik's four looks: line, dotted line, dot and blank space.

**The default bar.** A fresh install groups the stats with three line dividers: CPU usage and temperature, a line, GPU usage and temperature, a line, RAM, a line, then download, upload and ping. Battery is off by default.

**Write the list.** A `Vec` written out in place is `vec![...]` with commas between entries:

```rust
Slot::Stat(StatKind::CpuUsage),
```

**Order is data.** Nothing else in the code knows CPU comes first. Change this list, and the bar, the Arrange page and the two-line layout all follow.

## Your turn

- In `default_order`, fill the `vec![...]` with the eleven slots in the order above.
- Save. Press Run to print the default bar.
