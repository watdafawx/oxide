Windows filled in the power status, but most of it is not plain numbers. `BatteryFlag` is one byte that answers several yes/no questions at once: is there a battery, is it charging, is it low. Reading it means reading single bits.

**Eight switches in a byte.** A byte is 8 bits, and each bit is a switch that is off (0) or on (1). Counting from the right, the bits are worth 1, 2, 4, 8, 16, 32, 64 and 128, and the byte's value is the sum of the switches that are on. A flag of 10 is 8 + 2: the switch worth 8 and the switch worth 2 are on.

**Why pack them.** A C struct from the 1990s saves space by putting many answers in one number, and Win32 still does it everywhere. Such a number is called a **bit mask** or **flags**. In `BatteryFlag`, the bit worth 128 means "no system battery", the bit worth 8 means "charging", and 2 means "low".

**Ask with &.** `flag & 8` keeps only the bits that are on in both numbers. If the bit worth 8 is on in `flag`, the answer is 8, otherwise 0. So `flag & 8 != 0` reads "is the bit worth 8 on?", whatever the other switches say. `flag == 8` would be wrong: a charging, low battery is 10.

**A second gap.** `BatteryLifePercent` is a plain number from 0 to 100, except 255, which means "unknown". A desktop reports flag 128 and percent 255, so it decodes to `None`: the right answer, not a failure. Here the same check reads a different flag, Windows' battery saver:

```rust
let saver_on = status.SystemStatusFlag & 1 != 0;
```

## Your turn

- In `decode_battery`, give back `None` when the bit worth 128 is on or the percent is 255.
- Otherwise give back a `Battery` with the percent as `f64`, and `charging` from the bit worth 8.
- Save.
