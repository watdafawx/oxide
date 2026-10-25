Kinetik's Settings window has a page per topic down the left side. This one starts with three: Stats (what is on the bar), Arrange (the order) and Appearance (the look). Which page is shown belongs to the window, not to the saved settings.

**Pages are enums.** `Page` has three variants and `PAGES` lists them in sidebar order. `SettingsWindow::page` holds the one shown; `render` matches on it to pick the content.

**Not a setting.** Closing and reopening Settings and starting on Stats is fine; nobody saves "the page I was on". So `page` lives in the window's own state, not in the `Settings` global.

**A listener on another view.** `sidebar_row` gets a `Context<SettingsWindow>`, so its `cx.listener` hands back the `SettingsWindow` as `this`, not the bar. Same four inputs, different view. The Arrange list will select a row the same way:

```rust
.on_click(cx.listener(move |this, _, _, cx| { this.selected = index; cx.notify(); }))
```

**Highlight the current.** `sidebar_row` already paints the current page's row blue with `.when(page == current, ..)`, the same trick as the labels in Views.

## Your turn

- In `sidebar_row`, add `.on_click(cx.listener(..))` that sets `this.page` to `page` and notifies.
- Save. Press Run and click through the three pages.
