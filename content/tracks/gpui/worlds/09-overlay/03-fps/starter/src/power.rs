//! The laptop battery.

use windows::Win32::System::Power::{GetSystemPowerStatus, SYSTEM_POWER_STATUS};

use crate::sensor::{Missing, Sensor};
use crate::stat::StatKind;

/// The laptop battery, decoded.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Battery {
    pub percent: f64,
    pub charging: bool,
}

/// Ask Windows for the power status. `unsafe`: Rust cannot check what the Windows function does.
pub fn read_power_status() -> Option<SYSTEM_POWER_STATUS> {
    let mut status = SYSTEM_POWER_STATUS::default();
    // SAFETY: `status` is a valid, writable SYSTEM_POWER_STATUS for the whole call.
    unsafe { GetSystemPowerStatus(&mut status) }.ok()?;
    Some(status)
}

/// BatteryFlag 128 is "no system battery"; a percent of 255 is "unknown".
pub fn decode_battery(status: &SYSTEM_POWER_STATUS) -> Option<Battery> {
    if status.BatteryFlag & 128 != 0 || status.BatteryLifePercent == 255 {
        return None;
    }
    Some(Battery { percent: status.BatteryLifePercent as f64, charging: status.BatteryFlag & 8 != 0 })
}

/// The laptop battery's charge.
#[derive(Default)]
pub struct BatterySensor;

impl Sensor for BatterySensor {
    fn kind(&self) -> StatKind {
        StatKind::Battery
    }

    fn read(&mut self) -> Option<f64> {
        read_power_status().as_ref().and_then(decode_battery).map(|b| b.percent)
    }

    fn missing(&self) -> Option<Missing> {
        let status = read_power_status()?;
        if decode_battery(&status).is_none() { Some(Missing::NoDevice) } else { None }
    }
}
