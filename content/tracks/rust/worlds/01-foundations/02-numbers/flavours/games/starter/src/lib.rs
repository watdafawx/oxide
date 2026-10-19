/// Exact average of two stack counts: average_stack(3, 4) -> 3.5
pub fn average_stack(a: i32, b: i32) -> f64 {
    // TODO: convert the sum with `as f64`, then divide by 2.0
    todo!()
}

/// Keep only the low 8 bits of an item id: slot_byte(300) -> 44
pub fn slot_byte(id: u32) -> u8 {
    // TODO: cast with `as u8`
    todo!()
}

/// A hopper counter that wraps instead of crashing: wrap_count(250, 10) -> 4
pub fn wrap_count(a: u8, b: u8) -> u8 {
    // TODO: use a method on `a` that wraps around
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn average_stack_keeps_the_half() {
        assert_eq!(average_stack(3, 4), 3.5);
        assert_eq!(average_stack(10, 20), 15.0);
    }

    #[test]
    fn slot_byte_keeps_low_byte() {
        assert_eq!(slot_byte(300), 44, "300 does not fit in a u8; only the low 8 bits stay");
        assert_eq!(slot_byte(65), 65);
    }

    #[test]
    fn wrap_count_wraps_past_255() {
        assert_eq!(wrap_count(250, 10), 4, "250 + 10 = 260, which wraps past 255 to 4");
        assert_eq!(wrap_count(1, 2), 3);
    }
}
