//! Kinetik's stat model.

pub mod format;
pub mod reading;
pub mod stat;

pub use format::{format_bytes, format_percent, format_speed};
pub use reading::Reading;
pub use stat::{ALL, StatKind, Unit};

/// Kinetik's warning limits: at or above these, a value turns the warning colour.
#[derive(Debug, Clone, PartialEq)]
pub struct Limits {
    pub cpu_usage: f64,
    pub cpu_temp: f64,
    pub gpu_usage: f64,
    pub gpu_temp: f64,
    pub ping: f64,
}

impl Default for Limits {
    fn default() -> Self {
        Limits { cpu_usage: 90.0, cpu_temp: 85.0, gpu_usage: 95.0, gpu_temp: 83.0, ping: 100.0 }
    }
}

impl Limits {
    /// The limit for one kind, if it has one.
    pub fn limit_for(&self, kind: StatKind) -> Option<f64> {
        match kind {
            StatKind::CpuUsage => Some(self.cpu_usage),
            StatKind::CpuTemp => Some(self.cpu_temp),
            StatKind::GpuUsage => Some(self.gpu_usage),
            StatKind::GpuTemp => Some(self.gpu_temp),
            StatKind::Ping => Some(self.ping),
            _ => None,
        }
    }
}

/// Should this reading be drawn in the warning colour?
pub fn is_warning(reading: &Reading, limits: &Limits) -> bool {
    let _ = (reading, limits);
    false // TODO: match on (reading.value, limits.limit_for(reading.kind)):
          // both Some -> value >= limit, anything else -> false
}

#[cfg(test)]
mod tests {
    use super::*;

    fn reading(kind: StatKind, value: Option<f64>) -> Reading {
        Reading { kind, value }
    }

    #[test]
    fn cpu_at_90_degrees_is_hot() {
        let limits = Limits::default();
        assert!(is_warning(&reading(StatKind::CpuTemp, Some(90.0)), &limits));
        assert!(is_warning(&reading(StatKind::CpuTemp, Some(85.0)), &limits), "85 is at the limit: warn");
    }

    #[test]
    fn fine_or_unknown_is_not_a_warning() {
        let limits = Limits::default();
        assert!(!is_warning(&reading(StatKind::Ping, Some(20.0)), &limits));
        assert!(!is_warning(&reading(StatKind::CpuTemp, None), &limits));
        assert!(!is_warning(&reading(StatKind::NetDown, Some(1e9)), &limits));
    }
}
