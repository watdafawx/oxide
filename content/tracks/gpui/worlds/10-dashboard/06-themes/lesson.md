Kinetik's dashboard has four looks: Modern, plus Terminal, Amber terminal and Yellow terminal. The setting is a `DashTheme` in `Settings`, one word per look. Drawing needs actual colours, so something has to turn the word into them.

**One struct per look.** `Theme` holds five things: a background, a card colour, the text colour, an accent for gauges and graphs, and whether to use a monospace font. `render` and `Card` read only from a `Theme`, never from the setting itself.

**One arm per choice.** `theme(choice)` is a `match` with four arms, and each arm builds a whole `Theme`. The Terminal look is green on black:

```rust
DashTheme::Terminal => Theme { background: 0x000000, card: 0x0a0f0a, text: 0x33ff66, accent: 0x33ff66, monospace: true },
```

**The other three.** Amber: background `0x0d0800`, card `0x140c00`, text and accent `0xffb000`, monospace. Yellow: background `0x0b0b00`, card `0x141400`, text and accent `0xffee33`, monospace. Modern: background `0x15171c`, card `0x22252d`, text `0xf2f2f2`, accent `0x5ab0ff`, not monospace.

**Why terminal looks.** The terminal themes copy old monochrome monitors: a near-black background, one bright colour for everything, and Consolas, so the numbers line up in columns like on a terminal.

**Live switch.** The dashboard watches the Settings global, so changing the theme redraws it at once, like the bar's live preview. A fifth look later is one more arm, and nothing in `render` changes.

## Your turn

- In `theme`, `match` on `choice` with one arm per look.
- Build each arm's `Theme` from the colours above. Save.
- Switch `theme` in the settings file and press Run.
