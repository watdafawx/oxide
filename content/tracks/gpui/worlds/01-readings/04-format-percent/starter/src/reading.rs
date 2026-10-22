//! One measurement, and the text the bar shows for it.

use crate::format_percent;
use crate::stat::{StatKind, Unit};

/// One measurement: which stat, and its value if Kinetik could read it.
#[derive(Debug, Clone, PartialEq)]
pub struct Reading {
    pub kind: StatKind,
    pub value: Option<f64>,
}

impl Reading {
    /// The text the bar shows for this reading.
    pub fn text(&self) -> String {
        let Some(v) = self.value else {
            return "--".to_string();
        };
        match self.kind.unit() {
            Unit::Percent => format_percent(v),
            Unit::Celsius => format!("{v:.0}°C"),
            Unit::Millis => format!("{v:.0} ms"),
            Unit::Bytes => format!("{v}"),
            Unit::BytesPerSec => format!("{v}"),
        }
    }
}
