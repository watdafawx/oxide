Hover a `--` in Kinetik and it tells you why: the CPU temperature needs administrator, the battery is not on this PC, the ping got no answer. A bare `None` cannot say any of that. The sensors need a way to explain themselves.

**Three reasons.** `Missing` in `sensor.rs` has three variants: `NeedsAdmin`, `NoDevice` and `Timeout`. `missing_text` turns each into the hover card's words.

**Ask the sensor.** `Sensor` gains `missing()`, with a default body that says `None`. Sensors that know better override it: the GPU says `NoDevice` without nvml, the battery says `NoDevice` on a desktop, ping says `Timeout`.

**WMI's error codes.** The thermal sensor gets an error number back from WMI when the query fails. `0x80041003` is "access denied", which means administrator would help. Anything else means this PC has no thermal zone:

```rust
if hres == WBEM_E_ACCESS_DENIED { Missing::NeedsAdmin } else { Missing::NoDevice }
```

**On many desktops.** The desktop this track was written on gets "not supported" from WMI, so its CPU temperature reads as not on this PC, not as needs administrator. Kinetik's driver route is the only way to get it here.

## Your turn

- In `missing_from_wmi`, give back `Missing::NeedsAdmin` when `hres` is `WBEM_E_ACCESS_DENIED`, else `Missing::NoDevice`.
- Save.
