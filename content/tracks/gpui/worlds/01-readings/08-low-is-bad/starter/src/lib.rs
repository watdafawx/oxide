//! Kinetik's stat model.

pub mod format;
pub mod limits;
pub mod reading;
pub mod stat;

pub use format::{format_bytes, format_percent, format_speed};
pub use limits::{Limit, Limits, WarnWhen};
pub use reading::Reading;
pub use stat::{ALL, StatKind, Unit};

/// Should this reading be drawn in the warning colour?
pub fn is_warning(reading: &Reading, limits: &Limits) -> bool {
    match (reading.value, limits.limit_for(reading.kind)) {
        // TODO: Below limits warn when v <= limit.at. Match on limit.when.
        (Some(v), Some(limit)) => v >= limit.at,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn battery(value: f64) -> Reading {
        Reading { kind: StatKind::Battery, value: Some(value) }
    }

    #[test]
    fn battery_at_15_warns() {
        assert!(is_warning(&battery(15.0), &Limits::default()));
    }

    #[test]
    fn battery_at_80_is_fine() {
        assert!(!is_warning(&battery(80.0), &Limits::default()));
        let hot = Reading { kind: StatKind::CpuTemp, value: Some(90.0) };
        assert!(is_warning(&hot, &Limits::default()), "a hot CPU still warns");
    }
}
