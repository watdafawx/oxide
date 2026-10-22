//! What Kinetik can measure.

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
            StatKind::GpuUsage => "GPU",
            StatKind::GpuTemp => "GPU",
            StatKind::NetDown => "NET",
            StatKind::NetUp => "NET",
            StatKind::Ping => "PING",
            StatKind::Battery => "BAT",
        }
    }
}
