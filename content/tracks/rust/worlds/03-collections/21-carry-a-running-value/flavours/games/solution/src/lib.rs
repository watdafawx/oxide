/// Each armor piece blocks a percent of what is left: 200.0 with [10.0, 50.0] -> 200 * 0.9 * 0.5 = 90.0
pub fn damage_after_armor(damage: f64, reductions: &[f64]) -> f64 {
    reductions.iter().fold(damage, |left, off| left * (1.0 - off / 100.0))
}

/// Take every hit off the start durability, one after the other, never going below 0.
pub fn durability_after_hits(start: u32, hits: &[u32]) -> u32 {
    hits.iter().fold(start, |left, h| left.saturating_sub(*h))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn damage_after_armor_applies_each_piece() {
        let got = damage_after_armor(200.0, &[10.0, 50.0]);
        assert!((got - 90.0).abs() < 1e-9, "expected 90.0, got {got}");
        assert_eq!(damage_after_armor(8.0, &[]), 8.0, "no armor, full damage");
    }

    #[test]
    fn durability_after_hits_never_goes_below_zero() {
        assert_eq!(durability_after_hits(250, &[30, 20]), 200);
        assert_eq!(durability_after_hits(5, &[3, 4, 1]), 0, "a broken tool stays at 0");
    }
}
