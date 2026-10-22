//! Kinetik's stat model: what the bar can show.

/// One thing Kinetik can measure. Kinetik Stats has 44; your bar starts with these nine.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StatKind {
    CpuUsage,
    CpuTemp,
    RamUsed,
    GpuUsage,
    GpuTemp,
    NetDown,
    NetUp,
    Ping,
    Battery,
}

/// Every kind, in a fixed order.
pub const ALL: [StatKind; 9] = [
    StatKind::CpuUsage,
    StatKind::CpuTemp,
    StatKind::RamUsed,
    StatKind::GpuUsage,
    StatKind::GpuTemp,
    StatKind::NetDown,
    StatKind::NetUp,
    StatKind::Ping,
    StatKind::Battery,
];

impl StatKind {
    /// The short text label Kinetik shows when icons are off.
    pub fn label(self) -> &'static str {
        match self {
            StatKind::CpuUsage => "CPU",
            StatKind::CpuTemp => "CPU",
            StatKind::RamUsed => "RAM",
            // TODO: GpuUsage and GpuTemp are "GPU", NetDown and NetUp are "NET",
            // Ping is "PING", Battery is "BAT". Then delete the `_` arm.
            _ => "?",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gpu_kinds_say_gpu() {
        assert_eq!(StatKind::GpuUsage.label(), "GPU");
        assert_eq!(StatKind::GpuTemp.label(), "GPU");
    }

    #[test]
    fn net_ping_and_battery_labels() {
        assert_eq!(StatKind::NetDown.label(), "NET");
        assert_eq!(StatKind::NetUp.label(), "NET");
        assert_eq!(StatKind::Ping.label(), "PING");
        assert_eq!(StatKind::Battery.label(), "BAT");
    }
}
