Kinetik shows the laptop battery with a `+` while charging. Windows has one function for it, `GetSystemPowerStatus`, written in C. Rust can call it through the `windows` crate, but the compiler cannot check what a C function does with the memory you hand it.

**The word for it.** Calling it is **unsafe**: `unsafe { .. }` marks a spot where you promise the rules hold that the compiler cannot check. Here the promise is simple: `status` is a real, writable struct for the whole call. The comment above the call says so.

**Fill in a struct.** The function writes into a `SYSTEM_POWER_STATUS` you pass with `&mut`. It gives back a `Result`; `.ok()?` turns a failure into `None`:

```rust
unsafe { GetSystemPowerStatus(&mut status) }.ok()?;
```

**Decode the flags.** `BatteryFlag` packs yes/no answers into bits: 128 means "no system battery", 8 means "charging". `flag & 128 != 0` asks "is bit 128 set". A `BatteryLifePercent` of 255 means "unknown".

**This PC.** A desktop has no battery, so it reports flag 128 and percent 255, and the sensor gives `None`. That is the right answer, not a failure.

## Your turn

- In `read_power_status`, make a default status, call `GetSystemPowerStatus` inside `unsafe` with `.ok()?`, give back `Some(status)`.
- In `decode_battery`, give back `None` for flag 128 or percent 255, else the percent and whether bit 8 is set.
- Save.
