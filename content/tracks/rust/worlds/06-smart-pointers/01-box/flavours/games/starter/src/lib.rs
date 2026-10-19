/// A conveyor belt: each segment holds some items, then continues into the rest of the belt.
/// The Box gives the recursive part a known size.
#[derive(Debug, PartialEq)]
pub enum Belt {
    Segment(u32, Box<Belt>),
    End,
}

use Belt::{End, Segment};

/// Returns a longer belt with a new segment carrying `items` at the front.
pub fn extend(belt: Belt, items: u32) -> Belt {
    // TODO: Segment(items, Box::new(belt))
    todo!()
}

/// Adds up the items on every segment. An empty belt carries 0.
pub fn total_items(belt: &Belt) -> u32 {
    // TODO: match on belt. Segment(n, rest) => n + total_items(rest), End => 0
    todo!()
}

/// Counts the segments.
pub fn segment_count(belt: &Belt) -> usize {
    // TODO: same shape as total_items, but each Segment counts as 1
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn three() -> Belt {
        Segment(1, Box::new(Segment(2, Box::new(Segment(3, Box::new(End))))))
    }

    #[test]
    fn extend_puts_new_segment_in_front() {
        let belt = extend(Segment(2, Box::new(End)), 1);
        assert_eq!(belt, Segment(1, Box::new(Segment(2, Box::new(End)))));
    }

    #[test]
    fn total_items_adds_every_segment() {
        assert_eq!(total_items(&three()), 6);
        assert_eq!(total_items(&End), 0, "an empty belt carries nothing");
    }

    #[test]
    fn segment_count_counts_segments() {
        assert_eq!(segment_count(&three()), 3);
        assert_eq!(segment_count(&End), 0, "an empty belt has no segments");
    }
}
