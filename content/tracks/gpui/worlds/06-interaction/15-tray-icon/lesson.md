Kinetik keeps an icon in the tray: click it for Settings, right-click it for a menu that works even while the bar is hidden. The `tray-icon` crate draws the icon and its menu, and reports clicks as events. `main.rs` reads those events every 100 ms and dispatches actions to the bar.

**Events in, actions out.** The bar already handles `OpenSettings` and `Exit`. The tray only has to say which one an event means. Two small functions decide; `main.rs` does the dispatching.

**Act on release.** The icon sends one `TrayIconEvent::Click` for the press and another for the release, with `button` and `button_state` fields that say which. React to the left button's release, the way buttons everywhere work. A pattern can name the fields it checks and skip the rest with `..`; a double click, whatever its fields, would be:

```rust
TrayIconEvent::DoubleClick { .. } => Some(TrayAction::OpenSettings),
```

**Two `MouseButton`s.** GPUI and `tray-icon` both have a type called `MouseButton`. Write the tray's one in full, `tray_icon::MouseButton::Left`, so Rust knows which you mean.

**Menus by id.** The tray menu's rows were made with ids `"settings"` and `"exit"`. A `match` on the id's text picks the action; anything else is `None`.

**GPUI's loop is fine.** Tray events still arrive while GPUI runs its own message loop; the 100 ms poll picks them up.

## Your turn

- In `tray_click_action`, give `Some(TrayAction::OpenSettings)` for a left-button release, `None` otherwise.
- In `tray_menu_action`, map `"settings"` and `"exit"` to their actions.
- Save. Press Run, then click the blue tray icon.
