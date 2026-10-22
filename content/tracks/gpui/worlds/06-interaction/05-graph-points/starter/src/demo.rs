//! A fixed snapshot to draw while the bar has no sensors yet.

use crate::pills::Snapshot;
use crate::stat::StatKind;

/// A busy gaming PC: hot CPU, low battery, no ping reading.
pub fn demo_snapshot() -> Snapshot {
    Snapshot::from([
        (StatKind::CpuUsage, 13.0),
        (StatKind::CpuTemp, 88.0),
        (StatKind::RamUsed, 8_589_934_592.0),
        (StatKind::GpuUsage, 97.0),
        (StatKind::GpuTemp, 71.0),
        (StatKind::NetDown, 3_565_158.0),
        (StatKind::NetUp, 52_000.0),
        (StatKind::Battery, 15.0),
    ])
}
