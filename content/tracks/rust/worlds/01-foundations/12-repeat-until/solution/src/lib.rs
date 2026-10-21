/// How many scoops of `scoop` sheets empty a pile? scoops_to_empty(10, 3) -> 4 (10, 7, 4, 1, gone)
pub fn scoops_to_empty(pile: i32, scoop: i32) -> u32 {
    let mut pile = pile;
    let mut scoops = 0;
    while pile > 0 {
        pile -= scoop;
        scoops += 1;
    }
    scoops
}

/// How many full boxes can `sheets` fill, `per_box` sheets each? boxes_filled(100, 24) -> 4
pub fn boxes_filled(sheets: u32, per_box: u32) -> u32 {
    let mut sheets = sheets;
    let mut boxes = 0;
    while sheets >= per_box {
        sheets -= per_box;
        boxes += 1;
    }
    boxes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scoops_until_the_pile_is_gone() {
        assert_eq!(scoops_to_empty(10, 3), 4);
        assert_eq!(scoops_to_empty(9, 3), 3);
        assert_eq!(scoops_to_empty(0, 3), 0, "an empty pile needs no scoops");
    }

    #[test]
    fn boxes_fill_until_sheets_run_out() {
        assert_eq!(boxes_filled(100, 24), 4);
        assert_eq!(boxes_filled(23, 24), 0);
        assert_eq!(boxes_filled(48, 24), 2);
    }
}
