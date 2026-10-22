Kinetik's dashboard has four themes: Modern, plus Terminal, Amber terminal and Yellow terminal looks. The setting is a `DashTheme` in `Settings`; drawing needs actual colours.

**One struct per look.** `Theme` holds a background, a card colour, text, an accent for gauges and graphs, and whether to use a monospace font. `render` and `Card` read only from it.

**One arm per choice.** `theme(choice)` turns the setting into colours: a `match` with four arms, each a whole `Theme`:

```rust
DashTheme::Amber => Theme { background: 0x0d0800, card: 0x140c00, text: 0xffb000, accent: 0xffb000, monospace: true },
```

**Terminal looks.** The terminal themes copy old monochrome monitors: near-black backgrounds, one bright colour for everything, and Consolas so numbers line up like on a terminal.

**Live switch.** The dashboard watches the Settings global, so changing the theme redraws it at once, like the bar's live preview.

## Your turn

- In `theme`, `match` on `choice` and give back the four themes: Modern, Terminal (green `0x33ff66`), Amber (`0xffb000`), Yellow (`0xffee33`).
- Save. Switch the theme in the settings file and press Run.
