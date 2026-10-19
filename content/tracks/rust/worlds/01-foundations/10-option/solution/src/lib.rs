/// half(8) -> Some(4), half(7) -> None
pub fn half(n: i32) -> Option<i32> {
    if n % 2 == 0 { Some(n / 2) } else { None }
}

/// half_or_zero(8) -> 4, half_or_zero(7) -> 0
pub fn half_or_zero(n: i32) -> i32 {
    half(n).unwrap_or(0)
}

/// half_plus_one(8) -> Some(5), half_plus_one(7) -> None
pub fn half_plus_one(n: i32) -> Option<i32> {
    half(n).map(|h| h + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn half_is_none_for_odd() {
        assert_eq!(half(8), Some(4));
        assert_eq!(half(7), None);
        assert_eq!(half(0), Some(0));
    }

    #[test]
    fn half_or_zero_falls_back() {
        assert_eq!(half_or_zero(8), 4);
        assert_eq!(half_or_zero(7), 0, "odd numbers have no half, so fall back to 0");
    }

    #[test]
    fn half_plus_one_maps_inside() {
        assert_eq!(half_plus_one(8), Some(5));
        assert_eq!(half_plus_one(7), None, "None stays None");
    }
}
