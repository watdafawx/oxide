/// How many counts pass `rule`? count_where(&[12, 2, 1], |n| n < 5) -> 2
pub fn count_where(counts: &[u32], rule: impl Fn(u32) -> bool) -> u32 {
    let mut matches = 0;
    for n in counts {
        // TODO: add 1 to `matches` when the rule says true for this count
    }
    matches
}

/// Every price, changed by `change`. change_all(&[100, 400], |p| p / 2) -> [50, 200]
pub fn change_all(prices: &[u32], change: impl Fn(u32) -> u32) -> Vec<u32> {
    let mut out = Vec::new();
    for p in prices {
        // TODO: push the price after `change` has been applied to it
        out.push(*p);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_where_uses_the_callers_rule() {
        let counts = [12, 2, 1, 0, 40];
        assert_eq!(count_where(&counts, |n| n < 5), 3, "2, 1 and 0 are low");
        assert_eq!(count_where(&counts, |n| n == 0), 1, "one product is sold out");
        let limit = 10;
        assert_eq!(count_where(&counts, |n| n > limit), 2, "a rule may capture");
    }

    #[test]
    fn change_all_applies_the_rule_to_every_price() {
        assert_eq!(change_all(&[100, 400], |p| p / 2), vec![50, 200]);
        assert_eq!(change_all(&[100, 400], |p| p + 5), vec![105, 405]);
    }
}
