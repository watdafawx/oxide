/// Boxes on each slot of a three-slot shelf. shelf_total([10, 20, 30]) -> 60
pub fn shelf_total(shelf: [u32; 3]) -> u32 {
    shelf[0] + shelf[1] + shelf[2]
}

/// The last slot of a nine-slot rack. last_box([1, 2, 3, 4, 5, 6, 7, 8, 9]) -> 9
pub fn last_box(boxes: [u32; 9]) -> u32 {
    boxes[boxes.len() - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_up_a_shelf_of_three() {
        assert_eq!(shelf_total([10, 20, 30]), 60);
        assert_eq!(shelf_total([0, 0, 7]), 7);
    }

    #[test]
    fn finds_the_last_box() {
        assert_eq!(last_box([1, 2, 3, 4, 5, 6, 7, 8, 9]), 9);
        assert_eq!(last_box([9, 0, 0, 0, 0, 0, 0, 0, 4]), 4);
    }
}
