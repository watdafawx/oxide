/// A conveyor belt: the items on one segment, then the rest of the belt. `End` ends it.
#[derive(Debug, PartialEq)]
pub enum Belt {
    Segment(u32, Box<Belt>),
    End,
}

/// The items on every segment added up. An empty belt carries 0.
pub fn total_items(belt: &Belt) -> u32 {
    // TODO: match on belt: End is 0, a Segment is its items plus total_items of the rest
    todo!()
}

/// How many segments the belt has.
pub fn segment_count(belt: &Belt) -> u32 {
    // TODO: same shape as total_items, but each Segment counts as 1
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn main_bus() -> Belt {
        Belt::Segment(8, Box::new(Belt::Segment(4, Box::new(Belt::Segment(3, Box::new(Belt::End))))))
    }

    #[test]
    fn total_items_adds_every_segment() {
        assert_eq!(total_items(&main_bus()), 15, "8 + 4 + 3");
        assert_eq!(total_items(&Belt::End), 0, "an empty belt carries nothing");
    }

    #[test]
    fn segment_count_counts_segments() {
        assert_eq!(segment_count(&main_bus()), 3);
        assert_eq!(segment_count(&Belt::End), 0, "an empty belt has no segments");
    }
}
