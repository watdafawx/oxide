Kinetik keeps an icon in the tray: click it for Settings, right-click it for a menu that works even while the bar is hidden. The `tray-icon` crate draws the icon and its menu, and reports clicks as events. `main.rs` reads those events every 100 ms and dispatches actions to the bar.

**Events in, actions out.** The bar already handles `OpenSettings` and `Exit`. The tray only has to say which one an event means. Two small functions decide; `main.rs` does the dispatching.

**Act on release.** The icon sends an event for the press and another for the release. React to the left button's release, the way buttons everywhere work. Fields you do not care about are skipped with `..`:

```rust
TrayIconEvent::Click { button: tray_icon::MouseButton::Left, button_state: MouseButtonState::Up, .. } => Some(TrayAction::OpenSettings),
```

**Menus by id.** The tray menu's rows were made with ids `"settings"` and `"exit"`. A `match` on the id's text picks the action; anything else is `None`.

**Same loop as R5.** Spike R5 checked that tray events arrive while GPUI runs its own message loop. They do; the 100 ms poll picks them up.

## Your turn

- In `tray_click_action`, give `Some(TrayAction::OpenSettings)` for a left-button release, `None` otherwise.
- In `tray_menu_action`, map `"settings"` and `"exit"` to their actions.
- Save. Press Run, then click the blue tray icon.
