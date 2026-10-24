//! Kinetik's stat model.

pub mod stat;

pub use stat::{ALL, StatKind};

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
            None => String::new(), // TODO: no value shows two dashes, not empty text
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_temp_shows_dashes() {
        let r = Reading { kind: StatKind::CpuTemp, value: None };
        assert_eq!(r.text(), "--");
    }
}
