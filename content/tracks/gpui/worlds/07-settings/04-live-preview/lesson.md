Kinetik's Settings window has a live preview: drag the font size and the bar on the taskbar changes as you drag. The bar should not poll for that. It should hear about every change to the global, the moment it happens.

**Watch a global.** `cx.observe_global::<Settings>(..)` runs its closure every time the `Settings` global changes. The closure gets the bar and its context, so it can update and redraw:

```rust
let watch = cx.observe_global::<Settings>(|bar, cx| {
```

**Clone, then apply.** The closure reads the global with `cx.global`, but `apply_settings` needs `&mut` access to the bar through the same `cx`. Cloning the settings first ends the borrow of the global before the bar changes.

**Keep the subscription.** `observe_global` gives back a `Subscription`. Dropping it stops the watching, like a dropped `Task` stops its work. Store it in `self.settings_watch`.

**Who changes it.** The Settings window changes the global with `Settings::update_global(cx, |s, _| ..)`; every observer hears about it after the update.

## Your turn

- In `watch_settings`, call `cx.observe_global::<Settings>(..)` with a closure that clones the settings, applies them, and notifies.
- Store the result in `self.settings_watch`.
- Save.
