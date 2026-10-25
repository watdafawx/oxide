/// Every price with `percent_off` percent taken off: ([100, 500], 20) -> [80, 400]
pub fn with_discount(prices: &[u32], percent_off: u32) -> Vec<u32> {
    prices.iter().map(|p| p - p * percent_off / 100).collect()
}

/// Every product name in capitals, for the shelf signs: [pen, mug] -> [PEN, MUG]
pub fn sign_labels(names: &[String]) -> Vec<String> {
    names.iter().map(|n| n.to_uppercase()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn with_discount_lowers_every_price() {
        assert_eq!(with_discount(&[100, 500], 20), vec![80, 400]);
        assert_eq!(with_discount(&[250], 0), vec![250], "0% off changes nothing");
    }

    #[test]
    fn sign_labels_are_in_capitals() {
        let names = vec!["pen".to_string(), "mug".to_string()];
        assert_eq!(sign_labels(&names), vec!["PEN", "MUG"]);
        assert_eq!(names, vec!["pen", "mug"], "the original list is untouched");
    }
}
