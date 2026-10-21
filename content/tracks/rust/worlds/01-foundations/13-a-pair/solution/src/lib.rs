/// A slot is (kind of box, how many). box_count((5, 64)) -> 64
pub fn box_count(slot: (u32, u32)) -> u32 {
    let (_kind, count) = slot;
    count
}

/// Pack crates into sheets: four sheets per crate. pack(3) -> (3, 12)
pub fn pack(crates: u32) -> (u32, u32) {
    (crates, crates * 4)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_count_of_a_slot() {
        assert_eq!(box_count((5, 64)), 64);
        assert_eq!(box_count((9, 0)), 0);
    }

    #[test]
    fn packing_returns_both_numbers() {
        assert_eq!(pack(3), (3, 12));
        assert_eq!(pack(0), (0, 0));
    }
}
