#![no_std]
//! A fixed-capacity FIFO queue. Only `core` is available here: no Vec, no heap.

#[cfg(test)]
extern crate std;

/// Holds up to `N` items in a plain array.
pub struct RingBuffer<T, const N: usize> {
    buf: [T; N],
    /// Index of the oldest item.
    head: usize,
    /// How many items are stored.
    len: usize,
}

impl<T: Copy + Default, const N: usize> RingBuffer<T, N> {
    pub fn new() -> Self {
        Self { buf: [T::default(); N], head: 0, len: 0 }
    }

    pub fn capacity(&self) -> usize {
        N
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn is_full(&self) -> bool {
        self.len == N
    }

    /// Adds an item at the back. A full buffer refuses and hands the value back.
    pub fn push(&mut self, value: T) -> Result<(), T> {
        // TODO: refuse when full (hand the value back in Err); otherwise write it into the first
        // free slot after the stored items, wrapping past the end, and count it
        let _ = value;
        todo!()
    }

    /// Removes and returns the oldest item.
    pub fn pop(&mut self) -> Option<T> {
        // TODO: None when empty; otherwise take the oldest item, move head on by one (wrapping),
        // and count one fewer
        todo!()
    }
}

impl<T: Copy + Default, const N: usize> Default for RingBuffer<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::vec::Vec;

    #[test]
    fn push_and_pop_in_order() {
        let mut rb: RingBuffer<u8, 4> = RingBuffer::new();
        assert_eq!(rb.push(1), Ok(()));
        assert_eq!(rb.push(2), Ok(()));
        assert_eq!(rb.len(), 2);
        assert_eq!(rb.pop(), Some(1));
        assert_eq!(rb.pop(), Some(2));
        assert_eq!(rb.pop(), None);
        assert!(rb.is_empty());
    }

    #[test]
    fn push_to_full_returns_err() {
        let mut rb: RingBuffer<u8, 2> = RingBuffer::new();
        rb.push(1).unwrap();
        rb.push(2).unwrap();
        assert!(rb.is_full());
        assert_eq!(rb.push(3), Err(3), "a full buffer must hand the value back");
        assert_eq!(rb.len(), 2);
    }

    #[test]
    fn wraps_around_the_end() {
        let mut rb: RingBuffer<u32, 3> = RingBuffer::new();
        let mut seen = Vec::new();
        for round in 0..10u32 {
            rb.push(round).unwrap();
            rb.push(round + 100).unwrap();
            seen.push(rb.pop().unwrap());
            seen.push(rb.pop().unwrap());
        }
        let expected: Vec<u32> = (0..10).flat_map(|r| [r, r + 100]).collect();
        assert_eq!(seen, expected);
        assert_eq!(rb.capacity(), 3);
    }
}
