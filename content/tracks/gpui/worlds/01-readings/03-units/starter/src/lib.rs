//! Kinetik's stat model.

pub mod reading;
pub mod stat;

pub use reading::Reading;
pub use stat::{ALL, StatKind};

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
            // TODO: temperatures are Celsius, NetDown and NetUp are BytesPerSec,
            // Ping is Millis. Then delete the `_` arm.
            _ => Unit::Percent,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn temps_are_celsius() {
        assert_eq!(StatKind::CpuTemp.unit(), Unit::Celsius);
        assert_eq!(StatKind::GpuTemp.unit(), Unit::Celsius);
    }

    #[test]
    fn network_and_ping_units() {
        assert_eq!(StatKind::NetDown.unit(), Unit::BytesPerSec);
        assert_eq!(StatKind::NetUp.unit(), Unit::BytesPerSec);
        assert_eq!(StatKind::Ping.unit(), Unit::Millis);
    }
}
