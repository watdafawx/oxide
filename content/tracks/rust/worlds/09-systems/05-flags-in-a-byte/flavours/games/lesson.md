A server list entry can show a handful of yes-or-no rules: whitelist on, PvP on, hardcore mode. Storing each as a whole byte wastes space, so protocols store each rule as one bit of a single byte. The pinger needs to turn single bits on, off and test them without touching the others.

**Eight answers in a byte.** A byte has 8 bits, so it can hold 8 separate yes-or-no rules. Bit 0, the one worth 1, says "whitelist"; bit 1, worth 2, says "PvP"; bit 2, worth 4, says "hardcore". `0b0000_0101` means whitelist and hardcore, no PvP.

**Names for bits.** `const WHITELIST: u8 = 0b0000_0001;` gives a bit a name. A **const** is a value fixed when the program is built: it can never change, the type is required, and the name is written in CAPITALS by habit. A bit with a name is called a **flag**, and a value used to pick out bits is a **mask**.

**On with `|`.** `|` is **OR**: a bit is on in the result if it is on in either side. `rules | PVP` turns PvP on and leaves every other rule as it was.

**Test with `&`.** `&` is **AND**: a bit stays on only if it is on in both sides, so `rules & PVP` keeps the PvP bit and clears the rest. If the result is not 0, PvP was on. For a mask of several bits, `rules & mask == mask` asks "are all of them on?".

**Off with `& !`.** `!` flips every bit of a number, so `!PVP` is every bit except PvP. `rules & !PVP` keeps everything but PvP, which turns it off.

```rust
const KEEP_INVENTORY: u8 = 0b0000_0001;
const MOB_GRIEFING: u8 = 0b0000_0010;

let gamerules = KEEP_INVENTORY | MOB_GRIEFING;        // both on
assert!(gamerules & MOB_GRIEFING != 0);               // griefing is on
assert_eq!(gamerules & !KEEP_INVENTORY, MOB_GRIEFING); // keep inventory turned off
assert_eq!(gamerules.count_ones(), 2);                // how many bits are on
```

**For the curious.** In C, `rules & MASK != 0` is a classic bug, because C reads it as `rules & (MASK != 0)`. Rust gives `&` a higher priority than `!=`, so it means what it looks like.

## Your turn

- Give `HARDCORE` the bit worth 4.
- Fill in `enable_rule`, `rule_enabled` and `disable_rule`, one operator each.
- In `count_enabled_rules`, count the bits that are on with `count_ones()`.
