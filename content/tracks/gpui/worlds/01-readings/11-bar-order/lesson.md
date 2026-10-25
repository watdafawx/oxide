Kinetik's Arrange page is one list: every stat and divider on the bar, left to right, and you drag rows to reorder it. In the model that list is a `Vec`, and each entry is either a stat or a divider.

**One of two things.** A place on the bar holds a stat or a divider, never both. That is an enum with data: `Slot::Stat(StatKind::Ping)` or `Slot::Divider(DividerStyle::Line)`. `DividerStyle` has Kinetik's four looks: line, dotted line, dot and blank space.

**The default bar.** A fresh install shows eight stats in four groups: the two CPU stats, the two GPU stats, RAM on its own, then download, upload and ping. A line divider sits between each group and the next. Battery is off by default.

**Write the list.** A `Vec` written out in place is `vec![...]`, with a comma after each entry. The starter already lists the eight stats in order. A divider is one more entry where it should sit, like this dot between ping and battery on a laptop's bar:

```rust
Slot::Stat(StatKind::Ping),
Slot::Divider(DividerStyle::Dot),
Slot::Stat(StatKind::Battery),
```

**Order is data.** Nothing else in the code knows CPU comes first. Change this list, and the bar, the Arrange page and the two-line layout all follow.

## Your turn

- In `default_order`, put a `Line` divider between each group and the next. Count the groups first: how many dividers does that make?
- Save. Press Run to print the default bar.
