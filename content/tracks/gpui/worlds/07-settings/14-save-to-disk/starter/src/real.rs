//! This PC's CPU and RAM.

use sysinfo::System;

use crate::sensor::Sensor;
use crate::stat::StatKind;

/// CPU usage from Windows, through sysinfo. The first read after creating it is 0:
/// usage is measured between two refreshes.
pub struct CpuSensor {
    system: System,
}

impl CpuSensor {
    pub fn new() -> Self {
        CpuSensor { system: System::new() }
    }
}

impl Default for CpuSensor {
    fn default() -> Self {
        CpuSensor::new()
    }
}

impl Sensor for CpuSensor {
    fn kind(&self) -> StatKind {
        StatKind::CpuUsage
    }

    fn read(&mut self) -> Option<f64> {
        self.system.refresh_cpu_usage();
        Some(self.system.global_cpu_usage() as f64)
    }
}

/// RAM in use, in bytes.
pub struct RamSensor {
    system: System,
}

impl RamSensor {
    pub fn new() -> Self {
        RamSensor { system: System::new() }
    }
}

impl Default for RamSensor {
    fn default() -> Self {
        RamSensor::new()
    }
}

impl Sensor for RamSensor {
    fn kind(&self) -> StatKind {
        StatKind::RamUsed
    }

    fn read(&mut self) -> Option<f64> {
        self.system.refresh_memory();
        Some(self.system.used_memory() as f64)
    }
}
