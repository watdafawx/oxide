//! CPU temperature from the ACPI thermal zone.

use serde::Deserialize;
use wmi::WMIConnection;

use crate::sensor::Sensor;
use crate::stat::StatKind;

/// ACPI reports tenths of a kelvin. These are values desktops report when they have no real sensor.
const DUMMY: [u32; 3] = [2732, 2982, 3010];

/// Tenths of a kelvin to °C, or None for a known dummy value or anything outside 1 to 120 °C.
pub fn acpi_to_celsius(tenths_kelvin: u32) -> Option<f64> {
    if DUMMY.contains(&tenths_kelvin) {
        return None;
    }
    let celsius = tenths_kelvin as f64 / 10.0 - 273.15;
    if (1.0..=120.0).contains(&celsius) { Some(celsius) } else { None }
}

#[derive(Deserialize)]
#[serde(rename = "MSAcpi_ThermalZoneTemperature", rename_all = "PascalCase")]
struct ThermalZone {
    current_temperature: u32,
}

/// CPU temperature from the ACPI thermal zone, the way Kinetik reads it without its driver.
/// Most laptops have one; most desktops do not, and some need administrator.
#[derive(Default)]
pub struct ThermalSensor;

impl ThermalSensor {
    pub fn new() -> Self {
        ThermalSensor
    }
}

impl Sensor for ThermalSensor {
    fn kind(&self) -> StatKind {
        StatKind::CpuTemp
    }

    fn read(&mut self) -> Option<f64> {
        let zones = WMIConnection::with_namespace_path(r"ROOT\WMI").and_then(|c| c.query::<ThermalZone>()).ok()?;
        zones.iter().find_map(|z| acpi_to_celsius(z.current_temperature))
    }
}
