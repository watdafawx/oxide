//! Where readings come from.

use crate::stat::StatKind;

/// Something that can read one stat. Real ones ask Windows; test ones play a script.
pub trait Sensor {
    /// Which stat this sensor reads.
    fn kind(&self) -> StatKind;
    /// The current value, or None when it cannot be read.
    fn read(&mut self) -> Option<f64>;
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
