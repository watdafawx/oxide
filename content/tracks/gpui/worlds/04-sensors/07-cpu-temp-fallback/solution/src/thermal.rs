//! CPU temperature from the ACPI thermal zone.

use serde::Deserialize;
use wmi::WMIConnection;

use crate::acpi_to_celsius;
use crate::sensor::Sensor;
use crate::stat::StatKind;

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
