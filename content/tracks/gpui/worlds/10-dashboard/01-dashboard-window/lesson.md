Kinetik's dashboard is a full-screen view of everything the app measures: a card per stat with the reading now, its average, its peak and a graph. The bar's menu has a new row, Dashboard, which opens it. This quest lines the cards up.

**Cards in rows.** A flex row puts cards side by side, but each card is only as wide as its content, so the rows come out ragged. A dashboard wants columns: every card the same width, each one lined up under the card above.

**The word for it.** A **grid** lays its children out in rows and columns. `.grid()` switches it on, and `.grid_cols(n)` says how many columns. Children fill the first row left to right, then start the next row, so twelve children in 4 columns make three rows of four.

```rust
div().grid().grid_cols(2).gap_2() // two to a row
    .child("CPU").child("RAM")
    .child("GPU").child("Disk")
```

**Each call hands it back.** Like every style call since the Views world, `.grid()` takes the `div`, changes its style and returns it, which is why the calls chain. Among style calls the order does not matter: `.gap_4().grid()` and `.grid().gap_4()` give the same grid.

**The number's type.** `grid_cols` wants a `u16`, a small whole number. `columns` is a `usize`, because two quests from now it is worked out from the window's width. `columns as u16` converts it, the same `as` the Rust track uses for every number conversion.

**The spacing scale.** `.gap_4()` is already on the outer `div`. The 4 is a step on GPUI's spacing scale, not pixels: one step is a quarter of the base text size, which is 16 px, so `gap_4` is 16 px and the bar's `gap_2` is 8 px. `.p_4()` is the same 16 px as padding inside the edge. When you need an exact size, `px(..)` says it in pixels.

**A component again.** Each card is a `Card`, a `RenderOnce` component like the pill: a title, the value big, the average and peak small, and a graph. `render` already builds one per stat. For now `columns` is fixed at 4.

## Your turn

- In `DashboardView`'s `render` (in `lib.rs`), make the outer `div` a grid with `columns` columns.
- Save. Press Run: the dashboard opens next to the bar.
