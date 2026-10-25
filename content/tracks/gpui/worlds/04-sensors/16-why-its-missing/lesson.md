Hover a `--` in Kinetik and it tells you why: the CPU temperature needs administrator, there is no battery, the ping got no answer. A bare `None` cannot say any of that. The sensors need a way to explain themselves.

**Three reasons.** `Missing` in `sensor.rs` has three variants: `NeedsAdmin`, `NoDevice` and `Timeout`. `missing_text` turns each into the hover card's words.

**Ask the sensor.** `Sensor` gains `missing()`, with a default body that says `None`, so the sensors that have nothing to explain need no change. Sensors that know better override it: the GPU says `NoDevice` without nvml, the battery says `NoDevice` on a desktop, ping says `Timeout`.

**Errors as numbers.** Windows functions report failure with an **HRESULT**: a 32-bit number whose top bit means "failed", and whose rest says which failure. They are written in hex because the parts line up with hex digits. WMI's own codes all start `0x8004`, and `0x80041003` is WBEM_E_ACCESS_DENIED, "access denied". Each code is one fixed number, so `==` compares them.

**Why i32.** The top bit makes the number negative when Rust stores it as an `i32`, which is what the `windows` crate uses. So `WBEM_E_ACCESS_DENIED` is written `0x8004_1003_u32 as i32`: the same 32 bits, read as signed. The same kind of check, for a different code:

```rust
if hres == WBEM_E_INVALID_CLASS { Missing::NoDevice } else { Missing::Timeout }
```

**Which reason.** Access denied means administrator would help. Any other WMI failure means no thermal zone answered: `NoDevice`. On the desktop this track was written on, WMI answers "not supported", so its CPU temperature reads "not on this PC"; only Kinetik's driver route can get it there.

## Your turn

- In `missing_from_wmi`, give back `Missing::NeedsAdmin` when `hres` is the access-denied code, else `Missing::NoDevice`.
- Save.
