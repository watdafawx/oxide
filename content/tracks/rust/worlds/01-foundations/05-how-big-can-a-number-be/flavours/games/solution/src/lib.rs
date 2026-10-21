/// The change in a stack: stack_change(10, 4) -> -6 (six items left the stack).
pub fn stack_change(before: i32, after: i32) -> i32 {
    after - before
}

/// The id of a block added by a mod. Ids run from 0 to 255.
pub fn modded_block_id() -> u8 {
    200
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dropping_items_is_negative() {
        assert_eq!(stack_change(10, 4), -6);
        assert_eq!(stack_change(4, 10), 6);
    }

    #[test]
    fn a_modded_block_id_can_reach_200() {
        assert_eq!(modded_block_id(), 200);
    }
}
