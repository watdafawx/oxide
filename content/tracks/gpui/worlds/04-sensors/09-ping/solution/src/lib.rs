//! Kinetik Stats: the bar, and the sensors that feed it.

pub mod demo;
pub mod format;
pub mod history;
pub mod limits;
pub mod order;
pub mod pills;
pub mod reading;
pub mod stat;

pub use format::{format_bytes, format_percent, format_speed};
pub use history::{History, Summary};
pub use limits::{Limit, Limits, WarnWhen, is_warning};
pub use order::{DividerStyle, Slot, default_order, move_up};
pub use pills::{BarItem, Column, Pill, Snapshot, bar_items, hover_summary, two_line_columns};
pub use reading::Reading;
pub use stat::{StatKind, Unit};
pub mod bar;
pub mod real;
pub mod sensor;
pub mod view;
pub mod disk;
pub mod gpu;
pub mod net;
pub mod thermal;
pub mod power;

pub use bar::{StatsBar, clamp_interval, wanted_kinds};
pub use real::{CpuSensor, RamSensor};
pub use sensor::{ScriptedSensor, Sensor};
pub use view::{LabelMode, Look};
pub use net::{NetSpeedSensor, counts_toward_total, speed};
pub use disk::DiskSensor;
pub use gpu::{GpuInfo, GpuSensor, NvidiaGpu, Vendor, pick_gpu};
pub use thermal::{ThermalSensor, acpi_to_celsius};
pub use power::{Battery, BatterySensor, decode_battery, read_power_status};

use std::net::{SocketAddr, TcpStream};
use std::time::{Duration, Instant};

/// How long a TCP connection to `addr` takes, or None when it does not connect within `timeout`.
pub fn ping_once(addr: SocketAddr, timeout: Duration) -> Option<Duration> {
    let start = Instant::now();
    TcpStream::connect_timeout(&addr, timeout).ok()?;
    Some(start.elapsed())
}

/// Ping in milliseconds to a server you choose (Kinetik's Stats page has the address box).
pub struct PingSensor {
    addr: SocketAddr,
    timed_out: bool,
}

impl PingSensor {
    /// Cloudflare's DNS on the HTTPS port: answers almost everywhere.
    pub fn new() -> Self {
        PingSensor::to(SocketAddr::from(([1, 1, 1, 1], 443)))
    }

    pub fn to(addr: SocketAddr) -> Self {
        PingSensor { addr, timed_out: false }
    }
}

impl Default for PingSensor {
    fn default() -> Self {
        PingSensor::new()
    }
}

impl Sensor for PingSensor {
    fn kind(&self) -> StatKind {
        StatKind::Ping
    }

    fn read(&mut self) -> Option<f64> {
        let time = ping_once(self.addr, Duration::from_secs(1));
        self.timed_out = time.is_none();
        time.map(|t| t.as_secs_f64() * 1000.0)
    }
}

/// Every sensor this PC has so far; the rest of the bar is scripted.
pub fn live_sensors() -> Vec<Box<dyn Sensor>> {
    let mut sensors: Vec<Box<dyn Sensor>> = vec![
        Box::new(CpuSensor::new()),
        Box::new(RamSensor::new()),
        Box::new(NetSpeedSensor::download()),
        Box::new(NetSpeedSensor::upload()),
        Box::new(DiskSensor::new()),
        Box::new(GpuSensor::usage()),
        Box::new(GpuSensor::temperature()),
        Box::new(ThermalSensor::new()),
        Box::new(BatterySensor),
        Box::new(PingSensor::new()),
    ];
    let real: Vec<StatKind> = sensors.iter().map(|s| s.kind()).collect();
    for kind in [StatKind::CpuUsage, StatKind::CpuTemp, StatKind::GpuUsage, StatKind::GpuTemp, StatKind::RamUsed, StatKind::NetDown, StatKind::NetUp, StatKind::Ping] {
        if !real.contains(&kind) {
            let script = (0..300).map(|i| 20.0 + (i % 7) as f64 * 5.0).collect();
            sensors.push(Box::new(ScriptedSensor::new(kind, script)));
        }
    }
    sensors
}
#[cfg(test)]
#[allow(unused_imports, dead_code)]
mod tests {
    use super::*;
    use gpui::{TestAppContext, prelude::*};
    use std::sync::Arc;
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::{cell::Cell, rc::Rc};

    fn wait(cx: &mut TestAppContext, ms: u64) {
        cx.executor().advance_clock(std::time::Duration::from_millis(ms));
        cx.run_until_parked();
    }

    #[test]
    fn local_server_answers() {
        let server = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let time = ping_once(server.local_addr().unwrap(), Duration::from_secs(1)).expect("the local server answers");
        assert!(time < Duration::from_secs(1));
    }

    #[test]
    fn unreachable_host_is_none() {
        // 10.255.255.1 is a private address nobody on this network uses.
        let addr = SocketAddr::from(([10, 255, 255, 1], 9));
        assert_eq!(ping_once(addr, Duration::from_millis(300)), None);
    }
}
