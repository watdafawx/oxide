/// Each item takes 1.5 seconds to smelt. smelting_seconds(10.0) -> 15.0
pub fn smelting_seconds(items: f64) -> f64 {
    0.0 // TODO: items times 1.5
}

/// Half of a stack. half_stack(5.0) -> 2.5
pub fn half_stack(count: f64) -> f64 {
    0.0 // TODO: divide by 2.0 (with the point!)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smelting_time_scales_with_items() {
        assert_eq!(smelting_seconds(10.0), 15.0);
        assert_eq!(smelting_seconds(3.0), 4.5);
    }

    #[test]
    fn half_a_stack_keeps_the_half() {
        assert_eq!(half_stack(5.0), 2.5);
        assert_eq!(half_stack(64.0), 32.0);
    }
}
