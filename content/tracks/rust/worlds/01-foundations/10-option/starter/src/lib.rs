/// half(8) -> Some(4), half(7) -> None
pub fn half(n: i32) -> Option<i32> {
    // TODO: if n is even return Some(n / 2), otherwise None
    todo!()
}

/// half_or_zero(8) -> 4, half_or_zero(7) -> 0
pub fn half_or_zero(n: i32) -> i32 {
    // TODO: ask half, falling back to 0 when it returns None
    todo!()
}

/// half_plus_one(8) -> Some(5), half_plus_one(7) -> None
pub fn half_plus_one(n: i32) -> Option<i32> {
    // TODO: ask half, then add 1 to the value inside the Some (None stays None)
    todo!()
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
