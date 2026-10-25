/// A new player's hotbar: 1 wooden sword, 16 bread, 8 torches.
pub fn starter_kit() -> Vec<u32> {
    vec![1, 16, 8]
}

/// The starter kit after picking up a full stack of cobblestone (64), which goes in the next slot.
pub fn kit_after_pickup() -> Vec<u32> {
    let mut kit = starter_kit();
    kit.push(64);
    kit
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starter_kit_has_three_stacks() {
        assert_eq!(starter_kit(), vec![1, 16, 8]);
    }

    #[test]
    fn a_pickup_goes_on_the_end() {
        assert_eq!(kit_after_pickup(), vec![1, 16, 8, 64], "the cobblestone goes in the next slot");
    }
}
