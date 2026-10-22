Without its driver, Kinetik reads CPU temperature from the ACPI thermal zone: a number the motherboard reports through WMI. Most laptops report a real one. Many desktops report the same fake number forever, and Kinetik ignores it. `ThermalSensor` in `thermal.rs` does the WMI query; you write the conversion.

**Tenths of a kelvin.** ACPI gives temperature in tenths of a kelvin: 3332 means 333.2 K. Kelvin minus 273.15 is Celsius, so 3332 is 60.05 °C:

```rust
let celsius = tenths_kelvin as f64 / 10.0 - 273.15;
```

**Known fakes.** Boards without a sensor report a constant: 2732 (0 °C), 2982 (25 °C) or 3010 (27.8 °C). A CPU never sits at exactly 25.05 °C for hours. `DUMMY` lists them; any of those gives `None`.

**Impossible numbers.** Anything below 1 °C or above 120 °C is a broken reading, not a CPU. A range with `contains` checks both ends at once.

**On this PC.** The query may fail outright: no thermal zone, or access denied. The sensor then gives `None`, and the hover card will say which, later in this world.

## Your turn

- In `acpi_to_celsius`, give back `None` for any value in `DUMMY`.
- Convert to °C, then give back `Some(celsius)` only when it is from 1 to 120.
- Save.
