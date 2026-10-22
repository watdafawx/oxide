//! CPU temperature from the ACPI thermal zone.

use serde::Deserialize;
use wmi::{WMIConnection, WMIError};

use crate::sensor::{Missing, Sensor};
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

/// WBEM_E_ACCESS_DENIED: the query needs administrator.
pub const WBEM_E_ACCESS_DENIED: i32 = 0x8004_1003_u32 as i32;

/// Why a WMI query gave nothing: access denied needs admin, anything else means no sensor.
pub fn missing_from_wmi(hres: i32) -> Missing {
    if hres == WBEM_E_ACCESS_DENIED { Missing::NeedsAdmin } else { Missing::NoDevice }
}

#[derive(Deserialize)]
#[serde(rename = "MSAcpi_ThermalZoneTemperature", rename_all = "PascalCase")]
struct ThermalZone {
    current_temperature: u32,
}

/// CPU temperature from the ACPI thermal zone, the way Kinetik reads it without its driver.
/// Most laptops have one; most desktops do not, and some need administrator.
#[derive(Default)]
pub struct ThermalSensor {
    missing: Option<Missing>,
}

impl ThermalSensor {
    pub fn new() -> Self {
        ThermalSensor::default()
    }
}

impl Sensor for ThermalSensor {
    fn kind(&self) -> StatKind {
        StatKind::CpuTemp
    }

    fn read(&mut self) -> Option<f64> {
        let zones = WMIConnection::with_namespace_path(r"ROOT\WMI").and_then(|c| c.query::<ThermalZone>());
        match zones {
            Ok(zones) => {
                let celsius = zones.iter().find_map(|z| acpi_to_celsius(z.current_temperature));
                self.missing = if celsius.is_some() { None } else { Some(Missing::NoDevice) };
                celsius
            }
            Err(WMIError::HResultError { hres }) => {
                self.missing = Some(missing_from_wmi(hres));
                None
            }
            Err(_) => {
                self.missing = Some(Missing::NoDevice);
                None
            }
        }
    }

    fn missing(&self) -> Option<Missing> {
        self.missing
    }
}
