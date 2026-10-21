/// The average of two stack counts, with the half kept. average_stack(3, 4) -> 3.5
pub fn average_stack(a: u32, b: u32) -> f64 {
    0.0 // TODO: convert the sum with `as f64`, then divide by 2.0
}

/// The whole planks in a decimal amount. whole_planks(3.9) -> 3
pub fn whole_planks(planks: f64) -> u32 {
    0 // TODO: cast `planks` to a u32 with `as`
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn average_keeps_the_half() {
        assert_eq!(average_stack(3, 4), 3.5);
        assert_eq!(average_stack(10, 20), 15.0);
    }

    #[test]
    fn decimals_are_cut_off() {
        assert_eq!(whole_planks(3.9), 3, "casting cuts, it does not round");
        assert_eq!(whole_planks(12.0), 12);
    }
}
