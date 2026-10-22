The bar's look, its order, the left-click action, the ping address: they are spread over the bar, the view and `BarUi`. A settings window needs them in one place it can change, and that will be saved to disk later. That place is `Settings` in `settings.rs`.

**One struct.** `Settings` has a field for everything the window can change: the interval, the bar order, the label mode, two lines, opacity, font size, gap, per-stat colours, the left-click action, the ping address, all taskbars, shift left.

**Ready for JSON.** `#[derive(Serialize, Deserialize)]` lets `serde_json` write and read it. The model's enums gained the same derives. `#[serde(default)]` fills any field missing from an older file with its default, so adding a setting later never breaks a saved file.

**Kinetik's defaults.** A new install updates every second, shows the default bar with text labels, a pill at 85 % opacity, 13-pixel text and an 8-pixel gap, opens Task Manager on a left click, and pings 1.1.1.1:

```rust
interval_ms: 1000, order: default_order(), label_mode: LabelMode::Text,
```

**Defaults in one place.** `Settings::default()` is the only place these numbers live. A reset button, a new profile and a broken file all start from it.

## Your turn

- In `Settings::default` (in `lib.rs`), fill in every field with Kinetik's default.
- Save.
