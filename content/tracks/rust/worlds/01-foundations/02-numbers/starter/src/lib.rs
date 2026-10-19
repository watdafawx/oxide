/// Exact average: average(3, 4) -> 3.5
pub fn average(a: i32, b: i32) -> f64 {
    // TODO: convert the sum with `as f64`, then divide by 2.0
    todo!()
}

/// Keep only the low 8 bits: low_byte(300) -> 44
pub fn low_byte(n: u32) -> u8 {
    // TODO: cast with `as u8`
    todo!()
}

/// Add without panicking on overflow: wrap_add(250, 10) -> 4
pub fn wrap_add(a: u8, b: u8) -> u8 {
    // TODO: use a method on `a` that wraps around
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn average_keeps_the_fraction() {
        assert_eq!(average(3, 4), 3.5);
        assert_eq!(average(10, 20), 15.0);
    }

    #[test]
    fn low_byte_truncates() {
        assert_eq!(low_byte(300), 44, "300 does not fit in a u8; only the low 8 bits stay");
        assert_eq!(low_byte(65), 65);
    }

    #[test]
    fn wrap_add_wraps_around() {
        assert_eq!(wrap_add(250, 10), 4, "250 + 10 = 260, which wraps past 255 to 4");
        assert_eq!(wrap_add(1, 2), 3);
    }
}
