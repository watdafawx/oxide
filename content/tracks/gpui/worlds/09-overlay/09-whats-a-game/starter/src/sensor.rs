//! Where readings come from.

use crate::stat::StatKind;

/// Something that can read one stat, on any thread. Real ones ask Windows; test ones play a script.
pub trait Sensor: Send {
    /// Which stat this sensor reads.
    fn kind(&self) -> StatKind;
    /// The current value, or None when it cannot be read.
    fn read(&mut self) -> Option<f64>;
    /// Why the last read gave None, if the sensor knows.
    fn missing(&self) -> Option<Missing> {
        None
    }
}

/// A stand-in sensor for tests and demos: gives its script's values in order, then None.
pub struct ScriptedSensor {
    pub kind: StatKind,
    pub script: Vec<f64>,
    pub next: usize,
}

impl ScriptedSensor {
    pub fn new(kind: StatKind, script: Vec<f64>) -> Self {
        ScriptedSensor { kind, script, next: 0 }
    }
}

impl Sensor for ScriptedSensor {
    fn kind(&self) -> StatKind {
        self.kind
    }

    fn read(&mut self) -> Option<f64> {
        let value = self.script.get(self.next).copied();
        self.next += 1;
        value
    }
}

/// Why a stat has no number. The hover card says it in words.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Missing {
    /// Windows only gives this to apps running as administrator.
    NeedsAdmin,
    /// This PC has no such device: no battery, no NVIDIA card, no thermal zone.
    NoDevice,
    /// It did not answer in time.
    Timeout,
}

/// The hover card's words for a missing number.
pub fn missing_text(missing: Missing) -> &'static str {
    match missing {
        Missing::NeedsAdmin => "needs administrator",
        Missing::NoDevice => "not on this PC",
        Missing::Timeout => "no answer",
    }
}
