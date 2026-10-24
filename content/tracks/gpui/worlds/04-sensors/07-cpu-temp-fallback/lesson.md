Without its driver, Kinetik reads CPU temperature from the ACPI thermal zone: a number the motherboard reports to Windows. Most laptops report a real one. Many desktops report the same fake number forever, and Kinetik ignores it. `ThermalSensor` in `thermal.rs` asks for the number; you write the conversion.

**Asking WMI.** `ThermalSensor::read` sends WMI the query for `MSAcpi_ThermalZoneTemperature`, and the `wmi` crate turns each answer into a `ThermalZone` struct, one per zone. That part is written. Each zone's `current_temperature` reaches your function as a plain `u32`.

**Tenths of a kelvin.** **ACPI**, the standard by which a motherboard reports power and heat, gives temperature in tenths of a kelvin: 3332 means 333.2 K. Kelvin starts at absolute zero, so kelvin minus 273.15 is Celsius, and 3332 is 60.05 °C. Divide by `10.0` after turning the `u32` into an `f64`, or the tenths are lost.

**Known fakes.** Boards without a sensor report a constant: 2732 (0 °C), 2982 (25 °C) or 3010 (27.8 °C). A CPU never sits at exactly 25.05 °C for hours. `DUMMY` lists them, and `DUMMY.contains(&value)` says whether a value is one of them.

**Impossible numbers.** Anything below 1 °C or above 120 °C is a broken reading, not a CPU. A range has `contains` too, and checks both ends at once. The other way round, from °C to tenths of a kelvin, with a range check:

```rust
let tenths = ((celsius + 273.15) * 10.0) as u32;
let plausible = (2740..=3930).contains(&tenths);
```

**When it fails.** The query may fail outright: no thermal zone, or access denied. The sensor then gives `None`, and later in this world the hover card says which.

## Your turn

- In `acpi_to_celsius`, give back `None` for any value in `DUMMY`.
- Convert to °C, then give back `Some(celsius)` only when it is from 1 to 120.
- Save.
