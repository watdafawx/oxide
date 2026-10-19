/// The counts in the first and last hotbar slot.
/// hotbar_ends(&[4, 5, 6, 7]) -> (4, 7)
pub fn hotbar_ends(hotbar: &[u32]) -> (u32, u32) {
    // TODO: index with 0 and with `hotbar.len() - 1`
    todo!()
}

/// Swap two slots: swap_slots((1, 2)) -> (2, 1)
pub fn swap_slots(slots: (u32, u32)) -> (u32, u32) {
    // TODO: destructure with `let (a, b) = slots;`, return them flipped
    todo!()
}

/// Items in one row of a crafting grid: row_total([1, 2, 3]) -> 6
pub fn row_total(row: [u32; 3]) -> u32 {
    // TODO: destructure with `let [a, b, c] = row;` and add
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hotbar_ends_picks_first_and_last() {
        assert_eq!(hotbar_ends(&[4, 5, 6, 7]), (4, 7));
        assert_eq!(hotbar_ends(&[9]), (9, 9), "one slot is both first and last");
    }

    #[test]
    fn swap_slots_flips_order() {
        assert_eq!(swap_slots((1, 2)), (2, 1));
    }

    #[test]
    fn crafting_row_totals_three_slots() {
        assert_eq!(row_total([1, 2, 3]), 6);
        assert_eq!(row_total([64, 0, 16]), 80);
    }
}
