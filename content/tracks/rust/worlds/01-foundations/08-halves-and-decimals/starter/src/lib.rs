/// Each sheet takes 1.5 ml of ink. ink_ml(10.0) -> 15.0
pub fn ink_ml(sheets: f64) -> f64 {
    0.0 // TODO: sheets times 1.5
}

/// Half of a pallet's sheets. half_pallet(5.0) -> 2.5
pub fn half_pallet(sheets: f64) -> f64 {
    0.0 // TODO: divide by 2.0 (with the point!)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ink_scales_with_sheets() {
        assert_eq!(ink_ml(10.0), 15.0);
        assert_eq!(ink_ml(3.0), 4.5);
    }

    #[test]
    fn half_a_pallet_keeps_the_half() {
        assert_eq!(half_pallet(5.0), 2.5);
        assert_eq!(half_pallet(64.0), 32.0);
    }
}
