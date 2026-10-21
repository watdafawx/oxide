/// Items in each slot of a crafting row. row_total([10, 20, 30]) -> 60
pub fn row_total(row: [u32; 3]) -> u32 {
    row[0] + row[1] + row[2]
}

/// The last slot of the hotbar. last_slot([1, 2, 3, 4, 5, 6, 7, 8, 9]) -> 9
pub fn last_slot(hotbar: [u32; 9]) -> u32 {
    hotbar[hotbar.len() - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_up_a_crafting_row() {
        assert_eq!(row_total([10, 20, 30]), 60);
        assert_eq!(row_total([0, 0, 7]), 7);
    }

    #[test]
    fn finds_the_last_hotbar_slot() {
        assert_eq!(last_slot([1, 2, 3, 4, 5, 6, 7, 8, 9]), 9);
        assert_eq!(last_slot([9, 0, 0, 0, 0, 0, 0, 0, 4]), 4);
    }
}
