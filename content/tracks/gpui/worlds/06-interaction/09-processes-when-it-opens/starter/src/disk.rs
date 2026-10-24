//! Free disk space.

use std::path::Path;

use sysinfo::Disks;

use crate::sensor::Sensor;
use crate::stat::StatKind;

/// Free space on C:, in bytes.
pub struct DiskSensor {
    disks: Disks,
}

impl DiskSensor {
    pub fn new() -> Self {
        DiskSensor { disks: Disks::new_with_refreshed_list() }
    }
}

impl Default for DiskSensor {
    fn default() -> Self {
        DiskSensor::new()
    }
}

impl Sensor for DiskSensor {
    fn kind(&self) -> StatKind {
        StatKind::DiskFree
    }

    fn read(&mut self) -> Option<f64> {
        self.disks.refresh(true);
        let c = self.disks.list().iter().find(|d| d.mount_point() == Path::new("C:\\"))?;
        Some(c.available_space() as f64)
    }
}
