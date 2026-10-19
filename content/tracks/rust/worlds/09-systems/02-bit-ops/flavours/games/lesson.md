**Why now:** a server list entry carries a handful of yes/no facts (whitelist, PvP, hardcore) and a MOTD color. Storing each as a full byte wastes space; protocols store them as **bits** in one number.

```rust
const WHITELIST: u8 = 1 << 0;   // 0b001
const PVP: u8       = 1 << 1;   // 0b010
const HARDCORE: u8  = 1 << 2;   // 0b100

let mut rules = 0u8;
rules |= PVP;                   // turn on:  0b010
rules &= !PVP;                  // turn off: 0b000
let on = rules & HARDCORE != 0; // test

let color = (0xFFu32 << 16) | (0x80 << 8) | 0x07;   // pack r, g, b: 0x00FF8007
let r = (color >> 16) as u8;                         // unpack: 0xFF
```

- `|` sets bits, `& !mask` clears them, `& mask` tests them. `<<` and `>>` move bits.
- Widen before shifting: `u32::from(r) << 16`. Shifting a `u8` left by 16 overflows.
- `as u8` keeps the low 8 bits, which is exactly what unpacking needs.
- `count_ones()` counts the 1 bits.

## Your turn

In `src/lib.rs`:

- `enable_rule`, `disable_rule`, `rule_enabled`: set, clear and test rule bits.
- `pack_color` and `unpack_color`: three bytes in and out of `0x00RRGGBB`.
- `count_enabled_rules`: how many bits are on.
