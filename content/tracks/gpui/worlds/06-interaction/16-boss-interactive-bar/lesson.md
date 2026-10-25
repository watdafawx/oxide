Every way to touch the bar has been built once: a click with its setting, hover and the hover card with its graph and processes, actions, the right-click menu and its rows, and the tray icon. This boss writes the two functions that wire them onto the bar again, in one go.

**What is given.** `HoverCard`, `card_for`, `graph`, `graph_points`, `top_five`, the actions, `on_pill_click`, `menu`, `menu_row`, the tray mapping, and `main.rs` with the tray loop. `view.rs` sends every pill through `interactive_pill` and the bar through `bar_layer`.

**The pill.** An id; a lighter look and `ui.hovered` under the mouse; the left-click action; a tooltip that fills the card's process list as it opens; the right-button menu position. Each is one call on the pill, and each call gives the pill back, so they chain:

```rust
pill
    .id(..)                                  // first: on_click and tooltip need a Stateful<Div>
    .hover(..).on_hover(..)                  // the look, then the state
    .on_click(..)                            // the Left-click action
    .tooltip(..)                             // the card, with fresh processes
    .on_mouse_down(MouseButton::Right, ..)   // where the menu opens
```

**Copy before the chain.** The card and the `top_processes` function come from `bar`, which the listeners cannot reach later. Take both into locals first, as the starters did, and let the `move` closures carry them.

**The bar.** Focus, so actions arrive: the `match` on `&this.ui.focus` gives back the bar either way. Then the three action handlers, and the menu, deferred and anchored, while `menu_at` holds a point.

**From the tray.** The tray loop in `main.rs` dispatches the same actions as the menu, so Exit from the tray quits through your handler too.

## Your turn

- Write `interactive_pill`: id and click first, then hover, the tooltip and the right click.
- Write `bar_layer`: focus, actions, menu.
- Press Run: click, hover, right-click, and try the tray icon's menu.
