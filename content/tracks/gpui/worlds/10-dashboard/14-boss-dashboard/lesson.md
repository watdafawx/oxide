Every piece of the dashboard is written: card numbers, columns, text size, gauges, themes, the layout editor, the JSON snapshot, the handler, the key, the address and the server thread. This boss draws the window from scratch, and with the server from the last quest already in place, the web side goes live with it.

**Settings first.** `render` needs the dashboard settings and the bar. `cx.global::<Settings>()` and `self.bar.read(cx)` both borrow `cx`, and the bar's borrow lasts as long as you use `bar`. So clone the dashboard settings out first, read the window's width, and only then borrow the bar, once.

**The grid's numbers.** `columns_for(width)` gives the columns. A card's width is `(width - px(32.)) / columns as f32`, because the outer `p_4` takes 16 pixels on each side. `theme(settings.theme)` gives the colours.

**One Card per visible stat.** `visible_cards(&settings.hidden)` gives the kinds; map each one to a `Card` and collect them. Its fields: `kind`, `numbers` from `card_numbers`, `share` from the value and the widget's `bar_share` (only percentages and temperatures get one), the history, the theme and `text_size` from `card_text_size`.

**The history, or nothing.** A card's graph wants a `Vec<f64>`. A stat that was never read has no history, so map the `Option` and fall back with `unwrap_or_default()`, which gives the type's empty value, here an empty `Vec`:

```rust
let points: Vec<f64> = bar.histories.get(&StatKind::Ping).map(|h| h.values().collect()).unwrap_or_default();
```

**Names the tests look for.** The outer `div` is named with `.debug_selector(|| "dashboard".into())`; each `Card` already names itself, such as `card-CpuUsage`. It fills the window with `.size_full()`, gets `.p_4()` and the theme's background, the grid of `columns`, `.gap_4()`, and the cards as children.

**Wired up.** `main.rs` makes a key on first start, prints the link, starts `serve_dashboard`, and copies `web_snapshot` into the shared state every time the bar changes, with `cx.observe_self`.

**Safe by default.** The server listens on `127.0.0.1` unless sharing is on, and every API request needs the key: the two rules from earlier in this world, now live.

## Your turn

- Write `DashboardView::render`: settings and width, the grid's numbers, the bar, one `Card` per visible stat.
- Put the cards in the named outer `div` as a grid. The first four objectives follow it.
- Press Run, then open the printed link in a browser.
