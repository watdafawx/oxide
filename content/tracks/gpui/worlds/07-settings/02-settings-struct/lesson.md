The bar's look, its order, the left-click action, the ping address: they are spread over the bar, the view and `BarUi`. A settings window needs them in one place it can change, and that will be saved to disk later. That place is `Settings` in `settings.rs`.

**One struct.** `Settings` has a field for everything the window can change: the interval, the bar order, the label mode, two lines, opacity, font size, gap, per-stat colours, the left-click action, the ping address, all taskbars, shift left.

**Ready for JSON.** `#[derive(Serialize, Deserialize)]` lets `serde_json` write and read it; the model's enums gained the same derives. `#[serde(default)]` fills any field missing from a file with its default, so adding a setting later never breaks a file an older version saved.

**Kinetik's defaults.** A new install updates every 1000 ms, shows `default_order()` with text labels on one line, draws pills at 0.85 opacity with 13-pixel text and an 8-pixel gap, has no colours of its own, opens Task Manager on a left click, pings `"1.1.1.1"`, uses only the main taskbar (`all_taskbars: false`) and shifts nothing left (`shift_left: 0`).

**Colours in order.** `colours` is a `BTreeMap`, not a `HashMap`. A `BTreeMap` keeps its keys sorted, so the saved JSON lists them in the same order every time, and a file you compare or keep in git does not change for nothing. A new install starts with an empty one.

**One place.** `impl Default` makes `Settings::default()` the only place these numbers live. A reset button, a new profile and a broken file all start from it. The same shape for a smaller struct:

```rust
impl Default for Limit {
    fn default() -> Self {
        Limit { at: 90.0, when: WarnWhen::Above }
    }
}
```

## Your turn

- In `Settings::default` (in `lib.rs`), replace the `todo!()` with a `Settings { .. }` that gives every field Kinetik's default.
- Save.
