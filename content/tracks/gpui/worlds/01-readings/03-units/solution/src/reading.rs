//! One measurement, and the text the bar shows for it.

use crate::StatKind;

/// One measurement: which stat, and its value if Kinetik could read it.
#[derive(Debug, Clone, PartialEq)]
pub struct Reading {
    pub kind: StatKind,
    pub value: Option<f64>,
}

impl Reading {
    /// The text the bar shows for this reading.
    pub fn text(&self) -> String {
        match self.value {
            Some(v) => format!("{v}"),
            None => "--".to_string(),
        }
    }
}
