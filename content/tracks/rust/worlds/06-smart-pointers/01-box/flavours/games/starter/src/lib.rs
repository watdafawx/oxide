/// A conveyor belt: the items on one segment, then the rest of the belt. `End` ends it.
#[derive(Debug, PartialEq)]
pub enum Belt {
    // TODO: this does not build (E0072). Put the rest of the belt behind a Box.
    Segment(u32, Belt),
    End,
}

/// A longer belt: a new segment carrying `items` in front of `belt`.
pub fn extend(belt: Belt, items: u32) -> Belt {
    // TODO: a new Segment holding `items` and the old belt in a Box
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_two_segment_belt_builds() {
        let belt = Belt::Segment(8, Box::new(Belt::Segment(4, Box::new(Belt::End))));
        match belt {
            Belt::Segment(first, _) => assert_eq!(first, 8, "the first segment carries 8 items"),
            Belt::End => panic!("the belt should start with a segment"),
        }
    }

    #[test]
    fn extend_puts_new_segment_in_front() {
        let belt = extend(Belt::Segment(4, Box::new(Belt::End)), 8);
        assert_eq!(belt, Belt::Segment(8, Box::new(Belt::Segment(4, Box::new(Belt::End)))));
    }
}
