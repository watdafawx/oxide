/// A slot is (item id, count). slot_count((5, 64)) -> 64
pub fn slot_count(slot: (u32, u32)) -> u32 {
    let (_id, count) = slot;
    count
}

/// Craft planks: four per log. craft_planks(3) -> (3, 12)
pub fn craft_planks(logs: u32) -> (u32, u32) {
    (logs, logs * 4)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_count_of_a_slot() {
        assert_eq!(slot_count((5, 64)), 64);
        assert_eq!(slot_count((9, 0)), 0);
    }

    #[test]
    fn crafting_returns_both_numbers() {
        assert_eq!(craft_planks(3), (3, 12));
        assert_eq!(craft_planks(0), (0, 0));
    }
}
