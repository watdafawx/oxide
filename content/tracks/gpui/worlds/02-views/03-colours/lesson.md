Kinetik's bar is dark, its values white, and a value past its warning limit turns red. The CPU temperature in the starter is 88 °C, over the 85 °C limit, so it should be red. Right now everything is the same colour.

**Colours as numbers.** `rgb(0xff5c5c)` is a colour written as six hex digits: two for red, two for green, two for blue, each from `00` to `ff`. `0xffffff` is white, `0x202020` a dark grey.

**Where it goes.** `.bg(..)` colours a box's background; `.text_color(..)` colours the text inside it. The starter gives the bar `.bg(rgb(0x202020))` and asks `value_colour` for each pill's text colour.

**One rule, one place.** Which colour a value gets is a decision, so it lives in one function. Every pill asks it, and the boss of this world will too:

```rust
if warning { rgb(0xff5c5c) } else { rgb(0xffffff) }
```

**Why this red.** `0xff5c5c` is a soft red that reads well on dark grey; pure `0xff0000` glares on a dark taskbar. Change it here and every warning follows.

## Your turn

- In `value_colour`, give back `rgb(0xff5c5c)` when `warning` is true and `rgb(0xffffff)` otherwise.
- Save. Press Run: CPU 88°C turns red.
