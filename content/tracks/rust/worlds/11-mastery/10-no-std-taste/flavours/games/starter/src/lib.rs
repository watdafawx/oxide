#![no_std]
//! A fixed-capacity FIFO queue for chunk ids. Only `core` is available here: no Vec, no heap.

#[cfg(test)]
extern crate std;

/// Holds up to `N` items in a plain array.
pub struct LoadQueue<T, const N: usize> {
    buf: [T; N],
    /// Index of the oldest item.
    head: usize,
    /// How many items are stored.
    len: usize,
}

impl<T: Copy + Default, const N: usize> LoadQueue<T, N> {
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

    /// Adds an item at the back. A full queue refuses and hands the value back.
    pub fn enqueue(&mut self, value: T) -> Result<(), T> {
        // TODO: refuse when full; otherwise write at (head + len) % N and bump len
        let _ = value;
        todo!()
    }

    /// Removes and returns the oldest item.
    pub fn dequeue(&mut self) -> Option<T> {
        // TODO: None when empty; otherwise read buf[head], advance head (wrapping), shrink len
        todo!()
    }
}

impl<T: Copy + Default, const N: usize> Default for LoadQueue<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::vec::Vec;

    #[test]
    fn dequeues_in_arrival_order() {
        let mut q: LoadQueue<u8, 4> = LoadQueue::new();
        assert_eq!(q.enqueue(1), Ok(()));
        assert_eq!(q.enqueue(2), Ok(()));
        assert_eq!(q.len(), 2);
        assert_eq!(q.dequeue(), Some(1));
        assert_eq!(q.dequeue(), Some(2));
        assert_eq!(q.dequeue(), None);
        assert!(q.is_empty());
    }

    #[test]
    fn enqueue_on_a_full_queue_returns_err() {
        let mut q: LoadQueue<u8, 2> = LoadQueue::new();
        q.enqueue(1).unwrap();
        q.enqueue(2).unwrap();
        assert!(q.is_full());
        assert_eq!(q.enqueue(3), Err(3), "a full queue must hand the value back");
        assert_eq!(q.len(), 2);
    }

    #[test]
    fn queue_wraps_around_the_end() {
        let mut q: LoadQueue<u32, 3> = LoadQueue::new();
        let mut seen = Vec::new();
        for round in 0..10u32 {
            q.enqueue(round).unwrap();
            q.enqueue(round + 100).unwrap();
            seen.push(q.dequeue().unwrap());
            seen.push(q.dequeue().unwrap());
        }
        let expected: Vec<u32> = (0..10).flat_map(|r| [r, r + 100]).collect();
        assert_eq!(seen, expected);
        assert_eq!(q.capacity(), 3);
    }
}
