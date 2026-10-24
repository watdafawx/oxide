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

/// What a number means, so it can be written the right way.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Unit {
    Percent,
    Celsius,
    Bytes,
    BytesPerSec,
    Millis,
}

impl StatKind {
    /// The unit this kind's numbers are in.
    pub fn unit(self) -> Unit {
        match self {
            StatKind::CpuUsage => Unit::Percent,
            StatKind::GpuUsage => Unit::Percent,
            StatKind::Battery => Unit::Percent,
            StatKind::RamUsed => Unit::Bytes,
            StatKind::CpuTemp => Unit::Celsius,
            StatKind::GpuTemp => Unit::Celsius,
            StatKind::NetDown => Unit::BytesPerSec,
            StatKind::NetUp => Unit::BytesPerSec,
            StatKind::Ping => Unit::Millis,
        }
    }
}
