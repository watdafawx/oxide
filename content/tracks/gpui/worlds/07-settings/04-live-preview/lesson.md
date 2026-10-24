Kinetik's Settings window has a live preview: drag the font size and the bar on the taskbar changes as you drag. The bar should not poll for that. It should hear about every change to the global, the moment it happens.

**Watch a global.** `cx.observe_global::<Settings>(..)` runs its closure every time the `Settings` global changes. The closure gets two inputs: the bar, to change, and the bar's context, to read the global and notify. A watcher that only redraws would be:

```rust
let watch = cx.observe_global::<Settings>(|_bar, cx| cx.notify());
```

**Apply, then redraw.** Redrawing alone is not enough: the bar draws from its own `look`, `order` and `colours`, which `apply_settings` copies from a `Settings`. So the closure applies the new settings first, then notifies.

**Clone, then apply.** `cx.global` lends the settings out of `cx`, and while that loan lives, `cx` cannot be used for anything that changes it, such as `cx.notify()`. A clone gives the closure its own copy, so nothing stays borrowed from `cx` and the lines can come in any order.

**Keep the subscription.** `observe_global` gives back a `Subscription`. Dropping it stops the watching, like a dropped `Task` stopped its work in Live. Store it in `self.settings_watch`.

**Who changes it.** The Settings window changes the global with `Settings::update_global(cx, |s, _| ..)`; every observer hears about it after the update.

## Your turn

- In `watch_settings`, call `cx.observe_global::<Settings>(..)` with a closure that clones the settings, applies them to the bar, and notifies.
- Store the result in `self.settings_watch`.
- Save.
