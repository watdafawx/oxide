//! Network speed.

use std::time::{Duration, Instant};

use sysinfo::Networks;

use crate::sensor::Sensor;
use crate::stat::StatKind;

/// Bytes counted since the last read, over the time since the last read.
pub fn speed(bytes: u64, elapsed: Duration) -> f64 {
    let seconds = elapsed.as_secs_f64();
    if seconds == 0.0 { 0.0 } else { bytes as f64 / seconds }
}

/// Adapters that would count the same traffic twice, or none at all.
const SKIP: [&str; 5] = ["Loopback", "Teredo", "isatap", "vEthernet", "Filter"];

/// Should this adapter's traffic count toward the bar's total?
pub fn counts_toward_total(name: &str) -> bool {
    !SKIP.iter().any(|skip| name.contains(skip))
}

/// Download or upload speed, summed over the adapters that count.
pub struct NetSpeedSensor {
    networks: Networks,
    down: bool,
    last: Option<Instant>,
}

impl NetSpeedSensor {
    pub fn download() -> Self {
        NetSpeedSensor { networks: Networks::new_with_refreshed_list(), down: true, last: None }
    }

    pub fn upload() -> Self {
        NetSpeedSensor { networks: Networks::new_with_refreshed_list(), down: false, last: None }
    }
}

impl Sensor for NetSpeedSensor {
    fn kind(&self) -> StatKind {
        if self.down { StatKind::NetDown } else { StatKind::NetUp }
    }

    /// The first read only starts the clock: a speed needs two reads.
    fn read(&mut self) -> Option<f64> {
        self.networks.refresh(true);
        let now = Instant::now();
        let bytes: u64 = self
            .networks
            .list()
            .iter()
            .filter(|(name, _)| counts_toward_total(name))
            .map(|(_, data)| if self.down { data.received() } else { data.transmitted() })
            .sum();
        let last = self.last.replace(now)?;
        Some(speed(bytes, now - last))
    }
}
