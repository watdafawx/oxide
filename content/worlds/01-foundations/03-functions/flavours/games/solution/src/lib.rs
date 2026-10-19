/// A crafting grid is `side` slots wide and tall: grid_slots(3) -> 9
pub fn grid_slots(side: u32) -> u32 {
    side * side
}

/// item_count(3, 64) -> 192 items in 3 full stacks
pub fn item_count(stacks: u32, size: u32) -> u32 {
    stacks * size
}

/// inventory_line(3, 64) -> "3x64 = 192"
pub fn inventory_line(stacks: u32, size: u32) -> String {
    let total = item_count(stacks, size);
    format!("{stacks}x{size} = {total}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn square_multiplies() {
        assert_eq!(grid_slots(3), 9, "the crafting table is 3x3");
        assert_eq!(grid_slots(2), 4, "the inventory grid is 2x2");
    }

    #[test]
    fn area_multiplies_sides() {
        assert_eq!(item_count(3, 64), 192);
        assert_eq!(item_count(0, 64), 0);
    }

    #[test]
    fn summary_reuses_area() {
        assert_eq!(inventory_line(3, 64), "3x64 = 192");
        assert_eq!(inventory_line(2, 16), "2x16 = 32");
    }
}
