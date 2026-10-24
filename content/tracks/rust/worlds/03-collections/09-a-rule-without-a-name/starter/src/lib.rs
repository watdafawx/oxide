/// Every price plus `percent` percent tax: ([100, 500], 20) -> [120, 600]
pub fn with_tax(prices: &[u32], percent: u32) -> Vec<u32> {
    // TODO: write the tax rule as a closure that captures `percent`
    let mut out = Vec::new();
    for p in prices {
        // TODO: push the price with tax added, using your rule
        out.push(*p);
    }
    out
}

/// How many of each product to order so every shelf reaches `target`.
/// ([12, 2, 30], 20) -> [8, 18, 0]: a full shelf needs nothing.
pub fn top_up_amounts(counts: &[u32], target: u32) -> Vec<u32> {
    // TODO: write a closure that captures `target` and gives target minus the count, never below 0 (saturating_sub)
    let mut out = Vec::new();
    for n in counts {
        // TODO: push the amount to order for this count, using your rule
        out.push(*n);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn with_tax_adds_the_percent_to_every_price() {
        assert_eq!(with_tax(&[100, 500], 20), vec![120, 600]);
        assert_eq!(with_tax(&[250], 0), vec![250], "0% tax changes nothing");
        assert_eq!(with_tax(&[], 20), Vec::<u32>::new());
    }

    #[test]
    fn top_up_amounts_fill_each_shelf_to_the_target() {
        assert_eq!(top_up_amounts(&[12, 2, 30], 20), vec![8, 18, 0]);
    }
}
