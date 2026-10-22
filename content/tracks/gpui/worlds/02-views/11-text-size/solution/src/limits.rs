//! When a value turns the warning colour.

use crate::reading::Reading;
use crate::stat::StatKind;

/// Which side of the limit is bad.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WarnWhen {
    /// Too high is bad: temperatures, usage, ping.
    Above,
    /// Too low is bad: battery, free disk space.
    Below,
}

/// One warning limit.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Limit {
    pub at: f64,
    pub when: WarnWhen,
}

/// Kinetik's warning limits.
#[derive(Debug, Clone, PartialEq)]
pub struct Limits {
    pub cpu_usage: f64,
    pub cpu_temp: f64,
    pub gpu_usage: f64,
    pub gpu_temp: f64,
    pub ping: f64,
    pub battery: f64,
}

impl Default for Limits {
    fn default() -> Self {
        Limits {
            cpu_usage: 90.0,
            cpu_temp: 85.0,
            gpu_usage: 95.0,
            gpu_temp: 83.0,
            ping: 100.0,
            battery: 20.0,
        }
    }
}

impl Limits {
    /// The limit for one kind, if it has one.
    pub fn limit_for(&self, kind: StatKind) -> Option<Limit> {
        let above = |at| Some(Limit { at, when: WarnWhen::Above });
        match kind {
            StatKind::CpuUsage => above(self.cpu_usage),
            StatKind::CpuTemp => above(self.cpu_temp),
            StatKind::GpuUsage => above(self.gpu_usage),
            StatKind::GpuTemp => above(self.gpu_temp),
            StatKind::Ping => above(self.ping),
            StatKind::Battery => Some(Limit { at: self.battery, when: WarnWhen::Below }),
            _ => None,
        }
    }
}

/// Should this reading be drawn in the warning colour?
pub fn is_warning(reading: &Reading, limits: &Limits) -> bool {
    match (reading.value, limits.limit_for(reading.kind)) {
        (Some(v), Some(limit)) => match limit.when {
            WarnWhen::Above => v >= limit.at,
            WarnWhen::Below => v <= limit.at,
        },
        _ => false,
    }
}
