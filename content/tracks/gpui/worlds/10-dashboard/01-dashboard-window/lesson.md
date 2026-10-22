Kinetik's dashboard is "a full-screen view of everything Kinetik Stats measures": cards for CPU, GPU, memory, network, disk and more, each with the current reading, its average and peak, and a graph. The bar's menu has a new row, Dashboard, which opens it.

**Cards in rows.** A flex row puts cards side by side but makes each as wide as its content, and the rows come out ragged. A dashboard wants columns: every card the same width, lined up under each other.

**The word for it.** A **grid** lays children out in rows and columns. `.grid()` switches it on and `.grid_cols(n)` says how many columns; children fill the grid left to right, then the next row:

```rust
.grid().grid_cols(columns as u16)
```

**A component again.** Each card is a `Card`, a `RenderOnce` component like the pill: title, the value big, average and peak small, and a graph. `render` makes one per stat.

**Four for now.** `columns` is fixed at 4 here. Two quests from now it follows the window's width.

## Your turn

- In `DashboardView`'s `render` (in `lib.rs`), add `.grid().grid_cols(columns as u16)` to the outer `div()`.
- Save. Press Run: the dashboard opens next to the bar.
