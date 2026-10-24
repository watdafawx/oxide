//! The operation meter. Nothing to edit here.
use std::cell::Cell;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

/// A `String -> u32` map that counts every operation on it (get, contains, insert, entry).
#[derive(Debug, Default)]
pub struct Counter {
    map: HashMap<String, u32>,
    ops: Cell<usize>,
}

impl Counter {
    pub fn new() -> Self {
        Self::default()
    }

    /// How many operations were done so far. Reading this is free.
    pub fn ops(&self) -> usize {
        self.ops.get()
    }

    fn tick(&self) {
        self.ops.set(self.ops.get() + 1);
    }

    pub fn contains(&self, key: &str) -> bool {
        self.tick();
        self.map.contains_key(key)
    }

    pub fn get(&self, key: &str) -> Option<u32> {
        self.tick();
        self.map.get(key).copied()
    }

    pub fn insert(&mut self, key: String, value: u32) {
        self.tick();
        self.map.insert(key, value);
    }

    pub fn entry(&mut self, key: String) -> Entry<'_, String, u32> {
        self.tick();
        self.map.entry(key)
    }
}
